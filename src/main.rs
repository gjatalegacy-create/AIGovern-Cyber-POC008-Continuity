use poc008_continuity::{input, summary, Store};
fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("lock-probe") if args.len() == 3 => {
            use std::io::Write;
            let _store = Store::open(&args[2])?;
            println!("LOCK_HELD");
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            let mut line = String::new();
            std::io::stdin()
                .read_line(&mut line)
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        Some("step") if args.len() == 7 => {
            let text = std::fs::read_to_string(&args[6]).map_err(|e| e.to_string())?;
            let timestamp = args[5].parse().map_err(|_| "POC_TICK_FORMAT")?;
            let mut store = Store::open(&args[2])?;
            let record = store.step(&input(&args[3], &args[4], &text), timestamp)?;
            let mut result = summary(&record)?;
            result["recovered_tail_bytes"] = store.recovered_tail_bytes().into();
            result["wal_bytes"] = store.bytes()?.into();
            println!(
                "{}",
                serde_json::to_string(&result).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Some("inspect") if args.len() == 4 => {
            let store = Store::open(&args[2])?;
            let identity = input(&args[3], "inspect-only", "not ingested").identity;
            let record = store.head(&identity)?.ok_or("POC_NO_SCOPED_HEAD")?;
            println!("{}", summary(&record)?);
            Ok(())
        }
        _ => Err(
            "usage: poc008-continuity step STORE CASE OP TICK INPUT.txt | inspect STORE CASE"
                .into(),
        ),
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(70);
    }
}
