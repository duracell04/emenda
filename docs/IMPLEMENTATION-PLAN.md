# Emenda V0.1 Implementation Plan

> **Frozen implementation plan, version 2.0.0**

## Execution contract

Complete one browser-only V0.1 objective through small, independently verifiable increments. Each increment follows:

```text
inspect
→ implement one decision
→ run focused checks
→ inspect the diff
→ append factual evidence
→ commit
→ push
→ verify pushed identity
→ continue
```

The constitution remains immutable during implementation. Only `docs/EVIDENCE.md` is updated with factual evidence.

## Canonical implementation sequence

```text
Documentation baseline + Documentation Gate
→ strict-TypeScript domain and schemas
→ TextSurface + MockTextSurface
→ InferenceProvider + MockInferenceProvider
→ controller, scheduler, context, and revision
→ validator + presentation state
→ complete mock product + Mock Product Gate
→ Architecture Gate
→ BrowserTextSurface
→ MV3 worker, options, and overlay
→ OpenRouterProvider + Provider Gate
→ textarea runtime
→ conventional contenteditable runtime
→ Browser Integration + V0.1 Conformance Gate
→ stop
```

Do not reorder this sequence. The increments below elaborate it without creating additional gates.

## 0. Documentation baseline + Documentation Gate

Verify:

- exactly 13 tracked Markdown files and no implementation artifacts;
- every header identifies version 2.0.0;
- freeze ID and supersession statement are correct;
- all local links resolve;
- every canonical-sequence occurrence is byte-identical;
- native-related terms occur only in explicitly deferred contexts;
- 11 immutable Git-blob SHA-256 values match `PACKAGE-MANIFEST.md`;
- `docs/EVIDENCE.md` is an empty template;
- `git diff --check` passes.

After the gate passes, initialize the evidence ledger with the constitution commit and exact validation results.

## 1. strict-TypeScript domain and schemas

Add only the one-package toolchain and core values:

- opaque `RevisionId`, `SourceReference`, `SnapshotReference`, `SuggestionId`;
- immutable `Revision`, `TextRange`, `ObservedChange`, `SurfaceSignal`, `SurfaceSnapshot`, `TextContext`, `Correction`, and request/result values;
- state and typed failures;
- strict Zod schemas for external and message boundaries;
- scalar-offset utilities;
- constants for 600 ms, 1,200 scalars, eight seconds, and 32 KiB.

Test Georgian, Russian, combining marks, emoji, bounds, and opaque-reference behavior.

## 2. TextSurface + MockTextSurface

Implement the locked `TextSurface` port and a deterministic mock that can:

- emit committed and composition signals;
- return exact snapshots;
- simulate source, snapshot, text, focus, writability, and mapping changes;
- record replacement requests and return typed outcomes;
- prove Apply and refusal without browser types.

## 3. InferenceProvider + MockInferenceProvider

Implement the locked cancelable provider port and a deterministic mock that can:

- resolve clean and one-correction results;
- resolve malformed and typed failures;
- delay results under fake-clock control;
- record one request per revision;
- observe cancellation without making it authoritative.

## 4. controller, scheduler, context, and revision

Implement:

- synchronous revision reservation;
- immediate invalidation and best-effort cancellation;
- one trailing-edge debounce at exactly 600 ms;
- composition invalidation and `compositionend` eligibility;
- post-debounce capture;
- sentence focus;
- paragraph context or evenly balanced clamped window;
- exact 1,200-scalar limit;
- empty, whitespace-only, and nonlinguistic silence;
- one provider request for each current eligible revision;
- authoritative stale silence.

Use fake clocks for every timing assertion.

## 5. validator + presentation state

Implement:

- exact strict-schema acceptance;
- zero-or-one correction behavior;
- supported-profile and fail-closed language rules;
- insertion, deletion, and replacement validation;
- range, focus, original, no-op, category, and explanation checks;
- `Idle | Debouncing | Checking | Suggestion | Applying | Error`;
- current `SuggestionId` capabilities;
- Apply and Dismiss commands;
- no persistent clean state.

