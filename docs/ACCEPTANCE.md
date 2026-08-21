# Emenda V0.1 Acceptance

> **Frozen acceptance contract, version 2.0.3**

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

An evidence commit records an already-existing implementation commit that was actually tested. Later success does not erase an earlier failure; record failure and recovery separately. Never record credentials, authorization headers, raw private text, page URLs, tab/frame/document metadata, full provider bodies, source identities, or DOM data.

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

The current v2.0.3 objective ends after the Documentation Gate: preserve v2.0.2, rewrite, verify, hash, commit, and push its direct-child 13-file Markdown freeze, confirm remote identity and a clean worktree, then stop. All implementation gates require a separate future objective.

## 3. Documentation Gate

The gate passes only when:

- the commit is a documentation-only direct child of v2.0.2 commit `6a4ddc65fa9067f94023f87aebe48840e1b88bc2`, while v2.0.1 commit `d70b277998a23663ee6befc77dd6bb0da50ebcca` remains in its ancestry;
- the tracked Markdown inventory is exactly the 13 paths declared by `PACKAGE-MANIFEST.md`, with no implementation source added;
- all documents identify version 2.0.3, and every freeze-ID occurrence is `emenda-clean-room-v2.0.3-2026-08-21`;
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
- ordinary input is eligible only from a same-source, same-generation trusted `beforeinput`/`input` ticket that binds exact pre/post tuples and passes the complete foreground/exposed textarea predicate at both events; every later `beforeinput` first invalidates the prior ticket, the first input clears it, and each private expiry callback clears only its own still-current opaque ticket while listener microtasks do not, and an unpaired or untrusted change on that otherwise supported textarea may refresh the baseline and invalidate stale authority without reserving a revision or requesting inference, while the exact registered Apply input is the sole bypass;
- no request starts at 599 ms and exactly one eligible request starts at 600 ms; later committed input replaces the timer;
- each eligible revision produces at most one request, and current revision authority wins every completion and cancellation race;
- stale results, stale failures, stale settings revisions, and stale Apply or Dismiss commands do not change presentation or text;
- API-key, model, and profile changes increment `settingsRevision`, cancel active inference, invalidate visible suggestions and obsolete errors, and leave processing paused until the next committed input; completing configuration returns to `Idle` without retry, and origin changes do not increment it;
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
- context and focus obey the canonical scalar limits in [`SPEC.md`](../SPEC.md#3-v01-runtime-and-limits), with the complete focus present, deterministic surrounding-context allocation, and silent refusal above the focus limit;
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

The deterministic composition proves the full loop from committed input through revision, debounce, cached settings, capture, context, inference, validation, suggestion, Apply or Dismiss, and final state. Mocks cover trusted paired keyboard/paste input and IME, a ticket surviving a `beforeinput`-queued microtask, untrusted, unpaired, post-expiry-callback, wrong-source, and wrong-generation input, clean and correction results, delayed stale completion, timeout, cancellation race, source and snapshot changes, changed text or selection, lost window or element focus, hidden or midpoint-covered document state, readonly state, mapping refusal, off-caret insertion/deletion/replacement, exact replacement, and self-authored replacement acknowledgement. Every provider success or failure rechecks the captured source, document, snapshot value/selection, foreground focus, and exposure before presentation; a completion-time programmatic change without an input event is silent and refreshes only the supported local baseline.

An exact expected self-mutation updates the post-edit baseline, emits no new observed change, advances authority without inference, and returns to `Idle`. A mismatch is external, refreshes the supported baseline, invalidates old Apply authority, and reserves a revision only when it independently has eligible paired provenance.

## 5. Architecture Gate

This gate verifies only the architecture that exists before browser integration:

- `core/` compiles under strict TypeScript while DOM, Chrome, Node, React, and extension types are unavailable;
- domain values, text policy, reducer, context, validation, and semantic ports contain no Zod or runtime mechanisms;
- a repository-wide import scan proves Zod appears only in `core/provider-schema/`, `extension/protocol/`, and the worker-owned trusted-settings boundary, with no re-export or indirect wrapper that broadens those locations;
- imports point from extension composition and adapters toward core, and core never imports `extension/`;
- public core declarations expose semantic capabilities and opaque references rather than browser, timer, transport, storage, or UI objects;
- the repository remains one npm package with exact direct dependency versions, a committed npm lockfile, and a clean `npm ci` install under recorded Node and npm versions;
- Zod is the only direct runtime dependency, and development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, and Chrome/Node types;
- no React, UI framework, OpenRouter SDK, monorepo tool, backend, database, code generation, native scaffold, or deferred-runtime placeholder exists.

Runtime-message behavior and external-schema enforcement are not Architecture Gate criteria. Manifest, permissions, registrations, storage isolation, DOM runtime behavior, and overlay accessibility are not Architecture Gate criteria.

## 6. Provider Gate

### 6.1 Protocol and trusted-boundary tests

Deterministic worker tests prove:

- one-shot runtime messages use `protocolVersion: 1` in discriminated strict Zod envelopes, reject unknown versions, types, properties, malformed payloads, and disallowed senders, and use no long-lived Port;
- content-origin operations require the complete top-level HTTP(S) sender class, options reads/saves/revocation require the exact packaged options-page URL and extension origin, and content-to-settings, options-to-content, altered extension-page URL, and all other cross-class combinations fail closed;
- trusted settings accept only the exact version-1 types, scalar bounds, base-model-shaped grammar, safe integer, and sorted canonical origins in [`SPEC.md`](../SPEC.md#5-settings-authority), rejecting the `openrouter` namespace, `~` alias syntax, arrays, and every colon-suffixed variant while making no catalog-existence claim, with no guessed migration;
- new settings use null API key and model, `profileMode: auto`, revision zero, and no origins; a complete valid save derives `isConfigured: true` and an incomplete or corrupt record fails closed;
- the options read view contains exactly the nonsecret fields, and revision-aware keep/replace/clear saves reject stale revisions, never disclose the existing key, preserve current worker-owned origins, and increment only for a real API-key/model/profile change;
- public configuration messages contain exactly `isConfigured` and `settingsRevision`; the content protocol carries no profile, and provider input always derives it from trusted settings;
- settings-change messages are validated and delivered to every live enabled content script, API-key/model/profile changes increment the revision and cancel current requests, and origin changes follow separate lifecycle messages;
- every check carries `settingsRevision`; a stale value returns current validated public configuration, the originating controller replaces its cache, and the rejected revision is not retried;
- `settingsRevision` and every Chrome-supplied sender field are absent from the OpenRouter payload;
- external model JSON is accepted only through the strict canonical schema before semantic derivation;
- the provider-authored corrected focus cannot enter the worker-to-content envelope, whose trusted derived correction shape remains unchanged.

### 6.2 OpenRouter transport tests

Tests inspect the exact outbound request and prove:

- the endpoint, authored headers, no-credentials/no-cache/no-redirect/no-referrer fetch controls, canonical prompt, two-message order, exact JSON-stringified user content, JSON Schema name and shape, explicit disabled-plugin entries, and every request field match [`SPEC.md`](../SPEC.md#9-provider-request);
- one required trusted base model-shaped ID is sent, with no compiled default, `openrouter` namespace, variant suffix, `models` array, or application-level substitution;
- the model-facing user payload is exactly the split bounded input and excludes URL, any separate or unbounded full-document field, source or snapshot identity, DOM structure, API key, settings metadata, and browser sender metadata; tests acknowledge that bounded context may equal all text of a short document, and embedded document instructions cannot alter the system instruction or request shape;
- the strict structured-output schema and local semantic pass reject missing or extra properties, more than one correction, malformed Unicode, CR, an over-limit explanation or corrected focus, and a whitespace-only explanation;
- `max_completion_tokens: 8192`, `reasoning.exclude: true`, `temperature: 0`, routing, one-request behavior, zero application retries, 15-second full-processing deadline, 32-KiB incremental response bound, and cancellation match the canonical contract; reasoning may consume that budget but no trace enters the bounded response;
- 2xx success accepts only parsed `application/json`, fatal UTF-8, and the canonical outer projection: no top-level or choice error/refusal, one index-0 choice, `finish_reason: stop`, assistant string content, and a returned model exactly equal to the trusted requested model before strict content parsing and semantic derivation;
- HTTP, media-type, decoding, transport, redirect, timeout, size, model-identity, envelope, finish, parse, schema, semantic, and unsupported outcomes are typed and redacted;
- authorization headers, credentials, raw contexts, and raw response bodies cannot enter logs, snapshots, errors, or telemetry;
- there is no enabled plugin, tool, server tool, healing, prompt transform, streaming, response cache, telemetry, OpenRouter SDK, application-level retry, `models`-array failover, or application-level model substitution;
- within-request provider fallback is enabled only across eligible endpoints for the same model and is not represented as a guarantee of immediate fallback or completion inside the deadline;
- a successful response's selected model is available to the live evidence path, while pre-response failure records it as unavailable.

### 6.3 Live provider evidence

Run the following corpus through the production parsing and derivation path using one writer-supplied documented direct-model ID and an OpenRouter key whose account/workspace has no enforced plugin policy that prevents request-level disabling. Record that precondition and requested model in the run metadata; the run itself, not settings syntax, qualifies its observed compatibility, and every returned model must equal the requested ID. In every case, `before` and `after` are empty and the Focus column is the complete focus. Calls are strictly sequential: a case does not start until the preceding case terminates. No case is retried or replaced within a run.

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

The Provider Gate requires 100% of its deterministic assertions and one complete live qualification run with `15/15` successes. A case succeeds only when it completes inside the canonical deadline, passes the strict schema and local semantic derivation, matches the table's required result, and is linguistically correct. This observed qualification is not a future reliability guarantee. Any failure remains factual and leaves the gate incomplete. After an implementation, configuration, or external-service change, a new complete 15-case attempt may be recorded as separate recovery evidence; individual failed cases are never retried in place. Missing credentials, exhausted quota, or an interrupted corpus also leaves the gate incomplete.

For each case record only the case identifier, selected model or `unavailable`, complete request latency, outcome, failure reason when any, and linguistic correctness. General evidence metadata from Section 1 still applies. Do not calculate percentiles, distributions, or a stochastic pass percentage, and do not run a concurrent stress corpus as part of this gate. No live record contains the credential or raw private text.

## 7. Browser Integration Gate

Automated extension tests run in Playwright's [bundled Chromium persistent context](https://playwright.dev/docs/chrome-extensions) against the production unpacked build. They instrument extension handlers and programmatic Chrome events but do not claim to operate browser toolbar or permission UI; actual user-gesture cases are owned by the headed manual tests in Section 8.

### 7.1 Manifest, storage, and configuration

Runtime tests prove:

- the Manifest V3 package declares `minimum_chrome_version` as `"140"`, grants OpenRouter only `https://openrouter.ai:443/*`, uses only the other locked permissions, has no static all-sites content script or `<all_urls>` grant, disables incognito, and bundles executable code locally;
- action, message, and permission listeners register synchronously at worker module evaluation, and every handler awaits one shared sticky initialization promise;
- worker initialization awaits `chrome.storage.local.setAccessLevel({ accessLevel: "TRUSTED_CONTEXTS" })` before any settings read or write and fails closed when the method is unavailable or rejected;
- the Chrome-140-compatible `runtime.onMessage` listener is not `async`, starts asynchronous dispatch, responds on every handled path through `sendResponse`, and returns literal `true` synchronously, including after a cold worker restart;
- a content script cannot read `chrome.storage.local` and cannot receive its change events;
- only the worker reads or writes the API key, model, and full settings record; the options page communicates through the worker;
- fresh, valid, corrupt, extra-property, and unknown-schema settings records follow the exact version-1 schema and fail-closed behavior;
- content initialization receives and caches only `isConfigured` and `settingsRevision`, then updates that cache only through validated messages rather than fetching before each capture;
- worker restart preserves only the intended durable settings and origin state and re-establishes storage isolation before reconciling them.

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
- startup reconciles corrupt or interrupted durable state, current exact grants, and the fixed registration before content or provider work opens; external `permissions.onAdded` removes unowned or broader grants without racing Emenda's pending exact prompt, external `permissions.onRemoved` performs disablement and cancellation, and every inference and Apply authorization repeats the current permission check;
- revocation first disables the origin and rejects new checks and Apply authorizations, then cancels associated requests and sends versioned origin-bound `Deactivate` to known document IDs;
- after an external removal or incomplete known-document set, unfiltered `tabs.query({})` drives a best-effort frame-0 broadcast without reading tab URLs; each receiver ignores a control message whose origin differs from its current nonopaque `location.origin`, including a forced cross-origin navigation race;
- deactivation invalidates revision authority, cancels debounce and inference, removes input and composition listeners and the overlay host, clears source and snapshot registries, and leaves the script inert;
- the registration is updated or removed before the exact optional permission is removed, while other enabled origins remain active;
- post-grant failure rolls settings, registration, and permission back best-effort, while startup reconciliation completes an interrupted rollback;
- overlapping settings save, enable, revoke, external-addition/removal, and startup work for two origins is FIFO-serialized from freshly read state and converges without a lost setting or origin, overbroad permission, or registration drift;
- `pagehide` tears authority and page-owned state down, every BFCache `pageshow` reauthorizes before resuming, and a document that starts prerendering defers handshake, observation, composition, and UI until one `prerenderingchange` reauthorization;
- an already-injected script cannot check, Apply, or resume after registration removal, permission removal, failed teardown delivery, or worker restart unless the current origin is enabled and reauthorized again.

Restricted pages, file URLs, PDFs, iframes, and incognito fail closed.

### 7.3 Textarea and safe Apply

On a visible, window-focused, active, writable, midpoint-exposed, sequentially keyboard-focusable light-DOM `<textarea>`, tests prove committed input, exact debounce, request, suggestion, Dismiss, and off-caret insertion, deletion, and replacement. Real keyboard, paste, and IME fixtures produce the required trusted `beforeinput`/`input` tickets and exact pre/post tuples even when a `beforeinput` listener queues a microtask before browser mutation. Matching delayed or coalesced selection notification after real keyboard or paste input preserves the one debounce and request; an intervening moved or mismatching selection cancels them. Synthetic dispatch, value-only changes, a ticket used after its private expiry callback, wrong-source/generation input, and page-invoked `execCommand` during a click/keydown or outside a gesture when no provenance ticket is outstanding create no revision or request. A stale expiry callback cannot clear a newer opaque ticket. Tests separately record that page work nested in a genuine trusted `beforeinput` or queued ahead of its expiry callback can consume that one-use ticket and is an explicitly bounded enabled-origin limitation. Trusted paired IME generations qualify and may use an in-bounds losslessly mapped noncollapsed candidate range only during their baseline-only intermediate phase; untrusted or unpaired start/input/end paths do not. Ordinary capture and final verification require a visible document, `document.hasFocus()`, the exact active textarea, its connected document, exact value, exact collapsed selection, and the same exposure predicate.

Apply begins only from the current trusted Apply control after the deliberate approval-UI focus handoff, then obtains one worker authorization for the current `settingsRevision` containing no page text or surface identity. On success, focus must remain in that control; the adapter restores the captured textarea and caret, verifies current authority plus source, document, foreground focus, opaque snapshot, writability, exact logical text, scalar mapping, captured selection, and exact original substring, then selects the exact mapped correction range inside one scoped synchronous internal phase. Tests prove selection observation is suspended only for that call and immediate readback, the unchanged value and original substring are rechecked, no queued selection event is awaited or required, and the sole mutation is `document.execCommand("insertText", false, replacement)`.

Success requires a `true` return, the exact synchronous self-authored input, and exact expected post-state. It is consumed as `AppliedChange`, updates the returned post-edit text and selection baseline, starts no debounce or inference, preserves textarea focus, and one native Undo restores the exact original text. Delayed and coalesced `select`/`selectionchange` fixtures are ignored only when the current source and selection match that baseline; a writer change invalidates normally. A changed source, document, snapshot, value, captured or target selection, foreground focus, writability, mapping, original, or worker authority refuses text mutation and enters `Error`; an unchanged failure best-effort restores and records the captured caret. A worker restart with a visible suggestion first completes sticky initialization and then applies the same current authorization predicates; initialization failure refuses. Forced teardown-delivery failure, external permission removal, hidden tab, window blur, page capture-phase change, `false`, throw, missing input, mismatching input, and unexpected post-state prove fail-closed behavior. An unexpected changed state refreshes baseline/authority and prevents a false no-mutation claim, but reserves and debounces only with independent eligible paired provenance; no fallback mutation runs.

### 7.4 Refused surfaces and mapping boundary

Textarea fixtures prove exact value capture and lossless bidirectional conversion at every Unicode-scalar and UTF-16 boundary, including ASCII, combining sequences, emoji, and supplementary-plane scalars. Lone surrogates, raw CR, a boundary inside a surrogate pair, an ordinary non-collapsed selection, and every non-round-tripping correction range fail before inference or mutation. An eligible intermediate IME candidate range is baseline-only and cannot reach either operation.

Explicit refusal fixtures prove that inputs, every contenteditable form, iframes, shadow-DOM editors, rich, virtualized, canvas, and Google Docs-style editors, hidden or offscreen textareas, readonly, disabled, inert, non-`tabIndex === 0`, disconnected, background-document, and ambiguous surfaces have no text read or captured and never infer or mutate. Baseline-only recapture is confined to an otherwise supported textarea whose provenance pairing failed. The shared exposure fixtures cover an exposed midpoint, a page cover at input, a cover added during debounce, a cover added before Apply, and the current Emenda host over the point: intersect the client rect with the layout viewport, hit-test its midpoint, skip only that host, and require the textarea as the first remaining element. Record that DOM hit-testing does not prove compositor-only or `pointer-events: none` visual occlusion. Bundle inspection proves there is no contenteditable or DOM-tree logical-text mapper.

### 7.5 IME, failures, and accessibility

Real event tests prove every composition start invalidates immediately and binds its collapsed pre-composition tuple; only a trusted start on a qualifying textarea creates an eligible generation; paired composing input survives listener-queued microtasks, while an untrusted or unpaired change, input after its ticket's expiry callback, an untrusted end, or a failed end exposure check makes it baseline-only or disqualifies the generation as specified; and only a trusted qualifying end at a collapsed caret after paired changes and a real terminal text change creates the single committed change and rebinds the terminal baseline to its new revision. A Chrome fixture covers a start at `[3,3]`, noncollapsed intermediate candidate ranges `[3,5]` and `[4,6]`, and a final `[6,6]`: the intermediate updates create zero requests, the end creates exactly one committed change, and its queued matching selection notification drains without cancelling the one debounce or request. Cancelled and no-op IME generations create none. A terminal pair is deduplicated only when text, source, and selection match within that generation; each mismatch is external input and must independently qualify. The exact registered self-authored Apply input succeeds without a `beforeinput` ticket and no other event can use that bypass.

Presentation tests prove the locked silent and `Error` mappings, including Open Settings for missing configuration. A suggestion presents the complete original and reconstructed corrected focus, marks exactly the one changed hunk, and separately shows category, explanation, Apply, and suggestion Dismiss. `[empty]`, changed whitespace, control, format, combining, and bidirectional-safety fixtures prove deterministic visible markers and bidi isolation. A content error presents only its redacted message, current error Dismiss, and Open Settings when applicable; suggestion and error capabilities cannot clear or act on each other.

The fixed, unanchored host follows the current textarea in sequential DOM order, owns a closed shadow root, and never autofocuses. Tests tab from the textarea into the first control and among all current controls, activate native buttons by pointer and ordinary Enter/Space, then prove Apply and either Dismiss variant best-effort restore safe unchanged textarea focus and selection. V0.1 registers no page-level custom shortcut. Only a trusted current-control event acts: synthetic page clicks or keys, page-focused input, stale controls, and controls or hosts hidden, moved, DOM-hit-test-covered, made transparent, or disconnected before target handling do nothing. Closed-root and document hit tests must agree for pointer activation, accepted control events do not bubble into page handling, and page capture-phase changes are caught by final verification. Browser evidence records that compositor-only and `pointer-events: none` covers remain outside this proof and are an enabled-origin trust limitation.

Page and model strings containing HTML, Markdown, URLs, event attributes, or script-shaped text render literally through text nodes or `textContent` and create no markup, link, or execution. Each new suggestion or content error emits exactly one polite notification; debounce and checking emit none. Accessible names, coherent focus order, visible focus, reduced motion, WCAG 2.2 AA contrast, and non-color meaning all pass. Toolbar names are only Enable or Reactivate; incomplete configuration opens Settings after successful activation, and site-access text accurately states the bounded trusted-input piggyback limitation. Action badge/title and options tests separately cover activation and revocation-command errors.

### 7.6 Confinement inspection

Bundle and runtime inspection prove Emenda-authored messages omit page URLs, tab/frame/document metadata, any separate or unbounded full-document field, source references, snapshot references, and DOM data; the only page text copied from content to worker is the bounded context, which may equal all text of a short document. Request identity, focus range, and `settingsRevision` also cross only as the nontext protocol authority fields declared by the specification and never enter provider input. Chrome-supplied sender metadata reaches the worker only as ephemeral authorization input: only the required fields are inspected, and none is persisted, logged, copied into errors, or forwarded to OpenRouter. No credential, raw private text, persistent text cache, analytics, telemetry, or remote executable code leaks into storage, logs, fixtures, snapshots, errors, or the bundle.

## 8. V0.1 Conformance Gate

The final gate requires all prior evidence to remain valid for the tested implementation tree and commit, plus:

- a clean checkout installs the committed dependency graph with `npm ci` under recorded Node and npm versions, then the complete deterministic suite and Playwright bundled-Chromium persistent-context suite pass;
- the production extension build and the implementation's single cross-platform audit command pass;
- dependency, bundle, permission, manifest, registration, and secret/text-leakage inspections match the constitution;
- a manual headed compatibility smoke passes on Chromium or Chrome for Testing 140, with exact browser build and host recorded, and covers actual toolbar grant and denial, external site-access revocation, explicit-port permission round trips, storage-event isolation, synchronous message response, dynamic registration/restart, sender lifecycle, Apply insertion/deletion/replacement, and one-step Undo;
- a manual unpacked-extension smoke passes on current Chrome Stable, including actual toolbar grant and denial, enablement, inference, Apply, Undo, external site-access revocation, and teardown;
- personal-device smokes are recorded separately for Windows Studio with current Chrome, MacBook with current Chrome, and Chromebook with current ChromeOS/Chrome;
- every device record states exact OS/browser versions, tested behaviors, failures, and limitations, without generalizing beyond that environment;
- supported and rejected surface claims match tested fixtures, accessibility and reduced-motion checks pass, and all privacy disclosure text matches the product behavior;
- trusted-origin, ticket-piggyback, quota, and possible provider-charge disclosure matches the enabled-site boundary, writer-owned key, and configured model behavior;
- the tested implementation commit is pushed and its local and remote identities match;
- a later evidence commit records that already-existing tested implementation tree and commit, is pushed and verified, and leaves a clean worktree.

The three browser evidence layers remain distinct: automated bundled Chromium, direct Chrome 140 compatibility, and manual current Chrome Stable. Device records supplement rather than replace them.

After V0.1 Conformance passes, stop. Distribution, native work, commercial work, broader surfaces, and general cross-platform claims require separately versioned objectives.
