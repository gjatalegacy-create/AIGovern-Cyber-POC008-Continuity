"""Verify every file listed by this tree's SHA-256 manifest."""
import hashlib
from pathlib import Path
root=Path(__file__).resolve().parents[1]
count=0
for line in (root/'artifact_hashes.sha256').read_text(encoding='utf-8').splitlines():
    expected,name=line.split('  ',1)
    path=(root/name).resolve()
    if not path.is_relative_to(root):
        raise SystemExit(f'FAIL: manifest path escapes root: {name}')
    if not path.is_file():
        raise SystemExit(f'FAIL: missing manifest file: {name}')
    actual=hashlib.sha256(path.read_bytes()).hexdigest()
    if actual!=expected:
        raise SystemExit(f'FAIL: SHA-256 mismatch: {name}; expected {expected}, actual {actual}')
    count+=1
print(f'PASS: {count} listed files match the release manifest.')