## 6. complete mock product + Mock Product Gate

Compose the core with both mocks and prove:

```text
change
→ revision
→ debounce
→ capture
→ context
→ check
→ validation
→ suggestion
→ Apply or Dismiss
→ exact replacement or no mutation
```

Cover cancellation races, stale results, stale failures, stale Apply, changed sources, mismatch refusals, composition, and complete state transitions.

## 7. Architecture Gate

Before browser code:

- compile `core/` with a dedicated configuration excluding DOM, Chrome, Node, React, and extension types;
- enforce `extension → core` import direction;
- verify the runtime and development dependency allowlists;
- verify no browser mechanism entered shared values or ports;
- record the gate evidence.

## 8. BrowserTextSurface

Implement leaf behavior for eligible top-level light-DOM surfaces:

- event and composition observation;
- exact textarea logical-text and selection mapping;
- conventional contenteditable logical-text and Range mapping;
- private source registry and opaque snapshots;
- capture refusal for every excluded surface;
- complete pre-apply verification;
- runtime-gated `execCommand("insertText")` mutation;
- no alternative mutation strategy.

Unit-test mapping utilities before runtime integration.

## 9. MV3 worker, options, and overlay

Implement:

- Chrome 102+ manifest with only locked permissions;
- incognito disabled;
- exact-origin toolbar activation;
- persistent optional permissions and one dynamic registration;
- trusted-context `chrome.storage.local` access;
- write-only API-key and concrete-model settings;
- content-script `hasApiKey` status only;
- versioned strict messages;
- fixed, unanchored shadow-root overlay;
- display-safe state, Apply, Dismiss, Escape, Alt+Enter;
- accessible names, visible focus, reduced motion, and WCAG 2.2 AA styles.

Bundle all executable code locally with esbuild.

## 10. OpenRouterProvider + Provider Gate

Implement:

- the fixed chat-completions endpoint;
- minimal non-streaming payload;
- `provider.require_parameters: true`;
- strict JSON Schema response formatting;
- local Zod validation;
- adapter-copied revision identity;
- eight-second timeout;
- incremental 32 KiB body limit;
- cancellation;
- typed, redacted failures;
- zero retry, healing, fallback, cache, telemetry, or analytics.

Run deterministic provider tests. Then use a dedicated spend-limited key for one correction and one clean case for each of `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`, plus unsupported-language evidence. Record the concrete model, UTC time, latency, sanitized outcomes, and cost-relevant facts.

## 11. textarea runtime

Use Playwright's [persistent-Chromium extension setup](https://playwright.dev/docs/chrome-extensions) to prove:

- permission grant and revocation;
- capture after committed input;
- exact debounce and stale behavior;
- Apply and Dismiss;
- changed-source refusal;
- IME handling;
- focus preservation;
- one native Undo restoring exact original text.

## 12. conventional contenteditable runtime

Repeat the runtime contract for simple `contenteditable="true"` and `plaintext-only` fixtures. Prove lossless mapping, insertion, deletion, replacement, refusal for ambiguous structures, and one-step Undo.

## 13. Browser Integration + V0.1 Conformance Gate

Run the full unpacked build in persistent Chromium and verify:

- all supported surface flows;
- every exclusion fails closed;
- worker restart and message validation behavior;
- permission and trusted-storage behavior;
- stale silence and current-authority safety;
- overlay focus, keyboard, accessible names, visible focus, reduced motion, and contrast;
- no secret or raw-text leakage;
- current Chrome Stable smoke;
- final dependency, permission, and bundle inventories.

Append exact evidence, commit, push, verify local and remote commit identity, confirm a clean worktree, and stop.

## Deferred work

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, packaging, signing, Chrome Web Store publication, release automation, native placeholders, and cross-OS runtime claims are not implementation increments in V0.1.
