# Known limitations

The [execution record](evidence/RESULT.json) distinguishes completed checks from untested properties. The 35 passing tests include observing a limitation; passing an adverse control does not mean that limitation was prevented.

1. **Finite format.** Checkpoints permit at most 4096 turns and 8 MiB, with additional string, collection and source-input bounds. A payload bound can be reached first. The process comparison runs through 65 turns; it does not establish the full 4096-turn horizon or indefinite continuity. Oversized input is tested to fail before append.

2. **Restricted contract.** PDC1's auxiliary field is `POC008:SCIENCE_NOT_EXECUTED`. It is not a model result, external evaluation or native knowledge verdict. Identity strings are scope data, not authenticated user identities.

3. **Partial integration.** Science-query evaluation, KiE, PIM, Spine, native T1/T2 and the full private runtime are excluded. Existing operational detector and response data do not constitute accepted scientific knowledge.

4. **New persistence code.** Store and the POC framing are new. Extracted record/projection/CRC components do not make this adapter the original KnowledgeVault. The two WAL formats are incompatible.

5. **Valid rollback is accepted.** A test replaces current history with a valid older prefix and successfully opens the old state. Replay establishes internal consistency, not that the file is the newest committed version. No protected external head or authenticated freshness mechanism exists.

6. **Hashes are not authentication.** CRC detects classes of accidental corruption. Unkeyed SHA-256 and FNV-1a do not establish sender identity, authorization or freshness. Replacing a coherent valid history remains outside the defended boundary.

7. **One implementation checks itself.** Validation restores the head and recomputes one input with the same engine. Shared implementation mistakes can survive execution and validation. The uninterrupted comparison tests reset consistency, not semantic truth or independent scientific agreement.

8. **Source pinning without migration.** Records hash the exact `continuity.rs` bytes. Even comments or formatting changes alter that binding. WAL compatibility across source revisions is not promised. Preserve matching released source and binaries; no migration or upgrade path is supplied.

9. **Full replay and growing history.** Each open reads and validates the complete WAL and recomputes its transitions. Every record retains a complete checkpoint. Per-frame bounds do not limit total history size, startup time or memory consumption. No latency, bandwidth or energy benchmark is claimed.

10. **Inspection may change a file.** `inspect` opens the ordinary Store. Missing/empty files may be initialized; valid incomplete tails may be truncated and synchronized. Detected complete corruption causes refusal without repair in tested cases. Inspect copies when preserving evidence.

11. **Locks coordinate cooperating owners.** The tested lock refuses a competing process and releases after owner termination. It depends on OS/filesystem behavior and is not process isolation, kernel protection or a defense against protocol bypass. Hostile path replacement is not tested.

12. **Durability coverage is limited.** Synchronization precedes in-memory publication, and a failed append marks the handle unusable for subsequent commits. Write/sync faults, storage-controller failures, power cuts and every crash instruction boundary are not injected. An unsuccessful write/sync may have an uncertain on-disk outcome; reopen and replay are required. Complete corruption, partial-header corruption and recoverable tails are distinct tested cases.

13. **Ticks are supplied inputs.** `TICK` enables reproducibility; it is not authenticated or guaranteed monotonic. Exact retries bind it. No physical timing claim follows.

14. **Platform and independent coverage.** The recorded local run uses Rust/Cargo 1.98.0, Windows GNU. Remote Windows/Linux CI and independent reproduction are separate activities, pending until linked. No separately tested package MSRV, dependency vulnerability audit or unbounded fuzzing result is claimed.

15. **No certification.** AI-assisted engineering and maintainer execution are not independent peer review, proof of the full theory, certification, AI Act compliance or a general cybersecurity guarantee.
