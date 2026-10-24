use poc008_continuity::pd_memory::{encode_record, hex, PdOperationRecord};
use poc008_continuity::{input, summary, uninterrupted, Store};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static SERIAL: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "poc008-test-{}-{stamp}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn wal(&self) -> PathBuf {
        self.0.join("continuity.wal")
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn turns() -> Vec<(String, u64)> {
    ["I am describing a flag, with a red background.",
     "The flag also has a black symbol; the colours alone are not enough.",
     "The black symbol is an eagle, with two heads.",
     "I mean the Albanian flag: shqiponjë dykrenare, sfond i kuq.",
     "Keep those details together; my description is still a user report, not independent verification."]
        .iter().enumerate().map(|(i,s)|(s.to_string(),1000+i as u64)).collect()
}
fn run_step(temp: &Temp, index: usize, text: &str, timestamp: u64) -> serde_json::Value {
    let file = temp.0.join(format!("turn-{index}.txt"));
    fs::write(&file, text).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_poc008-continuity"))
        .arg("step")
        .arg(temp.wal())
        .arg("flag-description")
        .arg(format!("op-{index}"))
        .arg(timestamp.to_string())
        .arg(file)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}

#[test]
fn five_dialogue_cycles_in_new_processes_equal_uninterrupted_engine() {
    let temp = Temp::new();
    let turns = turns();
    for (index, (text, timestamp)) in turns.iter().enumerate() {
        let result = run_step(&temp, index, text, *timestamp);
        assert_eq!(result["total_turns"], index + 1);
        assert_eq!(result["revision"], index + 1);
        assert_eq!(result["user_turns"][index], *text);
        assert!(result["knowledge_verdict"].is_null());
        assert_eq!(result["authority_transferred"], false);
        let store = Store::open(temp.wal()).unwrap();
        let id = input("flag-description", "unused", "unused").identity;
        let actual = store.head(&id).unwrap().unwrap().request.checkpoint;
        assert_eq!(actual, uninterrupted(&id, &turns[..=index]).unwrap());
    }
}

#[test]
fn sixty_four_restarts_preserve_state_and_the_sixty_fifth_transition() {
    let temp = Temp::new();
    let mut conversation = Vec::new();
    for i in 0..65 {
        let text=format!("Flag description step {i}: red background, black eagle; preserve the source and distinction.");
        conversation.push((text.clone(), 2000 + i as u64));
        let result = run_step(&temp, i, &text, 2000 + i as u64);
        let id = input("flag-description", "unused", "unused").identity;
        let expected = uninterrupted(&id, &conversation).unwrap();
        let hash: [u8; 32] = Sha256::digest(&expected).into();
        assert_eq!(result["checkpoint_sha256"], hex(&hash));
        let store = Store::open(temp.wal()).unwrap();
        let actual = store.head(&id).unwrap().unwrap().request.checkpoint;
        assert_eq!(actual, expected);
    }
}

#[test]
fn old_exact_retry_after_head_advanced_does_not_append_or_reset_head() {
    let temp = Temp::new();
    let mut store = Store::open(temp.wal()).unwrap();
    let first = input("flag", "op1", "Red flag, black eagle.");
    let original = store.step(&first, 100).unwrap();
    let later = store
        .step(&input("flag", "op2", "The eagle has two heads."), 101)
        .unwrap();
    let length = store.bytes().unwrap();
    assert_eq!(store.step(&first, 100).unwrap(), original);
    assert_eq!(store.bytes().unwrap(), length);
    assert_eq!(store.head(&first.identity).unwrap().unwrap(), later);
    drop(store);
    let mut reopened = Store::open(temp.wal()).unwrap();
    assert_eq!(reopened.step(&first, 100).unwrap(), original);
    assert_eq!(reopened.head(&first.identity).unwrap().unwrap(), later);
}

#[test]
fn changed_retry_and_stale_new_request_fail_without_writing() {
    let temp = Temp::new();
    let mut store = Store::open(temp.wal()).unwrap();
    let first = input("flag", "one", "Red flag.");
    store.step(&first, 100).unwrap();
    let stale = store
        .prepare(&input("flag", "stale", "Black eagle."), 101)
        .unwrap();
    store
        .step(&input("flag", "two", "Two heads."), 101)
        .unwrap();
    drop(store);
    let before = fs::read(temp.wal()).unwrap();
    let mut store = Store::open(temp.wal()).unwrap();
    assert!(store.commit(stale).unwrap_err().contains("CAS_CONFLICT"));
    assert!(store
        .step(&input("flag", "one", "Changed text."), 100)
        .unwrap_err()
        .contains("CONFLICT"));
    assert!(store.step(&first, 999).unwrap_err().contains("CONFLICT"));
    drop(store);
    assert_eq!(fs::read(temp.wal()).unwrap(), before);
}

