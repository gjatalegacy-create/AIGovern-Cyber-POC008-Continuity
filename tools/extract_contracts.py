"""One-time, local extraction. Private runtime is read-only; it is not bundled."""
import hashlib
import json
import argparse
import subprocess
from pathlib import Path

DEST = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--source-root", type=Path)
parser.add_argument("--check", action="store_true")
parser.add_argument("--rustfmt", default="rustfmt")
args = parser.parse_args()
if args.check:
    records = json.loads((DEST / "provenance_contracts.json").read_text(encoding="utf-8"))
    for record in records:
        assert hashlib.sha256((DEST / record["output"]).read_bytes()).hexdigest() == record["output_sha256"], record["output"]
    print(f"PASS: {len(records)} extracted contract files match recorded hashes")
    raise SystemExit(0)
if args.source_root is None:
    parser.error("--source-root is required for extraction; --check only needs the public package")
SOURCE = args.source_root
manifest = []


def extract(relative, output, ranges, replacements=()):
    original = (SOURCE / relative).read_bytes()
    lines = original.decode("utf-8-sig").replace("\r\n", "\n").splitlines(keepends=True)
    text = "".join("".join(lines[first - 1:last]) for first, last in ranges)
    for before, after in replacements:
        assert before in text, (relative, before)
        text = text.replace(before, after)
    path = DEST / output
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.replace("\r\n", "\n"), encoding="utf-8", newline="\n")
    subprocess.run([args.rustfmt, "--edition", "2021", str(path)], check=True)
    manifest.append({"source": relative, "source_sha256": hashlib.sha256(original).hexdigest(),
                     "inclusive_ranges": ranges, "output": output,
                     "transformations": [{"from": a, "to": b} for a, b in replacements],
                     "line_endings": "LF", "formatting": "rustfmt --edition 2021",
                     "output_sha256": hashlib.sha256(path.read_bytes()).hexdigest()})


extract("shadow-contracts/src/pd_memory.rs", "src/pd_memory.rs",
        [(1, 4), (14, 223), (240, 280), (290, 678), (963, 1004)])
extract("shadow/src/pd_memory_store.rs", "src/projection.rs",
        [(1, 52), (96, 108), (124, 170), (218, 271), (273, 289)],
        [("shadow_contracts::pd_memory", "crate::pd_memory"),
         ("//! NEW operational projection over the existing KnowledgeVault WAL.",
          "//! Source-derived scoped projection; POC008 supplies a separate, limited WAL adapter."),
         ("//! Only KnowledgeVault commits this projection after the same backend sync.",
          "//! The POC008 Store publishes this projection only after its durable append."),
         ("/// Caller holds the shared KnowledgeVault commit lock and this projection's\n    /// write lock until durable append and publication finish.",
          "/// Caller holds the exclusive Store file lock and mutable projection through durable append.")])
extract("shadow-contracts/src/lib.rs", "src/fnv.rs", [(1409, 1416)])
extract("shadow/src/vault_disk.rs", "src/framing.rs", [(107, 119), (126, 128)],
        [("fn frame_crc", "pub(crate) fn frame_crc")])
(DEST / "provenance_contracts.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
