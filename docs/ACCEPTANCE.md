# Emenda V0.2 Acceptance

> **Frozen acceptance contract, version 2.3.0**

## 1. Role and evidence standard

This document derives verifiable gates from [`SPEC.md`](../SPEC.md), [`ARCHITECTURE.md`](ARCHITECTURE.md), and [`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md). SPEC supplies product behavior, Architecture supplies ownership boundaries, and the Implementation Plan supplies build order.

A gate passes only through reproducible evidence recorded at the tested state. Use these evidence levels precisely:

```text
inspected
compiled
deterministic
integration
live
runtime
```

Each entry records environment and tool versions, the relevant command or manual procedure, exact sanitized outcome, limitations, applicable critical requirement IDs, and:

```text
constitution freeze ID:
constitution commit:
constitution tree:
tested implementation tree:
tested implementation commit:
```

An evidence commit records an already-existing implementation commit that was actually tested. Record each failure and later recovery as separate entries. Evidence admits exactly the sanitized fields defined here; the active credential, provider, and browser-authorization boundaries retain their private runtime values.

A deterministic gate passes when every required deterministic assertion passes. The Provider Gate's live qualification has the separate factual standard in Section 6.3.

## 2. Gates and current stop boundary

The six gates are:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.2 Conformance
```

Sprint 2 ends after the exact 13-file candidate passes Documentation, receives independent exact-tree review, becomes one atomic single-parent v2.3.0 freeze, and reaches the refs and clean state in the manifest. Sprint 3 → Sprint 4 → Sprint 5 implementation requires a separately authorized objective; documentation completion supplies no successor runtime evidence.

## 3. Documentation Gate

The gate passes when:

- the candidate starts from sole parent `958d96b22c94f5bc2cee5b13bd49356bd86c9f2a`, tree `c41fb653dfe98a67935e7606ec2429ca6aebbcb0`; prior freezes, especially `docs/v2.2.1-freeze` at `cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d`, and the two-parent v2.1.0 convergence remain preserved;
- the tracked inventory is exactly the manifest's 13 Markdown paths, with exactly 12 immutable files changed and `docs/EVIDENCE.md` identical to the published parent;
- the 12 immutable files identify version 2.3.0 and product V0.2 where applicable, and the current freeze ID is `emenda-clean-room-v2.3.0-2026-10-07`; explicitly historical identities and the entire historical ledger are exempt from blanket replacement;
- behavior, ownership, sequence, acceptance, engineering, UX, brand, authorization and integrity resolve to their singular authority homes, and every local Markdown link resolves;
- canonical sequence occurrences agree with Implementation Plan Section 2, and six gates retain their order with only the final name changed to V0.2 Conformance;
- SPEC has 23 unique active critical IDs and Section 3.1 maps each exactly once, retaining the 18 existing high-risk invariants; DU-01 through DU-21 have the canonical coverage in Section 4.5;
- comparison with the predecessor proves the unchanged 15-case corpus, canonical prompt bytes, model-facing schema/serialization, profiles, scalar context/focus policy, provider generation fields, strict validation, correction derivation, textarea exclusions, safe Apply/Dismiss/native Undo, credentials/exact-origin policy, no retry/fallback, toolchain/dependencies and visual assets/tokens;
- the only semantic delta is explicit proofreading, provider-free local settling, verified context-menu intent, protocol 2, the added `contextMenus` permission, worker lease/fencing/coalescing/cache/recovery, text-free restart markers, bounded resources and qualification inheritance, with derived disclosure, acceptance and continuation;
- all 11 independent checksums are generated from the final staged raw blobs and match the manifest; manifest self-reference and mutable ledger remain excluded;
- ledger failures, qualified environment/artifact/limitations, the one 15/15 run and two complete independent semantic reviews remain byte-for-byte historical, with no successor implementation claim or empty-ledger requirement;
- `git diff --cached --check` passes; scratch verification introduces no tracked executable, ADR, lock or extra document;
- one independent read-only consistency, security, architecture, acceptance and implementability review binds to the final candidate tree; verified blockers are resolved and substantive changes receive renewed review;
- one atomic freeze commit has the reviewed tree and sole parent, reaches `origin/docs/v2.3.0-freeze` and `origin/main`, and has verified matching remote identities, preserved older refs/ancestry and a clean tracked worktree with relevant ignored state preserved.

Sprint 2 runs no inference and changes no implementation or installed state. The predecessor implementation audit is not a successor Documentation Gate; Sprint 3 updates that audit after immutable intake.

### 3.1 Critical-requirement traceability

[`SPEC.md`](../SPEC.md#critical-requirement-identifiers) owns these requirements. This table derives their minimum acceptance coverage. An ID is covered only when every listed acceptance section passes at its owning evidence level and each corresponding evidence entry cites that ID.

| Requirement | Required acceptance coverage |
| --- | --- |
| `EM-AUTH-001` | Sections 4.1, 4.4, and 7.3: revision races, stale silence, and current capability authority |
| `EM-AUTH-002` | Sections 6.1 and 7.2: strict one-shot protocol and sender-class authorization |
| `EM-AUTH-003` | Sections 4.1, 6.1, and 7.1: settings revision, invalidation, and stale resynchronization |
| `EM-PERM-001` | Sections 6.1, 7.2, and 8: exact explicit-port origin derivation and runtime round trips |
| `EM-PERM-002` | Sections 6.1, 7.2, 7.3, and 8: current Check and Apply authorization |
| `EM-PERM-003` | Sections 7.2 and 8: serialized reconciliation, revocation, restart, BFCache, and prerender behavior |
| `EM-PRIV-001` | Sections 6.2 and 7.6: bounded provider text and metadata confinement |
| `EM-PRIV-002` | Sections 6.1 and 7.1: worker-only credentials and trusted-storage isolation |
| `EM-PRIV-003` | Sections 6.2 and 7.6: redaction and absence of text history, telemetry, and identifying metadata |
| `EM-PROV-001` | Sections 6.2 and 6.3: canonical request enforcement and complete live corpus |
| `EM-PROV-002` | Sections 4.3 and 6.1–6.3: strict result validation, exact model identity, and local derivation |
| `EM-PROV-003` | Sections 4.3, 6.3, and 7.5: probabilistic proposal, deterministic structure, independent agent qualification review and writer semantic approval |
| `EM-APPLY-001` | Sections 4.3, 7.3, and 8: controller, worker, and surface authority chain |
| `EM-APPLY-002` | Sections 4.4, 7.3, and 8: sole mutation, exact acknowledgement, refusal, and one-step Undo |
| `EM-APPLY-003` | Section 7.5: trusted current approval controls, focus handoff, and hit tests |
| `EM-SEC-001` | Sections 6.2 and 7.5: instruction isolation and literal text-only rendering |
| `EM-SEC-002` | Section 7.4: classification before read and fail-closed supported-surface boundary |
| `EM-SEC-003` | Sections 6.2–6.3, 7.1, 7.3–7.5, and 8: provider, credential, residual-risk disclosure, and environment-specific evidence |
| `EM-RES-001` | Sections 4.1, 4.4, 6.2, 7.1–7.3, and 8: zero ambient provider traffic and per-controller settling |
| `EM-OPS-001` | Sections 4.4, 6.1–6.2, and 7.1: global exclusive bounded client dispatch and zero backlog |
| `EM-OPS-002` | Sections 4.1, 4.4, 6.1–6.2, and 7.1: exact active coalescing, bounded cache and fresh reuse authority |
| `EM-OPS-003` | Sections 4.4, 6.1–6.2, and 7.1–7.2: provider uncertainty, one-shot recovery, deadlines, restart and permanent fencing |
| `EM-QUAL-001` | Sections 4.2–4.3 and 6.2–6.3: deterministic model-facing equivalence before inherited qualification |

## 4. Mock Product Gate

### 4.1 Unified state machine and authority

Fake-clock and reducer tests prove:

- one pure reducer owns `Disabled | Idle | Settling | Checking | Suggestion | Applying | Error` and effects own timers, inference, messaging, storage interaction, and surface operations;
- each eligible committed input reserves a new revision synchronously, clears a current error or suggestion, cancels older work best-effort, and starts one trailing 600 ms settling timer;
- ordinary input is eligible only from a same-source, same-generation trusted `beforeinput`/`input` ticket that binds exact pre/post tuples and passes the complete foreground/exposed textarea predicate at both events; every later `beforeinput` first invalidates the prior ticket, the first input clears it, and each private expiry callback clears only its own still-current opaque ticket while listener microtasks do not, and an unpaired or untrusted change on that otherwise supported textarea may refresh the baseline and invalidate stale authority without reserving a revision or requesting inference, while the exact registered Apply input is the sole bypass;
- after twenty trusted edits, zero inference/catalog/readiness/status requests occur; at 599 ms after the final edit the controller remains Settling, at 600 ms local bookkeeping finishes in Idle with zero inference capture or dispatch; later edits replace only that controller’s one timer; two controllers settle independently; Disabled performs no observation capture or provider work;
- revisions grant no dispatch authority; only a verified explicit intent can capture bounded current input and request deep work, without prior typing; current source/revision/configuration and exact operation token win every completion/cancellation race;
- stale results, stale failures, stale settings revisions, and stale Apply or Dismiss commands do not change presentation or text;
- Provider, either API-key, either model, and profile changes increment `settingsRevision`, cancel active inference, invalidate visible suggestions and obsolete errors, and return to `Idle`; subsequent typing remains provider-free and only fresh explicit intent dispatches, and origin changes do not increment it;
- the simulated public configuration contains only `isConfigured` and `settingsRevision` and is replaced in every live enabled controller by validated update events rather than fetched before capture;
- every composition start invalidates immediately and binds its collapsed pre-composition tuple; only a trusted start on a qualifying surface creates an eligible generation, every composing change must use a trusted same-generation pair and refresh the exact text/selection baseline, an in-bounds losslessly mapped intermediate IME candidate range may be noncollapsed, delayed selection notifications are ignored only when they match that baseline, any untrusted, unpaired, malformed, or mismatching composing event disqualifies the generation, and only a trusted qualifying end at a collapsed caret whose terminal text changed emits the sole committed change; cancelled and no-op generations remain silent;
- a terminal pair after composition end is deduplicated only when text, source, and selection all match within that composition generation; each individual mismatch is external and reserves a revision only when it independently qualifies;
- a moved caret, changed selection, hidden-document transition, or window blur clears input/composition provenance, invalidates current authority and presentation, and causes no automatic retry when foreground focus returns;
- one direct textarea-to-current-approval-UI handoff and focus movement among its current controls preserve the captured selection, while focus leaving both invalidates it; the scoped internal correction-range selection is verified synchronously, and delayed or coalesced notifications preserve authority only when source, current value and selection, generation, and revision identity if one exists equal the latest applicable accepted ordinary, composition, or Apply baseline.

### 4.2 Deterministic text policy

Pure tests cover ASCII, Georgian, Russian, combining sequences, emoji, and supplementary-plane scalars and prove:

- every range is half-open and measured in Unicode scalars rather than UTF-16 units, bytes, or graphemes;
- outside an eligible baseline-only intermediate IME pair, a non-collapsed selection fails silently; a collapsed caret selects deterministically;
- paragraphs are maximal scalar ranges between explicit LF boundaries;
- `.`, `!`, and `?` begin a terminator sequence; trailing `Pe`/`Pf` punctuation and U+0022/U+0027 quotation marks remain in it, and the boundary is tested only after that complete sequence for Unicode `White_Space`, LF, or end;
- sentence ranges partition the paragraph: leading whitespace belongs to the first following sentence, terminator-following and final trailing whitespace belongs to the preceding sentence, a boundary offset before the next non-whitespace scalar selects that next sentence, and paragraph end selects the final sentence;
- a caret immediately before LF selects the paragraph to its left, immediately after LF selects the following paragraph, and between consecutive LFs selects the empty paragraph; leading/trailing whitespace, terminator/closer edges, document start/end, and `One. Two.` boundary offsets have exact fixtures;
- a focus without a Unicode Letter scalar (`\p{L}`) produces no request;
- context and focus obey the canonical scalar limits in [`SPEC.md`](../SPEC.md#3-v02-runtime-and-limits), with the complete focus present, deterministic surrounding-context allocation, and silent refusal above the focus limit;
- complete paragraphs, truncation, even division, odd trailing allocation, boundary clamping, and backfill behave exactly as specified;
- browser and provider boundaries reject lone surrogates and raw CR, while model-authored corrected focus is compared as Unicode scalars without normalization or relocation and maps from focus-relative to context-relative to snapshot-relative coordinates exactly once.

### 4.3 Validation, failures, and presentation

Semantic validation and mock-provider cases prove:

- clean, empty, nonlinguistic, unsupported-language, over-limit-focus, ordinary non-collapsed-selection, and ordinary unsupported-capture outcomes return silently to `Idle`;
- supported `auto` results are accepted, fixed mode accepts only its exact profile or `unsupported`, `unsupported` is accepted only with an empty correction list and returns to `Idle`, and a different supported profile is invalid provider output with no suggestion and `Error`;
- the external result accepts only the strict shape in [`SPEC.md`](../SPEC.md#8-model-facing-contract-and-local-derivation), while the worker-to-content result contains only the trusted derived correction and never model-authored `correctedFocus`;
- minimum Unicode-scalar edit distance produces the specified deterministic result for insertion, deletion, substitution, adjacent edits, repeated-character ties, combining sequences, emoji, and supplementary-plane scalars;
- unchanged `correctedFocus`, separated edit hunks, excess corrected-focus or explanation length, whitespace-only explanation, malformed Unicode or CR, malformed language combinations, and non-reconstructing or unmappable derivations are rejected;
- one accepted hunk derives the exact half-open range, `original`, and `replacement`, remains inside focus, and reconstructs `correctedFocus` exactly;
- a whole-focus translation-shaped replacement can satisfy the structural one-hunk rule, but the system never represents that fact as proof of semantic preservation;
- missing configuration enters `Error` with Open Settings;
- current timeout, provider failure, invalid response, and Apply refusal enter `Error`;
- stale completion or cancellation causes no presentation change;
- one current valid correction creates one `SuggestionId` capability; suggestion Dismiss accepts only that capability and mutates nothing;
- one current content error creates a separate `ErrorId`; error Dismiss accepts only that capability, clears no suggestion, and mutates no page text;
- Apply reaches the surface only after the controller verifies the current suggestion, current revision, and their association.

### 4.4 Complete simulated product

The deterministic composition proves trusted input → immediate revision invalidation → local settling → Idle with no provider work, followed separately by verified explicit intent → cached public settings → capture/context → worker admission/recovery/reuse or dispatch → validation → suggestion → Apply or Dismiss → Idle. Mocks cover trusted paired keyboard/paste input and IME, a ticket surviving a `beforeinput`-queued microtask, untrusted, unpaired, post-expiry-callback, wrong-source, and wrong-generation input, clean and correction results, delayed stale completion, timeout, cancellation race, source and snapshot changes, changed text or selection, lost window or element focus, hidden or midpoint-covered document state, readonly state, mapping refusal, off-caret insertion/deletion/replacement, exact replacement, and self-authored replacement acknowledgement. Every provider success or failure rechecks the captured source, document, snapshot value/selection, foreground focus, and exposure before presentation; a completion-time programmatic change without an input event is silent and refreshes only the supported local baseline.

An exact expected self-mutation updates the post-edit baseline, emits no new observed change, advances authority without inference, and returns to `Idle`. A mismatch is external, refreshes the supported baseline, invalidates old Apply authority, and reserves a revision only when it independently has eligible paired provenance.

Controlled races prove R1 is permanently retired by editing/invalidation, R2 starts only after permitted lease release/recovery, and late R1 success/failure cannot clear R2's marker, own its lease, enter cache, alter readiness/presentation or mutate text. Repeat with worker restart, settings/provider switches, revocation, navigation and opaque source replacement. Fake clocks prove remote exclusion until the original deadline, no renewal by coalescing, release after that bound and rejection of the old response forever; local uncertainty remains Recovering until exactly one successful idle status inspection during a fresh explicit action.

Cache fixtures prove four entries versus five, 65,536 versus 65,537 serialized UTF-8 bytes (including retained identity fields), 59,999 versus 60,000 ms since completion, completion-order eviction and non-sliding reuse. Oversized entries are not retained. Corrections and no-correction results can reuse; failures, raw bodies and diagnostic readiness cannot. One one-shot next-expiry timer is rescheduled only as entries change, never periodic. Navigation/revocation/source/configuration changes immediately make affected entries ineligible, and lookup reauthorizes even before notification delivery. A reused correction obtains a fresh suggestion capability; old capabilities stay invalid. Worker termination clears everything.

### 4.5 Daily-use acceptance migration

These stable labels derive the approved Sprint 1 criteria. Every listed section is required at its own evidence level; the table adds no alternate behavioral authority.

| Criterion | Required observable result and canonical coverage |
| --- | --- |
| DU-01 | Twenty edits, zero oMLX requests: Sections 4.1/4.4, 7.3, 8 |
| DU-02 | Local settling exactly 600 ms after final edit, including 599/600 boundaries: Section 4.1 |
| DU-03 | Idle produces no provider traffic: Sections 4.1/4.4, 6.2, 8 |
| DU-04 | Focus changes produce no provider traffic: Sections 4.1/4.4, 7.2, 8 |
| DU-05 | Navigation produces no provider traffic: Sections 4.1/4.4, 7.2, 8 |
| DU-06 | Startup produces no provider traffic: Sections 6.2, 7.1–7.2, 8 |
| DU-07 | Worker wakeup produces no provider traffic: Sections 6.2, 7.1–7.2, 8 |
| DU-08 | Native German/English spelling works with oMLX stopped: Section 8 runtime |
| DU-09 | Explicit current bounded capture, at most one deep POST: Sections 4.4, 6.1–6.2, 7.3 |
| DU-10 | Exact identical actions coalesce or reuse: Sections 4.4, 6.1–6.2 |
| DU-11 | Incompatible content/configuration invalidates reuse: Sections 4.1/4.4, 6.1, 7.1 |
| DU-12 | Cross-tab and diagnostic concurrency is exclusive with zero backlog: Sections 6.1–6.2, 7.1 |
| DU-13 | Edits retire result/Apply authority immediately: Sections 4.1/4.4, 7.3 |
| DU-14 | Interrupted/uncertain execution is retained and fenced: Sections 6.1–6.2, 7.1–7.2 |
| DU-15 | Exactly one local status inspection on next explicit action: Sections 6.1–6.2, 7.1–7.2 |
| DU-16 | Busy/loading/malformed/unreachable recovery never dispatches: Sections 6.1–6.2, 7.1–7.2 |
| DU-17 | Server/auth/timeout/admission failure leaves native spelling usable: Sections 6.2, 7.5, 8 |
| DU-18 | Local failure stays local: Sections 6.2, 7.6 |
| DU-19 | Apply/Dismiss/one-step Undo retain writer authority and trigger no deep work: Sections 4.3–4.4, 7.3/7.5, 8 |
| DU-20 | Qualified model-facing semantics remain identical: Sections 4.2–4.3, 6.2–6.3 |
| DU-21 | One bounded final personal synthetic explicit local smoke: Section 8 |

## 5. Architecture Gate

This gate verifies the architecture established before browser integration:

- `constitution/` preserves every frozen path and byte; strict `constitution.lock.json` has exactly the schema/version, repository URL, freeze ID, 40-lowercase-hex commit, and 40-lowercase-hex tree fields defined by the Implementation Plan, while the copied manifest supplies and verifies inventory and independent hashes without duplicated lock data;
- `core/` compiles under strict TypeScript with exactly its ECMAScript library types and core-authored declarations;
- the committed exact Node/npm/TypeScript tuple, package-manager metadata, engine metadata, exact direct versions, and npm lockfile agree, and the audit passes under that canonical tuple;
- domain values, text policy, reducer, context, validation, and semantic ports use pure TypeScript and the core-owned semantic dependency set;
- every committed TypeScript configuration passes with `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitOverride`, and `noFallthroughCasesInSwitch`; inspection confirms precise types and narrowly justified exceptional external boundaries, and focused compile-time or deterministic runtime tests establish the basis of each exception as required by [`ENGINEERING.md`](ENGINEERING.md#4-compiler-enforced-safety);
- a repository-wide import scan proves that the Zod location allowlist is exactly `core/provider-schema/`, `extension/protocol/`, and the worker-owned trusted-settings boundary;
- imports point from extension composition and adapters toward core, and the core import set consists of core modules and permitted external-boundary dependencies;
- public core declarations expose exactly semantic capabilities and opaque references;
- the repository remains one npm package with exact direct dependency versions, a committed npm lockfile, and a clean `npm ci` install under recorded Node and npm versions;
- the direct runtime dependency allowlist is exactly Zod, and the development dependency allowlist is exactly TypeScript, esbuild, Vitest, Playwright, Chrome types, and Node types, all at exact direct versions;
- React, UI frameworks, the OpenRouter SDK, monorepo tooling, backends, databases, code generation, native scaffolds, and deferred-runtime placeholders remain assigned to future authorized objectives.

The Provider Gate owns runtime-message behavior and external-schema enforcement. The Browser Integration Gate owns manifest, permissions, registrations, storage isolation, DOM runtime behavior, and overlay accessibility.

## 6. Provider Gate

### 6.1 Protocol and trusted-boundary tests

Deterministic worker tests prove:

- strict one-shot `protocolVersion: 2` messages reject unknown versions/types/properties/payloads/senders and use no long-lived Port;
- content operations require the complete active top-level HTTP(S) sender predicate; settings, origin revocation, local discovery and readiness require the exact packaged options sender, with every cross-class combination rejected;
- settings v2 accepts exactly the provider, nested configurations, profile, safe revision and canonical origins in [`SPEC.md`](../SPEC.md#5-settings-authority), with provider-specific model validation and no compiled model default;
- absent settings select local with null model/key, auto profile, revision zero and no origins; valid legacy v1 preserves inactive remote settings/profile/origins, increments once, migrates before authority opens, and never remigrates; corrupt, extra-property, unknown-schema and revision-overflow records fail closed;
- options views expose only the specified models/key-presence flags and safe fields; expected-revision saves require separate Keep/Replace/Clear actions, preserve worker origins, reject stale writes, never echo tokens, and increment only on provider/either-model/either-key/profile changes;
- one active-configuration predicate authorizes inference and Apply; local may omit a key while remote requires one, and remembered inactive settings never drive transport;
- public content configuration remains exactly `isConfigured`/`settingsRevision`, with no provider/profile/model/key; changes cancel requests and broadcast validated updates without retrying old text;
- stale checks resynchronize public configuration without retry; intent/source-correlation/operation tokens, revision and sender metadata enter neither provider payload;
- protocol 2 strictly validates one-shot intent, opaque source correlation, operation identity and Busy/unavailable outcomes; missing, reused, fabricated, wrong-document or retired intent tokens fail before dispatch; an editable menu flag alone grants no textarea authority;
- admission and operation ownership serialize across tabs and diagnostics: exact active identity coalesces with the same deadline, distinct identity is Busy, neither queues, and cache reuse first completes any required recovery and revalidates permission/source/configuration;
- trusted storage admits only the strict text-free marker separately from settings; write-before-POST failure refuses inference, malformed/unknown marker fails closed, and token-checked updates/clear reject late owners; settings changes/restart cannot erase unresolved execution;
- options discovery returns only strictly validated local IDs under 15 seconds; only the exact packaged-options Test command invokes the dedicated fixed-25-second local method with the immutable existing synthetic fixture, one fresh catalog GET, at most one POST and shared production parsing/derivation; a caller cannot choose or pass a longer deadline to writing/inference, discovery, OpenRouter or official corpus operations; synthetic tests read no page, store no raw result, cannot qualify a corpus, change configuration/provider, enable an origin or authorize Apply, and discard settings-change/restart races;
- external model JSON passes the canonical strict schema and pure derivation before the unchanged trusted correction can enter content messages.

### 6.2 Provider transport tests

Shared processing tests inspect both concrete adapters and prove:

- canonical prompt, two-message order, exact bounded split user serialization, schema, content confinement, fetch controls, fatal UTF-8, JSON media type, exact requested/returned model, one index-zero choice, assistant string content, stop finish and strict result/derivation all match [`SPEC.md`](../SPEC.md#9-provider-request);
- malformed/extra-property schemas, multiple corrections, malformed Unicode/CR, excessive or whitespace-only explanation, over-limit focus, unchanged/multihunk correction, profile contradiction, top-level/choice errors and refusals, including HTTP-200 error envelopes, all fail closed;
- writing/inference, independent discovery, OpenRouter and official corpus retain the 15-second deadline through final derivation; only the explicit local Settings synthetic test has a fixed 25-second total bound, including catalog GET and POST; controlled timing proves success beyond 15 but before 25 only for that test, expiry at 25 without retry, and unchanged writing expiry at 15; both paths preserve the 32-KiB incremental bound, fatal decoding, strict envelopes/schema/identity, cancellation and stale-revision behavior even for hanging, oversized, malformed or HTTP-200 error responses;
- each admitted explicit operation makes at most one inference POST; coalescing/cache reuse make none; local inference has one fresh bounded authenticated catalog GET first, shares its full-processing deadline and rejects unknown/wrong-case models before page text is sent; server/model/auth/resource unavailability, timeout and every other failure produce redacted outcomes without retries, response repair, application model substitution or cross-provider fallback;
- credential/header isolation, bounded page-text confinement and no private body/log/snapshot/history/telemetry leakage hold for success and failure paths;
- controlled transport distinguishes no possible POST, known terminal completion and possible dispatch with unknown completion; abort does not imply provider termination;
- unresolved local execution permits exactly one authenticated `GET http://127.0.0.1:8000/api/status` on the next admitted explicit action under its original full-processing deadline: only valid nonnegative safe integer zero `models_loading`, `active_requests`, `waiting_requests` permits normal catalog/POST; missing, malformed, busy, loading, auth failure, unreachable or timeout returns unavailable and retains recovery; idle is a snapshot, no atomic reservation or memory-admission guarantee;
- remote uncertainty holds the exclusive lease only until its original 15-second deadline, then releases without claiming termination; a subsequent action may dispatch while every late old callback remains fenced; restart reconstructs only bounded remaining time;
- idle, twenty edits, 600-ms expiry, IME commitment, focus, navigation, startup/wakeup and settings saves all make zero inference/catalog/readiness/status calls; no path adds model load/warm-up/keepalive/pinning or weakens oMLX's memory guard.

Local-specific tests prove the exact loopback endpoint, case-sensitive direct catalog selection, optional auth, `max_tokens: 8192`, `temperature: 0`, `stream: false`, `tool_choice: "none"`, `enable_thinking: false`, `thinking_budget: 0`, strict schema and complete absence of OpenRouter fields. The server is configured with `model_fallback: false`; oMLX may match case-insensitively, so the adapter’s fresh exact-membership catalog guard rejects unknown and wrong-case IDs before POST; aliases/profiles/fallback are never selected. Warning-199 grammar downgrades fail even with valid JSON. Local writing and the explicit startup test, including failures, cause zero remote inference traffic. The test has no separate preload, automatic/background warm-up, retry or pinning requirement, reports readiness only after shared production parsing/derivation, and cannot guarantee model residency or future latency after restart/eviction. Loopback binding, `critical` logging, synthetic leakage canaries and allowed local KV cache state are recorded without claiming native crash diagnostics are suppressed.

OpenRouter-specific tests preserve its fixed endpoint, required base-model/key grammar, exact disabled plugins, `max_completion_tokens: 8192`, `reasoning.exclude: true`, routing/data-collection constraints and same-model endpoint fallback. No local thinking/tool-choice field, model array, tool/server tool, plugin enablement, SDK, transform, attribution or extra payload field enters remote requests. Within-request fallback does not guarantee success within the deadline. Selected model is available only to sanitized evidence; pre-response failure records it unavailable.

### 6.3 Live provider evidence

Sprint 3 may inherit the ledger's qualified `gemma-3-12b-it-4bit` linguistic evidence at implementation `5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63`: one complete 15/15 production run and two independent complete semantic reviews, with all environment, artifact and limitation fields preserved. First prove canonical prompt bytes, model-facing strict schema, serialized user-message property order, generation fields and provider-specific linguistic payloads identical; compare focus/context on all corpus and scalar boundary fixtures; replay synthetic responses through parsing, fixed/auto profile validation, derivation and trusted corrections; prove new intent/source/operation tokens never enter provider input. Record both predecessor and successor identities and results. This inherits linguistic evidence only, never scheduler/browser/runtime evidence.

Model weights/identity, prompt, profiles, context/focus, response schema, generation semantics or derivation changes invalidate inheritance and require fresh complete qualification. When required, run the following unchanged corpus through production parsing and derivation using one directly configured documented model and the active provider’s actual credential policy; remote runs, when performed, are separate evidence and record the no-enforced-plugin-policy precondition. Record provider, requested case-sensitive model, server/build policy, measured cold/warm latency and memory observations with Brave running. The run, not discovery/readiness or settings syntax, qualifies observed compatibility; every returned model equals the requested ID. In every case, `before` and `after` are empty and the Focus column is the complete focus. Calls are strictly sequential: a case does not start until the preceding case terminates. Every official case retains the 15-second full-processing deadline; no case is retried or replaced within a run. Any explicitly performed local Settings synthetic test before the run is separate preparation with its own recorded outcome and actual cold/warm state, not a corpus case or qualification credit. The runner never performs an implicit preload, warm-up or per-case retry.

Independent named agent semantic reviewers collectively cover every corpus language/profile and every case, and compare each strictly parsed and derived result against the table after automated structural and exact-string checks. They review the actual returned profile, correction or clean/unsupported decision, category, explanation, language and meaning preservation independently of the implementation owner. Metadata records agent/model identity, profile/case coverage, method and findings truthfully as agent review. `linguistic correctness` is this qualification judgment, never misrepresented as human review or guaranteed future correctness. The writer still approves every actual suggestion; schema validity, temperature zero and string equality alone do not prove semantics.

| Case | `profileMode` | Focus | Required result |
| --- | --- | --- | --- |
| `de-CH-correction` | `de-CH` | `Dies ist ein synthetischer Satzz.` | one `spelling` correction to `Dies ist ein synthetischer Satz.` |
| `de-CH-clean` | `de-CH` | `Dies ist ein synthetischer Satz.` | `de-CH` with no correction |
| `en-GB-correction` | `en-GB` | `This is a synthetik sentence.` | one `spelling` correction to `This is a synthetic sentence.` |
| `en-GB-clean` | `en-GB` | `This is a synthetic sentence.` | `en-GB` with no correction |
| `en-US-correction` | `en-US` | `This is a synthetik sentence.` | one `spelling` correction to `This is a synthetic sentence.` |
| `en-US-clean` | `en-US` | `This is a synthetic sentence.` | `en-US` with no correction |
| `fr-FR-correction` | `fr-FR` | `Ceci est une phrase synthétiqe.` | one `spelling` correction to `Ceci est une phrase synthétique.` |
| `fr-FR-clean` | `fr-FR` | `Ceci est une phrase synthétique.` | `fr-FR` with no correction |
| `ka-GE-correction` | `ka-GE` | `ეს არის სინთეზური წინადადება..` | one `punctuation` correction to `ეს არის სინთეზური წინადადება.` |
| `ka-GE-clean` | `ka-GE` | `ეს არის სინთეზური წინადადება.` | `ka-GE` with no correction |
| `ru-RU-correction` | `ru-RU` | `Это синтетическое предложение..` | one `punctuation` correction to `Это синтетическое предложение.` |
| `ru-RU-clean` | `ru-RU` | `Это синтетическое предложение.` | `ru-RU` with no correction |
| `auto-fr-FR` | `auto` | `Ceci est une phrase synthétique.` | `fr-FR` with no correction |
| `fixed-en-GB-German` | `en-GB` | `Dies ist ein synthetischer Satz.` | `unsupported` with no correction |
| `auto-unsupported-Japanese` | `auto` | `これは合成の文です。` | `unsupported` with no correction |

The Provider Gate requires 100% of its deterministic assertions plus either verified equivalence and explicitly inherited published qualification or a required fresh complete live run with `15/15` successes. In a fresh run, a case succeeds only when it completes inside the canonical deadline, passes the strict schema and local semantic derivation, matches the table's required result, and is linguistically correct. This observed qualification is not a future reliability guarantee. Any failure remains factual and leaves the gate incomplete. After an implementation, configuration, or external-service change, a new complete 15-case attempt may be recorded as separate recovery evidence; individual failed cases are never retried in place. Missing required credentials, unavailable server/model, resource failure, exhausted quota, or an interrupted corpus also leaves the gate incomplete.

For each case record only the case identifier, selected model or `unavailable`, complete request latency, outcome, failure reason when any, and linguistic correctness. General evidence metadata from Section 1 still applies. Do not calculate percentiles, distributions, or a stochastic pass percentage, and do not run a concurrent stress corpus as part of this gate. No live record contains the credential or raw private text.

## 7. Browser Integration Gate

Automated extension tests run in Playwright's [bundled Chromium persistent context](https://playwright.dev/docs/chrome-extensions) against the production unpacked build. They instrument extension handlers and programmatic Chrome events but do not claim to operate browser toolbar or permission UI; actual user-gesture cases are owned by the headed manual tests in Section 8.

### 7.1 Manifest, storage, and configuration

Runtime tests prove:

- the Manifest V3 package declares `minimum_chrome_version` as `"140"`, grants provider access only to the exact required set `http://127.0.0.1:8000/*` and `https://openrouter.ai:443/*`, adds only `contextMenus` to the predecessor locked permissions, has no static all-sites content script or `<all_urls>` grant, disables incognito, and bundles executable code locally;
- action, context-menu `onClicked`, message, and permission listeners register synchronously at worker module evaluation, and every handler awaits one shared sticky initialization promise;
- worker initialization awaits `chrome.storage.local.setAccessLevel({ accessLevel: "TRUSTED_CONTEXTS" })` before any settings read or write and fails closed when the method is unavailable or rejected;
- the Chrome-140-compatible `runtime.onMessage` listener is not `async`, starts asynchronous dispatch, responds on every handled path through `sendResponse`, and returns literal `true` synchronously, including after a cold worker restart;
- a content script cannot read `chrome.storage.local` and cannot receive its change events;
- only the worker reads or writes both provider credentials/models and the full settings record; the options page communicates through the worker;
- fresh, valid-v2, valid-v1 migration, corrupt, extra-property and unknown-schema settings follow the exact v2/migration/fail-closed contract;
- content initialization receives and caches only `isConfigured` and `settingsRevision`, then updates that cache only through validated messages rather than fetching before each capture;
- worker restart preserves intended settings/origins and the separate text-free execution marker, retires predecessor tokens, clears the cache and performs no provider inspection;
- cache and operation fixtures across two tabs and options Test prove exclusive inference, exact coalescing, Busy without backlog and no raw diagnostic result retention; inspection proves serialized retention limits and text-free trusted persistence;
- context-menu create/update/remove reconciliation is idempotent, adds no duplicate item or permission, and follows current enabled-origin state; startup registration itself makes no provider traffic.

### 7.2 Enablement and revocation

Tests with multiple tabs and origins prove:

- an instrumented action-listener test proves top-level HTTP(S) validation and that its registered handler invokes `permissions.request` before any await, rejects a duplicate pending origin, and leaves no durable change after simulated denial, including from a cold worker;
- the sole origin-to-pattern function always emits an explicit default or nondefault port and is used by permission request, containment, removal, registration, and reconciliation; adjacent ports, schemes, hosts, and subdomains remain unauthorized;
- after grant, all post-prompt lifecycle work is serialized, the origin is persisted, and registration `emenda-enabled-origins` is created or updated with the exact enabled-origin match set, packaged content entry, isolated world, document-idle run time, persistence, `allFrames: false`, and `matchOriginAsFallback: false`;
- the worker pings the current tab and injects the same packaged entry in isolated-world frame 0 only when no script responds;
- repeated initialization creates no duplicate listener, controller, registry, or overlay;
- zero enabled origins produces zero dynamic content-script registrations;
- every content message has a fresh Chrome sender and is rejected unless extension ID, tab ID, frame 0, active lifecycle, nonempty document ID, HTTP(S) URL, matching nonopaque origin, enabled-origin membership, and current exact permission all validate; payload authority and `sender.tab.url` are ignored, while options-only commands, missing fields, opaque origins, iframes, prerender senders, stale settings, adjacent ports, and contradictory fields fail closed;
- one-shot messaging is used throughout and no long-lived runtime Port exists;
- startup reconciles corrupt or interrupted durable state, current exact grants, and the fixed registration before content or provider work opens; both required provider patterns survive reconciliation, confer no implicit writing-origin authority, and zero enabled origins still has no registration; external `permissions.onAdded` removes unowned or broader grants without racing Emenda's pending exact prompt, external `permissions.onRemoved` performs disablement and cancellation, and every inference and Apply authorization repeats the current permission check;
- revocation first disables the origin and rejects new checks and Apply authorizations, then cancels associated requests and sends versioned origin-bound `Deactivate` to known document IDs;
- after an external removal or incomplete known-document set, unfiltered `tabs.query({})` drives a best-effort frame-0 broadcast without reading tab URLs; each receiver ignores a control message whose origin differs from its current nonopaque `location.origin`, including a forced cross-origin navigation race;
- deactivation invalidates revision authority, cancels settling and inference, removes input and composition listeners and the overlay host, clears source and snapshot registries, and leaves the script inert;
- the registration is updated or removed before the exact optional permission is removed, while other enabled origins remain active;
- post-grant failure rolls settings, registration, and permission back best-effort, while startup reconciliation completes an interrupted rollback;
- overlapping settings save, enable, revoke, external-addition/removal, and startup work for two origins is FIFO-serialized from freshly read state and converges without a lost setting or origin, overbroad permission, or registration drift;
- `pagehide` tears authority and page-owned state down, every BFCache `pageshow` reauthorizes before resuming, and a document that starts prerendering defers handshake, observation, composition, and UI until one `prerenderingchange` reauthorization;
- an already-injected script cannot check, Apply, or resume after registration removal, permission removal, failed teardown delivery, or worker restart unless the current origin is enabled and reauthorized again.

Restricted pages, file URLs, PDFs, iframes, and incognito fail closed.

### 7.3 Textarea and safe Apply

On a visible, window-focused, active, writable, midpoint-exposed, sequentially keyboard-focusable light-DOM `<textarea>`, tests prove committed input, exact local settling with zero provider traffic, then a separate explicit menu action, at most one request, suggestion, Dismiss, and off-caret insertion, deletion, and replacement. Real keyboard, paste, and IME fixtures produce the required trusted `beforeinput`/`input` tickets and exact pre/post tuples even when a `beforeinput` listener queues a microtask before browser mutation. Matching delayed or coalesced selection notification after real keyboard or paste input preserves only the local settling timer; an intervening moved or mismatching selection cancels them. Synthetic dispatch, value-only changes, a ticket used after its private expiry callback, wrong-source/generation input, and page-invoked `execCommand` during a click/keydown or outside a gesture when no provenance ticket is outstanding create no revision or request. A stale expiry callback cannot clear a newer opaque ticket. Tests separately record that page work nested in a genuine trusted `beforeinput` or queued ahead of its expiry callback can consume that one-use ticket and is an explicitly bounded enabled-origin limitation. Trusted paired IME generations qualify and may use an in-bounds losslessly mapped noncollapsed candidate range only during their baseline-only intermediate phase; untrusted or unpaired start/input/end paths do not. Ordinary capture and final verification require a visible document, `document.hasFocus()`, the exact active textarea, its connected document, exact value, exact collapsed selection, and the same exposure predicate.

Context-menu fixtures separately prove a trusted current candidate plus browser-owned menu invocation, fresh document/sender and permission validation, controlled source-only focus restoration and immediate bounded capture without prior typing. Preserve native menu behavior. An editable input/contenteditable/iframe cannot authorize a textarea; synthetic menu/candidate events, replaced/detached sources, navigation, changed value/selection/configuration, composition, lost permission and window focus reject without choosing a different source or reading refused editor text. Repeat races between opening and clicking and between worker intent and content reply. Identical unchanged actions retain correlation/revision and coalesce/reuse; actual edits immediately retire authority.

Apply begins only from the current trusted Apply control after the deliberate approval-UI focus handoff, then obtains one worker authorization for the current `settingsRevision` containing no page text or surface identity. On success, focus must remain in that control; the adapter restores the captured textarea and caret, verifies current authority plus source, document, foreground focus, opaque snapshot, writability, exact logical text, scalar mapping, captured selection, and exact original substring, then selects the exact mapped correction range inside one scoped synchronous internal phase. Tests prove selection observation is suspended only for that call and immediate readback, the unchanged value and original substring are rechecked, no queued selection event is awaited or required, and the sole mutation is `document.execCommand("insertText", false, replacement)`.

Success requires a `true` return, the exact synchronous self-authored input, and exact expected post-state. It is consumed as `AppliedChange`, updates the returned post-edit text and selection baseline, starts no settling or inference, preserves textarea focus, and one native Undo restores the exact original text. Delayed and coalesced `select`/`selectionchange` fixtures are ignored only when the current source and selection match that baseline; a writer change invalidates normally. A changed source, document, snapshot, value, captured or target selection, foreground focus, writability, mapping, original, or worker authority refuses text mutation and enters `Error`; an unchanged failure best-effort restores and records the captured caret. A worker restart with a visible suggestion first completes sticky initialization and then applies the same current authorization predicates; initialization failure refuses. Forced teardown-delivery failure, external permission removal, hidden tab, window blur, page capture-phase change, `false`, throw, missing input, mismatching input, and unexpected post-state prove fail-closed behavior. An unexpected changed state refreshes baseline/authority and prevents a false no-mutation claim, but reserves and settles only with independent eligible paired provenance; no fallback mutation runs.

### 7.4 Refused surfaces and mapping boundary

Textarea fixtures prove exact value capture and lossless bidirectional conversion at every Unicode-scalar and UTF-16 boundary, including ASCII, combining sequences, emoji, and supplementary-plane scalars. Lone surrogates, raw CR, a boundary inside a surrogate pair, an ordinary non-collapsed selection, and every non-round-tripping correction range fail before inference or mutation. An eligible intermediate IME candidate range is baseline-only and cannot reach either operation.

Explicit refusal fixtures prove that inputs, every contenteditable form, iframes, shadow-DOM editors, rich, virtualized, canvas, and Google Docs-style editors, hidden or offscreen textareas, readonly, disabled, inert, non-`tabIndex === 0`, disconnected, background-document, and ambiguous surfaces have no text read or captured and never infer or mutate. Baseline-only recapture is confined to an otherwise supported textarea whose provenance pairing failed. The shared exposure fixtures cover an exposed midpoint, a page cover at input, a cover added during settling, a cover added before Apply, and the current Emenda host over the point: intersect the client rect with the layout viewport, hit-test its midpoint, skip only that host, and require the textarea as the first remaining element. Record that DOM hit-testing does not prove compositor-only or `pointer-events: none` visual occlusion. Bundle inspection proves there is no contenteditable or DOM-tree logical-text mapper.

### 7.5 IME, failures, and accessibility

Real event tests prove every composition start invalidates immediately and binds its collapsed pre-composition tuple; only a trusted start on a qualifying textarea creates an eligible generation; paired composing input survives listener-queued microtasks, while an untrusted or unpaired change, input after its ticket's expiry callback, an untrusted end, or a failed end exposure check makes it baseline-only or disqualifies the generation as specified; and only a trusted qualifying end at a collapsed caret after paired changes and a real terminal text change creates the single committed change and rebinds the terminal baseline to its new revision. A Chrome fixture covers a start at `[3,3]`, noncollapsed intermediate candidate ranges `[3,5]` and `[4,6]`, and a final `[6,6]`: the intermediate updates create zero requests, the end creates exactly one committed change, and its queued matching selection notification drains without cancelling the one local settling timer; 600-ms expiry still creates zero provider requests. Explicit intent is tested separately after composition ends. Cancelled and no-op IME generations create none. A terminal pair is deduplicated only when text, source, and selection match within that generation; each mismatch is external input and must independently qualify. The exact registered self-authored Apply input succeeds without a `beforeinput` ticket and no other event can use that bypass.

Presentation tests prove the locked silent and `Error` mappings, Busy without backlog, and Deep check temporarily unavailable for local failure/recovery/admission refusal, including Open Settings when configuration is the remedy. Native spelling remains independent through server/auth/timeout/admission failures. A suggestion presents the complete original and reconstructed corrected focus, marks exactly the one changed hunk, and separately shows category, explanation, Apply, and suggestion Dismiss. `[empty]`, changed whitespace, control, format, combining, and bidirectional-safety fixtures prove deterministic visible markers and bidi isolation. A content error presents only its redacted message, current error Dismiss, and Open Settings when applicable; suggestion and error capabilities cannot clear or act on each other.

The fixed, unanchored host follows the current textarea in sequential DOM order, owns a closed shadow root, and never autofocuses. Tests tab from the textarea into the first control and among all current controls, activate native buttons by pointer and ordinary Enter/Space, then prove Apply and either Dismiss variant best-effort restore safe unchanged textarea focus and selection. V0.2 registers no page-level custom shortcut. Only a trusted current-control event acts: synthetic page clicks or keys, page-focused input, stale controls, and controls or hosts hidden, moved, DOM-hit-test-covered, made transparent, or disconnected before target handling do nothing. Closed-root and document hit tests must agree for pointer activation, accepted control events do not bubble into page handling, and page capture-phase changes are caught by final verification. Browser evidence records that compositor-only and `pointer-events: none` covers remain outside this proof and are an enabled-origin trust limitation.

Page and model strings containing HTML, Markdown, URLs, event attributes, or script-shaped text render literally through text nodes or `textContent` and create no markup, link, or execution. Each new suggestion or content error emits exactly one polite notification; settling and checking emit none. Accessible names, coherent focus order, visible focus, reduced motion, WCAG 2.2 AA contrast, and non-color meaning all pass. Toolbar names are only Enable or Reactivate; incomplete configuration opens Settings after successful activation, and site-access text accurately states the bounded trusted-input piggyback limitation. Action badge/title and options tests separately cover activation and revocation-command errors.

### 7.6 Confinement inspection

Bundle and runtime inspection prove Emenda-authored messages omit page URLs, tab/frame/document metadata, any separate or unbounded full-document field, source references, snapshot references, and DOM data; the only page text copied from content to worker is the bounded context, which may equal all text of a short document. One-use intent, opaque source-correlation and operation tokens, revision identity, focus range and `settingsRevision` also cross only as the nontext protocol fields declared by the specification and never enter provider input. Chrome-supplied sender metadata reaches the worker only as ephemeral authorization input: only the required fields are inspected, and none is persisted, logged, copied into errors, or forwarded to either provider. Both credentials remain confined to the strictly validated worker-owned trusted-settings record and their owning active provider call; neither enters a content message, log, fixture, snapshot, error, evidence, or bundle. Raw private text enters no durable storage, log, fixture, snapshot, error, or evidence. The product contains no persistent text cache, analytics, telemetry, or remote executable code.

## 8. V0.2 Conformance Gate

The final gate requires all prior evidence to remain valid for the tested implementation tree and commit, plus:

- a clean checkout verifies the read-only constitution/lock, canonical Node/npm/TypeScript tuple and committed dependency graph via `npm ci`; complete deterministic, production build, audit/CI and Playwright bundled-Chromium persistent-context suites pass;
- dependency, bundle, permission, manifest, registration, redaction and privacy inspections match this freeze, including zero remote inference traffic in local mode and sanitized synthetic server-log canaries;
- the existing browser harness runs against the exact installed Brave executable and records Chromium/Brave build, actual Mac chip/RAM/OS, local server build/model/policy, latency and memory observations;
- personal unpacked-extension verification in the writer's existing Brave profile covers actual toolbar grant/denial, exact-port activation/revocation, storage isolation, context-menu source/focus handoff, sender/document lifecycle, local settling, zero ambient provider traffic, complete suggestion, Dismiss, off-caret insertion/deletion/replacement, writer-approved Apply, one native Undo, navigation invalidation and worker/browser/server restart recovery; synthetic controlled-provider cases supply editing paths without repeated real-model exploration;
- with oMLX stopped, native German `Feler` and English `sentense` spelling indications and native menu candidates remain usable; twenty edits, pauses, focus, navigation and worker/browser wakeup produce zero provider calls and no added language process or model management;
- after all inexpensive deterministic/browser/identity checks pass, perform one bounded synthetic explicit local proofread smoke through the exact installed production path within 15 seconds, with actual cold/warm/resource observations; no implicit warm-up, retry loop, model search or memory-guard relaxation is allowed; a refusal or failure is retained as a limitation and does not pass V0.2 Conformance;
- local model qualification is one preserved sequential 15/15 production-path run with the unchanged 15-second per-case deadline plus complete independent named agent semantic review; any explicit fixed-25-second Settings synthetic test is recorded separately as preparation, grants no qualification credit or authority, and does not guarantee future residency/latency; earlier failed attempts remain evidence;
- supported/refused surfaces, accessibility, reduced motion, enabled-origin residual risks, local/remote processing, credentials, logging/cache/native-diagnostic limits and possible remote charges are accurately disclosed;
- all 23 critical IDs have their required evidence, with deterministic proof, agent qualification judgment, actual writer approval, live observation and environment-specific runtime evidence kept distinct;
- the exact tested implementation commit is pushed and remotely verified, the verified production `dist/extension/` build is installed into the user’s Brave profile and usable after restart, and a later blueprint commit changes only `docs/EVIDENCE.md` to record already-existing tested implementation identities;
- the later ledger commit is pushed/verified, every frozen file remains unchanged and both tracked worktrees are clean.

Automated bundled Chromium, actual installed Brave and the personal Mac remain distinct evidence layers. Chrome 140 remains the declared minimum, but unperformed direct Chrome-140/current-Chrome/other-device tests are explicitly recorded as unproven compatibility and never inferred from Brave. Additional Windows, Chromebook and other runtime results supplement the personal objective when actually performed; they are not required to claim this recorded Mac/Brave installation.

After V0.2 Conformance passes, stop. Store distribution, native work, commercial work, broader surfaces, and general cross-platform claims require separately versioned objectives.
