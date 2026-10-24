"""Verify every file listed by the frozen release SHA-256 manifest."""
import hashlib
from pathlib import Path
root=Path(__file__).resolve().parents[1]
count=0
for line in (root/'artifact_hashes.sha256').read_text(encoding='utf-8').splitlines():
    expected,name=line.split('  ',1)
    path=(root/name).resolve()
    assert path.is_relative_to(root),name
    assert hashlib.sha256(path.read_bytes()).hexdigest()==expected,name
    count+=1
print(f'PASS: {count} listed files match the release manifest.')
