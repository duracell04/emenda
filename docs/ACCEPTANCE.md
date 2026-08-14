# Emenda V0.1 Acceptance

> **Frozen acceptance contract, version 2.0.1**

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

The current v2.0.1 objective ends after the Documentation Gate: rewrite, verify, hash, commit, and push the 13 Markdown files, confirm remote identity and a clean worktree, then stop. All implementation gates require a separate future objective.

## 3. Documentation Gate

The gate passes only when:

- the commit is a documentation-only child of v2.0.0 commit `a1a13607867db8e6eb2ea904f6387ba130f22ce7`;
- the tracked Markdown inventory is exactly the 13 paths declared by `PACKAGE-MANIFEST.md`, with no implementation source added;
- all documents identify version 2.0.1, and every freeze-ID occurrence is `emenda-clean-room-v2.0.1-2026-08-14`;
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
- a complete paragraph of at most 1,200 scalars is the context; otherwise context is at most 1,200 scalars, is exactly 1,200 only when sufficient document context exists, divides spare capacity evenly with an odd scalar trailing, clamps at document edges, and backfills from the available side;
- the complete focus is present, and a focus longer than 1,200 scalars fails closed without inference;
- correction offsets map from context-relative to snapshot-relative coordinates exactly once and remain inside the focus.

### 4.3 Validation, failures, and presentation

Semantic validation and mock-provider cases prove:

- clean, empty, nonlinguistic, unsupported-language, over-limit-focus, non-collapsed-selection, and ordinary unsupported-capture outcomes return silently to `Idle`;
- supported `auto` results are accepted, fixed mode accepts only its exact profile or `unsupported`, `unsupported` is accepted only with an empty correction list and returns to `Idle`, and a different supported profile becomes `LanguageMismatch` with no suggestion and `Error`;
- zero or one correction is handled; its only fields are `range.start`, `range.end`, `original`, `replacement`, `category`, and `explanation`;
- insertion, deletion, and replacement validate exact range, focus containment, original substring, replacement, category, explanation, and no-op refusal;
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
- `profileMode` defaults to `auto`;
- public configuration messages contain exactly `hasApiKey`, `profileMode`, and `settingsRevision`;
- settings-change messages are validated and delivered to every live enabled content script, API-key/model/profile changes increment the revision and cancel current requests, and origin changes follow separate lifecycle messages;
- every check carries `settingsRevision`, the worker rejects a stale value before using the key or model, cache resynchronization is possible, and the rejected revision is not retried;
- `settingsRevision` is absent from the OpenRouter payload;
- external model JSON is accepted only through the strict `core/provider-schema/` schema before semantic validation.

### 6.2 OpenRouter transport tests

Tests inspect the exact outbound request and prove:

- the endpoint is `https://openrouter.ai/api/v1/chat/completions`;
- one concrete configured model is present and no `models` fallback array exists;
- `stream` is disabled and strict structured output is requested;
- routing contains `require_parameters: true`, `allow_fallbacks: false`, and `data_collection: "deny"`;
- the body contains only the bounded linguistic payload and never a URL, full document, source or snapshot identity, DOM structure, API key, or Emenda settings revision;
- the response may contain only zero or one correction using exactly `range.start`, `range.end`, `original`, `replacement`, `category`, and `explanation`, plus a supported profile or `unsupported` at result level; extra properties are rejected;
- local Zod validation follows transport parsing, while request identity remains Emenda-authored;
- timeout occurs at eight seconds, response reading stops incrementally above 32 KiB, and cancellation has a typed outcome;
- HTTP, transport, timeout, size, parse, schema, and unsupported outcomes are typed and redacted;
- authorization headers, credentials, raw contexts, and raw response bodies cannot enter logs, snapshots, errors, or telemetry;
- there is no retry, healing, streaming, response cache, telemetry, provider fallback, model substitution, or OpenRouter SDK.

### 6.3 Live provider evidence

Using a dedicated spend-limited key and the configured structured-output model through the production validation path, record UTC time, concrete model, latency, and sanitized outcome for:

- one correction and one clean case for each of `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`;
- supported-profile detection in `auto`;
- fixed-profile mismatch;
- one unsupported-language case.

No live record contains the credential or raw private text.

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
- the production extension build and the single cross-platform `scripts/audit.mjs` command pass;
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
