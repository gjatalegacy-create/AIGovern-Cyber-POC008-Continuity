# Claim boundary

## Bounded research question

For a finite supported sequence of operational PD inputs in one scope, does restoring the current checkpoint into a fresh PD engine for each turn produce exactly the same checkpoint bytes as uninterrupted execution? Can the storage adapter preserve retry, head-comparison and context checks across process restarts?

The comparison executes the extracted PD engine. The persistence adapter is new POC code. A fresh instance of the same implementation is a deterministic control, not an independent authority.

## Claims and observations

The local suite has 35 passing tests: 20 retained source unit tests and 15 integration tests, with no failed or ignored tests. [The execution record](evidence/RESULT.json) identifies final commands and artifact identity.

| Bounded claim | Observation | What it does not establish |
| --- | --- | --- |
| Checkpoint continuation survives tested process boundaries | Five-turn and 65-turn sequences compare complete checkpoint bytes after every turn against uninterrupted execution | Unlimited operation, semantic truth or physical power-loss survival |
| A new accepted request matches the current head plus one input | Restore, recompute and compare the complete request before append; changed question/checkpoint tests refuse writes | Independent scientific review, client authentication or general query evaluation |
| Exact retry precedes current-head comparison | Historical retry returns its record without append; changed retry and stale prepared requests fail | Every concurrent schedule or hostile-process interference |
| Replay validates transitions and parent chains | Complete corruption, rehashed invalid transitions and wrong-parent records are rejected; tested incomplete tails are repaired | Original KnowledgeVault compatibility or authenticated storage |
| Tested scopes remain separate | Two contexts retain their records while sharing one WAL | User authentication, tenant authorization or universal isolation |
| This path issues no native knowledge verdict | Candidate fields stay empty, the auxiliary marker is not evaluated, and summaries report no verdict or authority transfer | Acceptance of a scientific claim or the full native pipeline |
| Older valid history is not recognized as rollback | Restoring a valid old prefix returns the old state | Rollback prevention; this is an adverse control |

## Source and adapter distinction

Selected PD core/detector/checkpoint/data types, fixed-mass arithmetic, operational PDC1 contract, projection, FNV-1a, CRC and neutral 0-NOI come from AIGovern-Cyber. Provenance ledgers identify source ranges, hashes and transformations. Exact-copy claims apply only where byte identity is established; inherited source comments are not new experimental evidence.

Store, its framing, CLI, transition recomputation and experiment harness are new implementation. Store locks a file, synchronizes before publishing the projection, and stages replay before exposing the opened Store. This does not reproduce the complete KnowledgeVault or its authority integration.

LGC steps and condition-witness fields describe the adapter's operational sequence. A label such as `D-BAB` does not establish that the excluded native admission system ran. Neutral 0-NOI borrows a supplied payload; it neither selects an action nor grants authority.

## Explicit limits

- Format bounds are 4096 turns and an 8 MiB checkpoint; another limit may be reached earlier.
- No science-query evaluation, KiE/PIM/Spine integration, native T1/T2 execution or native knowledge verdict occurs.
- PDC1's auxiliary field is explicitly non-evaluated.
- Valid older history can be accepted; there is no authenticated freshness anchor.
- CRC and unkeyed hashes check consistency, not trusted authorship.
- Records pin exact adapter-source bytes; cross-revision migration is absent.
- Every open replays the full WAL. `inspect` uses the mutable recovery path and is not forensic read-only access.
- Process restarts, file-lock tests and synchronization do not prove arbitrary power-loss, write/sync-fault or hostile-host resilience.
- No result is inherited from private-runtime counts, earlier POCs, model agreement, hosting or DOI issuance.

Independent reproduction, general security, certification, novelty and AI Act compliance are not claimed.
