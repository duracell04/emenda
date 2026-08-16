# Emenda V0.1 Acceptance

> **Frozen acceptance contract, version 2.0.2**

## 1. Role and evidence standard

This document derives verifiable gates from [`SPEC.md`](../SPEC.md), [`ARCHITECTURE.md`](ARCHITECTURE.md), and [`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md). It does not define a separate product or build order.

A gate passes only through reproducible evidence recorded at the tested state. Use these evidence levels precisely:

```text
inspected
compiled
deterministic
integration
live
runtime
```

Each entry records environment and tool versions, the relevant command or manual procedure, exact sanitized outcome, limitations, and:

```text
tested implementation tree:
tested implementation commit:
```

An evidence commit records an already-existing implementation commit that was actually tested. Later success does not erase an earlier failure; record failure and recovery separately. Never record credentials, authorization headers, raw private text, full provider bodies, source identities, or DOM data.

Every deterministic assertion required by a gate must pass; there is no partial deterministic pass. The Provider Gate's live qualification has the separate factual standard in Section 6.3.

## 2. Gates and current stop boundary

The six gates are:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

The current v2.0.2 objective ends after the Documentation Gate: preserve v2.0.1, rewrite, verify, hash, commit, and push its direct-child 13-file Markdown freeze, confirm remote identity and a clean worktree, then stop. All implementation gates require a separate future objective.

## 3. Documentation Gate

The gate passes only when:

- the commit is a documentation-only direct child of v2.0.1 commit `d70b277998a23663ee6befc77dd6bb0da50ebcca`, while v2.0.0 commit `a1a13607867db8e6eb2ea904f6387ba130f22ce7` remains in its ancestry;
- the tracked Markdown inventory is exactly the 13 paths declared by `PACKAGE-MANIFEST.md`, with no implementation source added;
- all documents identify version 2.0.2, and every freeze-ID occurrence is `emenda-clean-room-v2.0.2-2026-08-16`;
- `SPEC.md`, `docs/ARCHITECTURE.md`, and `docs/IMPLEMENTATION-PLAN.md` remain the authorities for behavior, architecture, and build order, with supporting documents introducing no contradiction;
- every occurrence of the canonical sequence is byte-identical and describes a Documentation baseline followed by seven implementation increments numbered 1 through 7;
- the Documentation Gate is a prerequisite rather than an implementation increment, and the six gate names and order are unchanged;
- every local Markdown link resolves;
- native and commercial work appears only as explicitly deferred work;
- the 11 immutable documents match individual SHA-256 values computed from exact staged Git-blob bytes;
- `PACKAGE-MANIFEST.md` and the empty `docs/EVIDENCE.md` ledger template are excluded from the checksum table;
- the evidence template uses `tested implementation tree` and `tested implementation commit` and makes no implementation claim;
- inspection finds no duplicated normative rule that conflicts with its owning authority;
- `git diff --check` passes and the staged diff contains exactly the 13 Markdown files;
- one documentation decision is committed and pushed, local and remote commit identities match, ancestry is verified, and the worktree is clean.

No package, source, test, build output, audit script, tag, or implementation evidence is created during this objective.

## 4. Mock Product Gate

### 4.1 Unified state machine and authority

Fake-clock and reducer tests prove:

- one pure reducer owns `Idle | Debouncing | Checking | Suggestion | Applying | Error` and effects own timers, inference, messaging, storage interaction, and surface operations;
- each eligible committed input reserves a new revision synchronously, clears a current error or suggestion, cancels older work best-effort, and starts one trailing 600 ms debounce;
- no request starts at 599 ms and exactly one eligible request starts at 600 ms; later committed input replaces the timer;
- each eligible revision produces at most one request, and current revision authority wins every completion and cancellation race;
- stale results, stale failures, stale settings revisions, and stale Apply or Dismiss commands do not change presentation or text;
- API-key, model, and profile changes increment `settingsRevision`, cancel active inference, invalidate visible suggestions, and leave processing paused until the next committed input; origin changes do not increment it;
- the simulated public configuration contains only `hasApiKey`, `profileMode`, and `settingsRevision` and is replaced in every live enabled controller by validated update events rather than fetched before capture;
- composition start invalidates immediately, composing input starts no inference, and composition end emits the sole committed change;
- an identical terminal input after composition end is deduplicated within that composition generation, while a divergent terminal input reserves a normal external revision.

### 4.2 Deterministic text policy

Pure tests cover ASCII, Georgian, Russian, combining sequences, emoji, and supplementary-plane scalars and prove:

- every range is half-open and measured in Unicode scalars rather than UTF-16 units, bytes, or graphemes;
- a non-collapsed selection fails silently; a collapsed caret selects deterministically;
- paragraphs are maximal scalar ranges between explicit LF boundaries;
- `.`, `!`, and `?` terminate only before Unicode `White_Space`, LF, or end; trailing `Pe`/`Pf` punctuation and U+0022/U+0027 quotation marks remain in the sentence;
- intersentence whitespace belongs to the preceding nonempty sentence, while paragraph-leading whitespace belongs to the following sentence;
- a focus without a Unicode Letter scalar (`\p{L}`) produces no request;
- context and focus obey the canonical scalar limits in [`SPEC.md`](../SPEC.md#3-v01-runtime-and-limits), with the complete focus present, deterministic surrounding-context allocation, and silent refusal above the focus limit;
- complete paragraphs, truncation, even division, odd trailing allocation, boundary clamping, and backfill behave exactly as specified;
- model-authored corrected focus is compared as Unicode scalars without normalization or relocation, and a derived correction maps from focus-relative to context-relative to snapshot-relative coordinates exactly once.

### 4.3 Validation, failures, and presentation

Semantic validation and mock-provider cases prove:

- clean, empty, nonlinguistic, unsupported-language, over-limit-focus, non-collapsed-selection, and ordinary unsupported-capture outcomes return silently to `Idle`;
- supported `auto` results are accepted, fixed mode accepts only its exact profile or `unsupported`, `unsupported` is accepted only with an empty correction list and returns to `Idle`, and a different supported profile is invalid provider output with no suggestion and `Error`;
- the external result accepts only the strict shape in [`SPEC.md`](../SPEC.md#8-model-facing-contract-and-local-derivation), while the worker-to-content result contains only the trusted derived correction and never model-authored `correctedFocus`;
- minimum Unicode-scalar edit distance produces the specified deterministic result for insertion, deletion, substitution, adjacent edits, repeated-character ties, combining sequences, emoji, and supplementary-plane scalars;
- unchanged `correctedFocus`, separated edit hunks, excess corrected-focus length, malformed language combinations, and non-reconstructing or unmappable derivations are rejected;
- one accepted hunk derives the exact half-open range, `original`, and `replacement`, remains inside focus, and reconstructs `correctedFocus` exactly;
- a whole-focus translation-shaped replacement can satisfy the structural one-hunk rule, but the system never represents that fact as proof of semantic preservation;
- missing configuration enters `Error` with Open Settings;
- current timeout, provider failure, invalid response, and Apply refusal enter `Error`;
- stale completion or cancellation causes no presentation change;
- one current valid correction creates one `SuggestionId` capability; Dismiss mutates nothing;
- Apply reaches the surface only after the controller verifies the current suggestion, current revision, and their association.

### 4.4 Complete simulated product

The deterministic composition proves the full loop from committed input through revision, debounce, cached settings, capture, context, inference, validation, suggestion, Apply or Dismiss, and final state. Mocks cover clean and correction results, delayed stale completion, timeout, cancellation race, source and snapshot changes, changed text, lost focus, readonly state, mapping refusal, exact replacement, and self-authored replacement acknowledgement.

An exact expected self-mutation updates the post-edit baseline, emits no new observed change, advances authority without inference, and returns to `Idle`. A mismatch is classified as external input, reserves a revision, and prevents the old Apply result from becoming authoritative.

## 5. Architecture Gate

This gate verifies only the architecture that exists before browser integration:

- `core/` compiles under strict TypeScript while DOM, Chrome, Node, React, and extension types are unavailable;
- domain values, text policy, reducer, context, validation, and semantic ports contain no Zod or runtime mechanisms;
- Zod use inside core is confined to `core/provider-schema/`;
- imports point from extension composition and adapters toward core, and core never imports `extension/`;
- public core declarations expose semantic capabilities and opaque references rather than browser, timer, transport, storage, or UI objects;
- the repository remains one npm package;
- Zod is the only direct runtime dependency, and development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, and Chrome/Node types;
- no React, UI framework, OpenRouter SDK, monorepo tool, backend, database, code generation, native scaffold, or deferred-runtime placeholder exists.

Runtime-message behavior and external-schema enforcement are not Architecture Gate criteria. Manifest, permissions, registrations, storage isolation, DOM runtime behavior, and overlay accessibility are not Architecture Gate criteria.

## 6. Provider Gate

### 6.1 Protocol and trusted-boundary tests

Deterministic worker tests prove:

- runtime messages use versioned, discriminated, strict Zod envelopes, and reject unknown versions, unknown types, extra properties, malformed payloads, and disallowed senders;
- trusted settings accept exactly `schemaVersion`, `apiKey`, `model`, `profileMode`, `settingsRevision`, and `enabledOrigins` with fail-closed migration or corruption handling;
- new settings use the canonical default model and `profileMode: auto`, valid existing concrete-model overrides remain authoritative, and the advanced model option can change that trusted setting;
- public configuration messages contain exactly `hasApiKey`, `profileMode`, and `settingsRevision`;
- settings-change messages are validated and delivered to every live enabled content script, API-key/model/profile changes increment the revision and cancel current requests, and origin changes follow separate lifecycle messages;
- every check carries `settingsRevision`, the worker rejects a stale value before using the key or model, cache resynchronization is possible, and the rejected revision is not retried;
- `settingsRevision` is absent from the OpenRouter payload;
- external model JSON is accepted only through the strict canonical schema before semantic derivation;
- the provider-authored corrected focus cannot enter the worker-to-content envelope, whose trusted derived correction shape remains unchanged.

### 6.2 OpenRouter transport tests

Tests inspect the exact outbound request and prove:

- the endpoint and every request field match the canonical provider contract in [`SPEC.md`](../SPEC.md#9-provider-request);
- the canonical default route and an advanced concrete-model override are each sent as the one trusted model value, with no `models` array;
- the model-facing user payload is exactly the split bounded input in [`SPEC.md`](../SPEC.md#8-model-facing-contract-and-local-derivation) and excludes URL, full document, source or snapshot identity, DOM structure, API key, and Emenda settings metadata;
- the strict structured-output schema rejects missing or extra properties and any correction count above one;
- the implementation uses a currently supported generation-limit parameter with a tested budget sufficient for the maximum corrected focus and schema envelope; the exact selected parameter and value are asserted by its deterministic request test;
- temperature, routing, one-request behavior, zero application retries, deadline, incremental response bound, and cancellation match the canonical contract;
- local Zod validation and semantic derivation follow transport parsing, while request and revision identity remain Emenda-authored;
- HTTP, transport, timeout, size, parse, schema, semantic, and unsupported outcomes are typed and redacted;
- authorization headers, credentials, raw contexts, and raw response bodies cannot enter logs, snapshots, errors, or telemetry;
- there is no healing, streaming, response cache, telemetry, OpenRouter SDK, application-level retry, `models`-array failover, or application-level model substitution;
- within-request provider fallback is enabled without being represented as a guarantee of immediate fallback, a changed model, or completion inside the deadline;
- a successful response's selected model is available to the live evidence path, while pre-response failure records it as unavailable.

### 6.3 Live provider evidence

Run the following corpus once through the production parsing and derivation path using the canonical default `openrouter/free` route. In every case, `before` and `after` are empty and the Focus column is the complete focus. Calls are strictly sequential: a case does not start until the preceding case terminates. An official case is neither retried nor replaced. Runs using an advanced model override and reruns of failed cases are separate diagnostics and do not alter the official result.

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

The Provider Gate requires 100% of its deterministic assertions and one complete 15-case live run. A case counts as a success only when it completes inside the canonical deadline, passes the strict schema and local semantic derivation, matches the table's required result, and is linguistically correct. The live run is qualification evidence rather than a reliability guarantee or an all-or-nothing success threshold. Report the factual success count as `x/15`; provider failures remain visible while deterministic fail-closed behavior protects the writer. Missing credentials, exhausted quota, or an interrupted corpus leaves the gate incomplete.

For each case record only the case identifier, selected model or `unavailable`, complete request latency, outcome, failure reason when any, and linguistic correctness. General evidence metadata from Section 1 still applies. Do not calculate percentiles, selected-model distributions, repeated-round statistics, or a stochastic pass percentage. Do not run a concurrent stress corpus as part of this gate. No live record contains the credential or raw private text.

## 7. Browser Integration Gate

Automated extension tests run in Playwright's [bundled Chromium persistent context](https://playwright.dev/docs/chrome-extensions) against the production unpacked build.

### 7.1 Manifest, storage, and configuration

Runtime tests prove:

- the Manifest V3 package declares `minimum_chrome_version` as `"140"`, uses only the locked permissions, has no static all-sites content script or `<all_urls>` grant, disables incognito, and bundles executable code locally;
- worker initialization awaits `chrome.storage.local.setAccessLevel({ accessLevel: "TRUSTED_CONTEXTS" })` before any settings read or write and fails closed when the method is unavailable or rejected;
- a content script cannot read `chrome.storage.local` and cannot receive its change events;
- only the worker reads or writes the API key, model, and full settings record; the options page communicates through the worker;
- content initialization receives and caches only `hasApiKey`, `profileMode`, and `settingsRevision`, then updates that cache only through validated messages rather than fetching before each capture;
- worker restart preserves only the intended durable settings and origin state.

### 7.2 Enablement and revocation

Tests with multiple tabs and origins prove:

- toolbar enablement accepts only a top-level HTTP(S) tab and requests that exact optional origin permission;
- after grant, the origin is persisted and registration `emenda-enabled-origins` is created or updated with the exact enabled-origin match set;
- the worker pings the current tab and injects the packaged script into the top frame only when no script responds;
- repeated initialization creates no duplicate listener, controller, registry, or overlay;
- zero enabled origins produces zero dynamic content-script registrations;
- revocation first disables the origin and rejects new work, then cancels associated requests and sends versioned `Deactivate` to every live tab on that origin;
- deactivation invalidates revision authority, cancels debounce and inference, removes input and composition listeners and the overlay host, clears source and snapshot registries, and leaves the script inert;
- the registration is updated or removed before the exact optional permission is removed, while other enabled origins remain active;
- an already-injected script cannot resume after registration removal or worker restart unless the origin is enabled again.

Restricted pages, file URLs, PDFs, iframes, and incognito fail closed.

### 7.3 Textarea and safe Apply

On a visible, focused, writable light-DOM `<textarea>`, tests prove committed input, exact debounce, request, suggestion, Dismiss, insertion, deletion, and replacement. Apply verifies current authority plus source, document, opaque snapshot, focus, writability, exact logical text, scalar mapping, and exact original substring before the sole `execCommand("insertText")` mutation.

An exact self-authored input is consumed as `AppliedChange`, updates the returned post-edit snapshot, starts no debounce or inference, preserves focus, and one native Undo restores the exact original text. Any expectation mismatch is ordinary external input and invalidates the prior Apply. Changed source, document, snapshot, text, focus, writability, mapping, and original all refuse mutation; a current refusal enters `Error`.

### 7.4 Contenteditable grammar and mapping

Run the same complete flow for both `contenteditable="true"` and `plaintext-only` using every permitted form: text nodes, `<br>`, transparent inline `<span>`, and simple top-level `<div>` or `<p>` blocks. Test computed whitespace modes `pre`, `pre-wrap`, `break-spaces`, `normal`, `nowrap`, and `pre-line`, accepting each only when exact visible-text mapping is proven.

For every accepted fixture, prove:

- every emitted logical scalar records its complete DOM source span;
- a collapsed logical space maps to the complete underlying whitespace run, with replacement boundaries at the beginning and end of the recorded spans;
- `<br>` emits one LF, each boundary between permitted top-level blocks emits one LF, and no synthetic leading or trailing LF appears;
- element-generated LF scalars have deterministic DOM spans;
- every scalar boundary round-trips and every accepted correction range yields one unique safe DOM replacement span;
- insertion, deletion, replacement, self-authored input consumption, changed-state refusal, focus preservation, and one-step native Undo match textarea behavior.

Explicit refusal fixtures cover mixed inline/block form, nested blocks, nested editing hosts, `contenteditable="false"` islands, hidden descendants, replaced elements, generated visual text, unsupported nodes or whitespace behavior, shadow DOM, rich or virtualized editors, canvas editors, and every ambiguous or non-round-tripping map.

### 7.5 IME, failures, and accessibility

Real event tests prove composition start invalidates immediately, composing input never checks, composition end creates the single committed change, an identical terminal input is deduplicated, and a divergent terminal input is handled as external input.

Presentation tests prove the locked silent and `Error` mappings, including Open Settings for missing configuration. The fixed, unanchored, shadow-root overlay appears only for current suggestions or writer-triggered errors, never steals focus, exposes understandable before/after text, category, explanation, Apply, and Dismiss, and makes stale controls inert. Escape dismisses and Alt+Enter applies only the current suggestion. Accessible names, coherent focus order, visible focus, reduced motion, WCAG 2.2 AA contrast, and non-color meaning all pass.

### 7.6 Confinement inspection

Bundle and runtime inspection prove source references, snapshot references, full document text, page URLs, and DOM data remain in the content script; only bounded context reaches the worker; and no secret, private text, persistent text cache, analytics, telemetry, or remote executable code leaks into storage, logs, fixtures, snapshots, errors, or the bundle.

## 8. V0.1 Conformance Gate

The final gate requires all prior evidence to remain valid for the tested implementation tree and commit, plus:

- the complete deterministic suite and Playwright bundled-Chromium persistent-context suite pass from a clean checkout;
- the production extension build and the implementation's single cross-platform audit command pass;
- dependency, bundle, permission, manifest, registration, and secret/text-leakage inspections match the constitution;
- direct compatibility smoke passes on Chromium or Chrome for Testing 140, with exact browser build and host recorded;
- a manual unpacked-extension smoke passes on current Chrome Stable, including the actual toolbar permission prompt, enablement, inference, Apply, Undo, revocation, and teardown;
- personal-device smokes are recorded separately for Windows Studio with current Chrome, MacBook with current Chrome, and Chromebook with current ChromeOS/Chrome;
- every device record states exact OS/browser versions, tested behaviors, failures, and limitations, without generalizing beyond that environment;
- supported and rejected surface claims match tested fixtures, accessibility and reduced-motion checks pass, and all privacy disclosure text matches the product behavior;
- the tested implementation commit is pushed and its local and remote identities match;
- a later evidence commit records that already-existing tested implementation tree and commit, is pushed and verified, and leaves a clean worktree.

The three browser evidence layers remain distinct: automated bundled Chromium, direct Chrome 140 compatibility, and manual current Chrome Stable. Device records supplement rather than replace them.

After V0.1 Conformance passes, stop. Distribution, native work, commercial work, broader surfaces, and general cross-platform claims require separately versioned objectives.
