# Voluntary reproduction and critical review

This capsule invites human technical review of a bounded continuity mechanism. No credential, affiliation, payment or favorable conclusion is required to use or critique the Apache-2.0 material.

Start with the [README commands](README.md), [execution record](evidence/RESULT.json), [claim boundary](CLAIM_BOUNDARY.md) and [known limitations](KNOWN_LIMITATIONS.md). Distinguish local execution evidence from remote CI or independent confirmation.

## Record enough to reproduce a finding

- Exact Git commit/release, archive or manifest hash, and local changes.
- OS, relevant filesystem, architecture, Rust version and dependency-lock identity.
- Complete commands, inputs, outputs, exit codes and skipped checks.
- Expected versus observed behavior and the smallest reproducer.
- The affected claim, adapter behavior or authority boundary.
- Whether you executed this Rust package, inspected source only or used a separate model.
- Relevant relationships, conflicts of interest and AI assistance. Affiliation is optional.

## Questions worth testing

1. Can a supported sequence diverge between uninterrupted execution and fresh-process continuation when comparing complete checkpoint bytes?
2. Does exact historical retry precede current-head checking? Can changed text, tick, scope or predecessor advance state incorrectly?
3. Does Store reject requests that differ from the current head plus one recomputed input? What assumptions does using the same engine require?
4. Does replay enforce the same transitions and parent chains? Distinguish complete corruption from incomplete trailing frames and report exact modified offsets.
5. Can one scope alter another's accepted state? Include identifiers and operation order without private data.
6. Does output imply native knowledge or science evaluation? The auxiliary marker is not evaluated; LGC labels do not establish execution of excluded native operators.
7. Does coherent older valid history remain accepted? This known limitation should remain visible, not be converted into a security guarantee.
8. What happens near input, checkpoint and turn limits? Separate resource refusal from semantic counterproof. No full 4096-turn experiment is claimed.
9. What happens after injected write/sync failures or other crash boundaries? Existing tests do not exhaust storage failures.
10. Is source-version mismatch refused clearly? Records pin adapter-source bytes and migration is absent.

## Classify evidence precisely

Use labels such as reproduced result, counterexample, implementation defect, unsupported claim, source observation or untested hypothesis. A simplified model is useful when differences are stated; it is not execution of the released Rust package.

File locks and process restarts do not prove power-loss or hostile-host resilience. A fresh instance of the same engine is not an independent authority. `inspect` may initialize or repair a Store; use copies when preserving evidence.

## Submit and respond

Use [GitHub Issues](https://github.com/gjatalegacy-create/AIGovern-Cyber-POC008-Continuity/issues) once published. The [Hugging Face mirror](https://huggingface.co/datasets/gjata-legacy/ess-mai-poc-008-continuity) is another distribution destination; its presence does not establish validation.

Use synthetic data. Do not submit credentials, personal data or production histories. Responses should retain adverse findings, link fixes to new revisions, and separate reproduction from interpretation. Reviewers' judgments must not be presented as endorsement of the full ESS-MAI theory or private runtime.