#[test]
fn two_contexts_do_not_overwrite_each_others_state() {
    let temp = Temp::new();
    let mut store = Store::open(temp.wal()).unwrap();
    let a = input("flag", "op1", "Red background.");
    let b = input("turbine", "op1", "Wind turbine geometry.");
    store.step(&a, 1).unwrap();
    let original_b = store.step(&b, 1).unwrap();
    store
        .step(&input("flag", "op2", "Black eagle."), 2)
        .unwrap();
    drop(store);
    let store = Store::open(temp.wal()).unwrap();
    assert_eq!(store.head(&a.identity).unwrap().unwrap().revision, 2);
    assert_eq!(store.head(&b.identity).unwrap().unwrap(), original_b);
}

#[test]
fn caller_changed_question_and_checkpoint_are_recomputed_before_commit() {
    let temp = Temp::new();
    let store = Store::open(temp.wal()).unwrap();
    let request = store
        .prepare(&input("flag", "one", "Red, black; shqiponjë."), 1)
        .unwrap();
    drop(store);
    let before = fs::read(temp.wal()).unwrap();
    let mut store = Store::open(temp.wal()).unwrap();
    let mut changed = request.clone();
    changed.scoped_question = "Invented accepted knowledge".into();
    assert!(store
        .commit(changed)
        .unwrap_err()
        .contains("RECOMPUTED_TRANSITION"));
    let mut changed = request;
    let n = changed.checkpoint.len();
    changed.checkpoint[n - 1] ^= 1;
    assert!(store
        .commit(changed)
        .unwrap_err()
        .contains("RECOMPUTED_TRANSITION"));
    drop(store);
    assert_eq!(fs::read(temp.wal()).unwrap(), before);
}

fn crc(parts: &[&[u8]]) -> u32 {
    let mut value = 0xffffffffu32;
    for byte in parts.iter().flat_map(|p| p.iter()) {
        value ^= u32::from(*byte);
        for _ in 0..8 {
            value = (value >> 1) ^ (0xedb88320 & (value & 1).wrapping_neg());
        }
    }
    !value
}
fn frame(record: &PdOperationRecord) -> Vec<u8> {
    let data = encode_record(record).unwrap();
    let n = data.len() as u32;
    let mut bytes = Vec::new();
    bytes.extend(n.to_le_bytes());
    bytes.extend((!n).to_le_bytes());
    bytes.extend(crc(&[&n.to_le_bytes(), &data]).to_le_bytes());
    bytes.extend(data);
    bytes
}
fn one_committed(temp: &Temp) -> PdOperationRecord {
    let mut store = Store::open(temp.wal()).unwrap();
    store
        .step(&input("flag", "one", "Red flag, exact punctuation!"), 1)
        .unwrap()
}

#[test]
fn corrupted_complete_frame_is_refused_without_repair() {
    let temp = Temp::new();
    one_committed(&temp);
    let mut bytes = fs::read(temp.wal()).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    fs::write(temp.wal(), &bytes).unwrap();
    assert!(Store::open(temp.wal())
        .err()
        .unwrap()
        .contains("CORRUPT_FRAME_CRC"));
    assert_eq!(fs::read(temp.wal()).unwrap(), bytes);
}

#[test]
fn rehashed_invalid_transition_is_refused_without_repair() {
    let temp = Temp::new();
    let record = one_committed(&temp);
    let mut request = record.request;
    request.scoped_question = "Invented acceptance after rewriting all public hashes".into();
    let forged = PdOperationRecord::construct(request, [0; 32]).unwrap();
    let bytes = [b"OPL2SPD1".as_slice(), frame(&forged).as_slice()].concat();
    fs::write(temp.wal(), &bytes).unwrap();
    assert!(Store::open(temp.wal())
        .err()
        .unwrap()
        .contains("RECOMPUTED_TRANSITION"));
    assert_eq!(fs::read(temp.wal()).unwrap(), bytes);
}

