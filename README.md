---
license: apache-2.0
language:
  - en
tags:
  - rust
  - reproducibility
  - checkpoint-continuity
pretty_name: AIGovern-Cyber POC008 Evidence-Bound Continuity
---
# AIGovern-Cyber POC008: Evidence-Bound Continuity

POC008 tests whether a finite ESS-MAI PD session can continue across separate operating-system processes with exactly the same checkpoint bytes as uninterrupted execution. It combines selected AIGovern-Cyber source with a new, small storage adapter. It is a public operational continuity experiment, not the complete private runtime.

Author and accountable maintainer: **Bledar Gjata, Independent Researcher, Albania**. [ORCID 0009-0003-7854-266X](https://orcid.org/0009-0003-7854-266X).

Distribution: [GitHub source and review](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC008-Continuity) and [Hugging Face research-artifact mirror](https://huggingface.co/datasets/gjata-legacy/ess-mai-poc-008-continuity). Software release DOI: [10.5281/zenodo.23613613](https://doi.org/10.5281/zenodo.23613613). [Research preprint](https://doi.org/10.5281/zenodo.23613785). The HF repository is an artifact distribution, not a training dataset.

## What is extracted, and what is new?

| Component | Provenance and role |
| --- | --- |
| PD core, detector, checkpoint logic and data types | Extracted executable engine. It processes actual text and retains turns, signals, concept state, pressure budget and detector state. No replacement counter or new scoring rule is used. |
| Restricted operational PDC1 contract | Extracted bounded codecs. The auxiliary science field carries the explicit `POC008:SCIENCE_NOT_EXECUTED` marker and is not evaluated. |
| PdProjection | Extracted scoped projection. Exact historical retries are resolved before checking the current head. |
| FixedMass, FNV-1a, frame CRC and neutral 0-NOI | Selected existing arithmetic, ID, corruption-checking and borrowed-payload implementations. None authenticates a producer or grants authority. |
| Store/WAL adapter and CLI | New POC code. Store locks one file, restores the current scoped head into fresh PD, recomputes one input, compares the request, appends and synchronizes, then publishes the in-memory projection. Replay repeats the checks. |

The new WAL has separate framing and is **not compatible with KnowledgeVault**. [Engine provenance](provenance_engine.json) and [contract provenance](provenance_contracts.json) record source ranges, transformations and hashes. Private histories and frozen runtime evidence are not bundled.

## Recorded local checks

**35 tests passed: 20 retained source unit tests and 15 integration tests; 0 failed, 0 ignored.** The final integration run took 135.34 seconds. The locked offline release build passed in 13.55 seconds. Both completed with zero warning lines on Rust/Cargo **1.98.0**, `x86_64-pc-windows-gnu`. These durations describe this run, not performance guarantees.

The [execution record](evidence/RESULT.json) identifies the tested package, commands and artifact hashes; logs are under [evidence/windows-gnu](evidence/windows-gnu/). The checks include:

- Five dialogue turns, each in a fresh process, with exact checkpoint-byte comparison after every turn.
- A 65-turn sequence across fresh processes, including the transition after 64 prior turns, with exact checkpoint-byte comparison at every step against the uninterrupted engine.
- Historical retry without append; changed text/timestamp and stale prepared-request rejection.
- Two contexts sharing one WAL without overwriting each other.
- Changed questions/checkpoints, rehashed invalid transitions and wrong-parent-chain rejection.
- Complete-record and detectable partial-header corruption refusal; recovery of incomplete trailing bytes while preserving the committed prefix.
- A competing process refused by the file lock, followed by release after the owner is killed.
- Oversized-input rejection before append.
- **Acceptance of a valid older WAL**, recorded as a rollback limitation.

Source tests cover import atomicity, identity and fresh-target checks, canonical ordering, semantic corruption, and retention of budget and detector state. These finite checks do not establish indefinite continuity, semantic truth or general security. [GitHub CI run 40079957984](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC008-Continuity/actions/runs/40079957984) also passed tests and release builds on Windows and Linux; see [the job record](evidence/github-ci.json). Independent human reproduction remains invited, not claimed.

## Build and reproduce

From the package root:

```sh
rustc --version --verbose
cargo test --locked
cargo build --release --locked
python tools/demo.py --binary target/release/poc008-continuity --output-dir demo-run
```

On Windows, use `target/release/poc008-continuity.exe` for `--binary`. `demo-run` must not exist. The demo creates five synthetic input fixtures, runs a new process for each turn, and saves JSONL output. Python is needed only for the demo and provenance utilities.

The standalone CLI also accepts:

```text
poc008-continuity step STORE CASE OP TICK INPUT.txt
poc008-continuity inspect STORE CASE
```

`INPUT.txt` supplies exact UTF-8 text. `TICK` is an explicit reproducibility input, not an authenticated clock. Repeating an operation requires identical text, scoped identity and tick; a new turn uses a new operation ID.

**`inspect` is not forensic read-only access.** It opens the ordinary Store: a missing/empty file may be initialized, and an incomplete tail may be repaired. Inspect a copy when preserving evidence. Every existing-WAL open replays the complete history.

Dependencies are pinned in [Cargo.toml](Cargo.toml) and [Cargo.lock](Cargo.lock); the first build may need downloads. Rust 1.98.0 is the measured toolchain. File-lock APIs became available in Rust 1.89, but no separately tested package MSRV is claimed.

## Boundaries

Checkpoint format limits are **4096 turns and 8 MiB**; another limit may be reached earlier. This experiment does not execute science queries, KiE, PIM, Spine, native T1/T2 or the complete authority pipeline. Its outputs contain no native knowledge verdict. Persisting a user report does not verify it.

Replay checks consistency, not authenticated freshness. A valid older file is accepted. File locks and synchronization depend on the OS/filesystem; tests do not simulate arbitrary power cuts or write/sync faults.

Records pin the exact adapter-source bytes. Even a comment-only edit to `continuity.rs` changes that binding: WAL replay across source revisions is not promised. No migration path is implemented.

See [Claim boundary](CLAIM_BOUNDARY.md), [Known limitations](KNOWN_LIMITATIONS.md), [Working method](WORKING_METHOD.md) and [Review protocol](REVIEW_PROTOCOL.md). The [Apache-2.0 license](LICENSE) and [attribution notice](NOTICE.md) apply. Citation and critical review are voluntary.

## Citation and lineage

[CITATION.cff](CITATION.cff) and [.zenodo.json](.zenodo.json) identify this software. Cite the actual release or commit used and the published POC008 software DOI.

Independent critique is invited through the [public PREreview request](https://prereview.org/preprints/doi-10.5281-zenodo.23613785). A published request is not a completed independent review.

Prior records: [ESS-MAI research origin](https://github.com/gjatalegacy-create/ESS-MAI), [POC003–006 archive](https://doi.org/10.5281/zenodo.22750188), [POC007 software](https://doi.org/10.5281/zenodo.23596907), and [POC007 preprint](https://doi.org/10.5281/zenodo.23597026). Their identifiers and results do not establish POC008 correctness.
