"""Run five public dialogue inputs, each in a fresh process; never overwrite a run."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

TURNS = [
    "I am describing a flag, with a red background.",
    "The flag also has a black symbol; the colours alone are not enough.",
    "The black symbol is an eagle, with two heads.",
    "I mean the Albanian flag: shqiponjë dykrenare, sfond i kuq.",
    "Keep those details together; my description is still a user report, not independent verification.",
]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output-dir", required=True, type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    output = args.output_dir.resolve()
    output.mkdir(parents=True, exist_ok=False)
    commands, results = [], []
    for index, text in enumerate(TURNS):
        input_file = output / f"turn-{index + 1:02}.txt"
        input_file.write_bytes(text.encode("utf-8"))
        relative = ["step", "continuity.wal", "flag-description",
                    f"op-{index}", str(1000 + index), input_file.name]
        result = subprocess.run([str(binary), *relative], cwd=output,
                                capture_output=True, check=False)
        (output / f"step-{index+1:02}.stderr.txt").write_bytes(result.stderr)
        if result.returncode:
            raise RuntimeError(f"step {index+1}: exit {result.returncode}, {result.stderr!r}")
        row = json.loads(result.stdout)
        assert row["revision"] == row["total_turns"] == index + 1
        assert row["user_turns"] == TURNS[:index + 1]
        assert row["knowledge_verdict"] is None
        assert row["authority_transferred"] is False
        assert row["science_executed"] is False
        commands.append({"argv_after_binary": relative, "exit_code": result.returncode})
        results.append(row)
        print(f"cycle={index+1} revision={row['revision']} turns={row['total_turns']} "
              f"checkpoint_sha256={row['checkpoint_sha256']} verdict=null")
    (output / "cycles.jsonl").write_text(
        "".join(json.dumps(row, ensure_ascii=False, sort_keys=True)+"\n" for row in results),
        encoding="utf-8", newline="\n")
    (output / "commands.json").write_text(json.dumps({
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "binary_filename": binary.name, "commands": commands,
        "scope": "Operational persistence; exact-checkpoint equivalence is checked by Cargo integration tests."
    }, indent=2)+"\n", encoding="utf-8", newline="\n")
    print("PASS: five separate processes; exact UTF-8 turns retained; no knowledge verdict.")


if __name__ == "__main__":
    main()
