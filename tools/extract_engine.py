#!/usr/bin/env python3
"""Extract the public operational PD engine from a separately supplied runtime.

The private runtime is read only. No histories, WALs, fixtures, capabilities, or
native authority code are copied. Run --check without --source-root to verify
the checked-in output hashes; a checkout of the private runtime is not required.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "provenance_engine.json"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def extract(source_root: Path, rustfmt: str) -> dict:
    entries = []
    engine = "quantum/src/progressive_debatic/"
    specs = [
        (engine + "mod.rs", "src/progressive_debatic/mod.rs",
         [(22, 25), (28, 28)], [],
         ["Retain only data-only engine modules; omit native runtime, seal, and authority reexports."]),
        (engine + "core.rs", "src/progressive_debatic/core.rs",
         [(1, 471), (514, 596)], [],
         ["Omit three seal/runtime tests (source lines 472-513); retain every engine implementation line and five engine tests."]),
        (engine + "types.rs", "src/progressive_debatic/types.rs",
         [(1, 17), (28, 252), (473, 504), (506, 506), (511, 515),
          (517, 517), (522, 529), (538, 562)],
         [("crate::hw_real::ffi::fnv1a_64", "crate::fnv1a64")],
         ["Omit authority imports, seals, two-phase/native authority data types and unused authority/bridge error variants.",
          "Omit the seal test; retain the three data-only type tests.",
          "Redirect the FNV-1a call to the standalone crate's same-algorithm helper; do not alter ID mixing."]),
        (engine + "genius_detector.rs", "src/progressive_debatic/genius_detector.rs",
         None, [], ["Retain complete source, including all three original tests."]),
        (engine + "checkpoint.rs", "src/progressive_debatic/checkpoint.rs",
         None, [], ["Retain complete source, including all seven original checkpoint tests."]),
        ("shadow-contracts/src/boundary/pd_spine.rs", "src/pd_spine_contract.rs",
         [(9, 9), (18, 38)], [],
         ["Extract only MASS_SCALE and FixedMass; no Spine contract, capability, or authority types."]),
        ("zero-noi/src/lib.rs", "src/zero_noi.rs", None, [],
         ["Retain complete neutral borrowed-payload primitive and its two original tests."]),
    ]
    rustfmt_version = subprocess.check_output([rustfmt, "--version"], text=True).strip()
    with tempfile.TemporaryDirectory(prefix="poc008-engine-extract-") as temporary:
        for original, output, ranges, replacements, transformations in specs:
            raw = (source_root / original).read_bytes()
            lines = raw.decode("utf-8").splitlines(keepends=True)
            if ranges is None:
                ranges = [(1, len(lines))]
            for start, end in ranges:
                if not 1 <= start <= end <= len(lines):
                    raise ValueError(f"Invalid extraction range {original}:{start}-{end}")
            selected = "".join("".join(lines[start - 1:end]) for start, end in ranges)
            selected_hash = sha(selected.encode("utf-8"))
            selected = selected.replace("\r\n", "\n")
            for before, after in replacements:
                if selected.count(before) != 1:
                    raise ValueError(f"Expected one exact dependency replacement: {original}: {before}")
                selected = selected.replace(before, after)
            staged = Path(temporary) / output
            staged.parent.mkdir(parents=True, exist_ok=True)
            staged.write_text(selected, encoding="utf-8", newline="\n")
            # skip_children prevents mod.rs from reading or modifying sibling files.
            subprocess.run([rustfmt, "--edition", "2021", "--config",
                            "skip_children=true,newline_style=Unix", str(staged)], check=True)
            formatted = staged.read_bytes()
            entries.append({
                "source_path_relative_to_runtime": original,
                "source_sha256": sha(raw),
                "source_bytes": len(raw),
                "source_lines": len(lines),
                "selected_line_ranges_inclusive": [list(pair) for pair in ranges],
                "selected_source_utf8_sha256": selected_hash,
                "transformations": transformations + [
                    "Normalize line endings to LF and run rustfmt (Rust 2021 edition); no scoring, threshold, or checkpoint algorithm edits."],
                "output_path": output,
                "output_sha256": sha(formatted),
                "output_bytes": len(formatted),
                "output_lines": len(formatted.splitlines()),
            })
            target = ROOT / output
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(formatted)
    return {
        "schema": 1,
        "artifact": "POC008 operational continuity engine extraction",
        "source_product": "AIGovern-Cyber (ESS-MAI / Gjata Legacy lineage)",
        "source_root_logical": "codex_zvn/runtime",
        "source_root_handling": "Supplied explicitly by the operator; machine-specific absolute paths are not published.",
        "scope": "Deterministic PD state, canonical per-session checkpoint, data-only types, fixed-mass arithmetic, neutral 0-NOI.",
        "excluded": ["Native T1/T2 authority", "Spine/seal/capability types", "Private histories and evidence", "Private runtime persistence and process transport"],
        "rustfmt": rustfmt_version,
        "extractor": "tools/extract_engine.py",
        "extractor_sha256": sha(Path(__file__).read_bytes()),
        "files": entries,
    }


def check() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    mismatches = []
    for entry in manifest["files"]:
        path = ROOT / entry["output_path"]
        if not path.is_file() or sha(path.read_bytes()) != entry["output_sha256"]:
            mismatches.append(entry["output_path"])
    if sha(Path(__file__).read_bytes()) != manifest["extractor_sha256"]:
        mismatches.append("tools/extract_engine.py")
    if mismatches:
        raise SystemExit("Provenance hash mismatch: " + ", ".join(mismatches))
    print(f"PASS: {len(manifest['files'])} engine output hashes and extractor hash")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-root", type=Path, help="Read-only source runtime directory")
    parser.add_argument("--rustfmt", default=shutil.which("rustfmt"), help="Path to rustfmt")
    parser.add_argument("--check", action="store_true", help="Verify checked-in output hashes without private source")
    args = parser.parse_args()
    if args.check:
        if args.source_root:
            parser.error("--check does not read the private source; omit --source-root")
        check()
        return
    if not args.source_root or not args.rustfmt:
        parser.error("extraction requires --source-root and an available rustfmt")
    source_root = args.source_root.resolve(strict=True)
    if source_root == ROOT or ROOT in source_root.parents or source_root in ROOT.parents:
        parser.error("source and destination must be separate, non-nested trees")
    manifest = extract(source_root, args.rustfmt)
    MANIFEST.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8", newline="\n")
    check()


if __name__ == "__main__":
    main()