#[test]
fn incomplete_tail_recovers_exact_committed_prefix() {
    for suffix in [vec![1, 2, 3], {
        let mut h = 100u32.to_le_bytes().to_vec();
        h.extend((!100u32).to_le_bytes());
        h.extend(0u32.to_le_bytes());
        h.extend([4, 5]);
        h
    }] {
        let temp = Temp::new();
        let record = one_committed(&temp);
        let before = fs::read(temp.wal()).unwrap();
        fs::OpenOptions::new()
            .append(true)
            .open(temp.wal())
            .unwrap()
            .write_all(&suffix)
            .unwrap();
        let store = Store::open(temp.wal()).unwrap();
        assert_eq!(store.recovered_tail_bytes(), suffix.len() as u64);
        assert_eq!(
            store.head(&record.request.identity).unwrap().unwrap(),
            record
        );
        drop(store);
        assert_eq!(fs::read(temp.wal()).unwrap(), before);
    }
}

#[test]
fn malformed_complete_header_is_not_mislabelled_as_incomplete_tail() {
    let temp = Temp::new();
    one_committed(&temp);
    let mut before = fs::read(temp.wal()).unwrap();
    before.extend([0u8; 12]);
    fs::write(temp.wal(), &before).unwrap();
    assert!(Store::open(temp.wal())
        .err()
        .unwrap()
        .contains("CORRUPT_FRAME_HEADER"));
    assert_eq!(fs::read(temp.wal()).unwrap(), before);
}

#[test]
fn detectably_corrupt_partial_header_is_refused_without_truncation() {
    for suffix in [
        vec![0u8; 4],
        vec![255u8; 4],
        vec![100, 0, 0, 0, 0],
        vec![100, 0, 0, 0, 155, 255, 0],
    ] {
        let temp = Temp::new();
        one_committed(&temp);
        let mut before = fs::read(temp.wal()).unwrap();
        before.extend(suffix);
        fs::write(temp.wal(), &before).unwrap();
        let error = Store::open(temp.wal())
            .err()
            .expect("detectable partial-header corruption must fail");
        assert!(error.contains("CORRUPT_FRAME_HEADER"), "{error}");
        assert_eq!(fs::read(temp.wal()).unwrap(), before);
    }
}

#[test]
fn operating_system_lock_refuses_other_process_then_releases_after_kill() {
    let temp = Temp::new();
    one_committed(&temp);
    let mut child = Command::new(env!("CARGO_BIN_EXE_poc008-continuity"))
        .arg("lock-probe")
        .arg(temp.wal())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut ready = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut ready)
        .unwrap();
    assert_eq!(ready.trim(), "LOCK_HELD");
    assert!(Store::open(temp.wal())
        .err()
        .unwrap()
        .contains("STORE_LOCK"));
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(Store::open(temp.wal()).is_ok());
}

#[test]
fn valid_old_prefix_is_accepted_documenting_rollback_limit() {
    let temp = Temp::new();
    let first = one_committed(&temp);
    let old = fs::read(temp.wal()).unwrap();
    {
        let mut store = Store::open(temp.wal()).unwrap();
        store
            .step(&input("flag", "two", "Two-headed eagle."), 2)
            .unwrap();
    }
    fs::write(temp.wal(), &old).unwrap();
    let store = Store::open(temp.wal()).unwrap();
    assert_eq!(store.head(&first.request.identity).unwrap().unwrap(), first);
    assert_eq!(summary(&first).unwrap()["revision"], 1);
}

#[test]
fn rehashed_wrong_parent_record_cannot_extend_the_chain() {
    let temp = Temp::new();
    let first = one_committed(&temp);
    let store = Store::open(temp.wal()).unwrap();
    let next = store
        .prepare(&input("flag", "two", "Black eagle, two heads."), 2)
        .unwrap();
    drop(store);
    let wrong = PdOperationRecord::construct(next, [55; 32]).unwrap();
    let mut bytes = fs::read(temp.wal()).unwrap();
    bytes.extend(frame(&wrong));
    fs::write(temp.wal(), &bytes).unwrap();
    assert!(Store::open(temp.wal())
        .err()
        .unwrap()
        .contains("REPLAY_CHAIN_CONFLICT"));
    assert_eq!(fs::read(temp.wal()).unwrap(), bytes);
    assert_eq!(first.revision, 1);
}

#[test]
fn oversized_input_fails_before_any_append() {
    let temp = Temp::new();
    one_committed(&temp);
    let before = fs::read(temp.wal()).unwrap();
    let mut store = Store::open(temp.wal()).unwrap();
    assert!(store
        .step(&input("flag", "oversized", &"a".repeat(8193)), 2)
        .is_err());
    drop(store);
    assert_eq!(fs::read(temp.wal()).unwrap(), before);
}
