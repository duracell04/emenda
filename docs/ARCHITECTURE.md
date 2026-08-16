# Emenda V0.1 Architecture

> **Frozen architecture, version 2.0.2**

## 1. Authority and objective boundary

[`SPEC.md`](../SPEC.md) defines product behavior. This document defines ownership, boundaries, import direction, and runtime data flow. [`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md) defines build order.

The current objective preserves the v2.0.1 freeze at `d70b277998a23663ee6befc77dd6bb0da50ebcca` and ends when its direct-child v2.0.2 Markdown constitution is rewritten, verified, hashed, committed, and pushed. Product implementation requires a separate future objective.

## 2. System shape

Emenda V0.1 is one npm package with two architectural regions:

```text
core/                         strict TypeScript product semantics
extension/                    Chromium Manifest V3 mechanisms
```

The active composition is:

```text
content script
  unified state machine + effect runner
  BrowserTextSurface
  shadow-root overlay
        |
        | validated, versioned messages
        v
service worker
  permissions and origin lifecycle
  trusted settings
  cancellation
  OpenRouter transport
        ^
        |
options page
```

There is one `BrowserTextSurface` implementation supporting both `<textarea>` and the constrained contenteditable grammar. There is no second controller or state machine for either surface class.

## 3. Ownership

| Concern | Owner | Boundary rule |
| --- | --- | --- |
| Domain values, deterministic text policy, reducer, context, scalar correction derivation, validation, and semantic ports | `core/` | Pure TypeScript; no DOM, Chrome, Node, React, or extension types; no Zod |
| Model-facing input and model-authored result schemas | `core/provider-schema/` | Zod is permitted only for this external model boundary |
| Runtime message schemas | `extension/protocol/` | Versioned, discriminated, strict Zod schemas |
| Controller instance, revision lifetime, cached public configuration, source registries, and presentation | content script | Raw source identity and DOM data remain here |
| Capture, scalar-to-DOM mapping, and mutation safety | `BrowserTextSurface` in `extension/content/` | Browser types are confined to the adapter |
| Trusted settings schema, storage, permissions, origin lifecycle, request cancellation, and OpenRouter traffic | service worker | Secrets and the configured model never enter content scripts |
| Settings interaction | options page through the worker | The options page never accesses trusted storage directly |
| Visible suggestion and error UI | content-script shadow overlay | It renders state and emits semantic commands; it does not own authority |

Source and snapshot references are opaque core values backed by content-script-private registries. They are never serialized to the worker, provider, logs, or durable storage.

## 4. Core state and effects

One pure reducer owns the complete product state:

```text
Idle | Debouncing | Checking | Suggestion | Applying | Error
```

It controls revision reservation, the 600 ms trailing debounce, request authority, validation, suggestions, Apply, Dismiss, and failure transitions. Inputs are semantic events; outputs are declarative effects. Effect handlers perform timers, inference, messaging, storage interaction, and DOM operations and return typed events to the reducer.

Each eligible committed change reserves a `RevisionId` synchronously. A newer revision cancels older work best-effort and is always authoritative. Stale results, failures, and commands cannot change presentation or text.

The semantic ports describe capture, conditional replacement, cancelable inference, and deterministic scheduling. They expose capabilities and typed outcomes, not browser or transport mechanisms. Deterministic mocks implement the same ports for the complete simulated product.

## 5. Import and dependency direction

Imports point toward product semantics:

```text
extension composition and adapters
              |
              v
        core semantic ports
              |
              v
       core domain and policy
```

`core/` never imports `extension/`. Model-schema code may depend on Zod and core domain definitions, but domain, policy, ports, and state do not depend on model-schema parsing. Protocol and worker schemas remain outside core.

Zod is the only direct runtime dependency. Development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, and Chrome/Node types. The package has no framework, backend, database, remote executable code, native placeholder, or monorepo machinery.

## 6. Trusted configuration flow

The worker owns these values in `chrome.storage.local`:

```text
schemaVersion
apiKey
model
profileMode
settingsRevision
enabledOrigins
```

`profileMode` defaults to `auto`.

The model setting uses the default and advanced concrete-model override defined in [`SPEC.md`](../SPEC.md#9-provider-request). Existing valid overrides remain trusted settings.

At worker initialization, it must await:

```text
chrome.storage.local.setAccessLevel({ accessLevel: "TRUSTED_CONTEXTS" })
```

No trusted-settings read or write may occur first. Unavailability or rejection leaves configuration and provider work closed. Content scripts must be unable both to read the storage area and to receive its change events.

The options page reads and changes settings through validated worker messages. On content-script initialization, the worker returns only:

```text
hasApiKey
profileMode
settingsRevision
```

The content script caches that public configuration. Validated settings-change messages to every live enabled content script replace the cache; capture does not fetch configuration again. API-key, model, and profile changes increment `settingsRevision`, cancel active inference, and invalidate visible suggestions. Origin changes use the activation and revocation lifecycle instead.

Every content-to-worker check carries the cached `settingsRevision`. Before using its private API key and model, the worker compares that value with current trusted settings. A stale request is rejected and the public cache is resynchronized; the rejected revision is not retried. `settingsRevision` is Emenda-internal and is never sent to OpenRouter.

## 7. Check and presentation flow

For an eligible revision:

1. The content script captures an opaque snapshot through `BrowserTextSurface` after debounce.
2. Pure core policy selects the deterministic focus and bounded context under the canonical limits in [`SPEC.md`](../SPEC.md#3-v01-runtime-and-limits).
3. The content script sends only that bounded logical context, its exact focus range, selected profile, request identity, and `settingsRevision` to the worker.
4. The worker revalidates origin, configuration revision, and message shape, splits the context into the model-facing `before`, `focus`, and `after` fields, then uses its private model and credential for one OpenRouter request.
5. The worker strictly validates the external result, invokes pure scalar derivation, and returns only the trusted derived correction or typed failure in a versioned outcome. Model-authored `correctedFocus` never crosses into the content script.
6. The reducer accepts the outcome only for the current revision and either returns to `Idle`, presents one suggestion, or enters `Error` according to the specification.

The page URL, full document, source identity, snapshot identity, DOM structure, API key, and model identifier do not cross boundaries that do not own them.

## 8. Revision and mutation authority

Apply is split deliberately:

- The controller verifies the current `SuggestionId`, current `RevisionId`, and that the suggestion belongs to that revision.
- `BrowserTextSurface` verifies the same connected source and document, opaque snapshot, focused writable surface, exact expected logical text, lossless mapping, and exact original substring.

The authorized replacement request contains the opaque source and snapshot references, expected logical text, snapshot-relative scalar range, original, and replacement. The surface does not query or reproduce reducer revision policy.

Immediately before the sole mutation leaf, runtime-gated `document.execCommand("insertText")`, the surface registers a one-use expected self-mutation containing the source, pre-edit text, post-edit text, target range, and replacement. An exact matching input becomes `AppliedChange`: it updates the logical-text and snapshot baseline, emits no `ObservedChange`, advances reducer authority without inference, and returns to `Idle`. A mismatch is an ordinary external committed change and invalidates the Apply result. No direct value assignment, DOM rewrite, clipboard operation, simulated input, fuzzy matching, or unique-match recovery is allowed.

Composition handling is centralized at the adapter/controller boundary. `compositionstart` invalidates current authority immediately; composing input only refreshes the adapter baseline; `compositionend` emits the sole committed change. An identical later terminal input is suppressed within that composition generation, while a divergent input is external and reserves a revision normally.

## 9. Browser text mapping

`BrowserTextSurface` owns exact logical-text construction and bidirectional mapping. It accepts only a visible, focused, writable, light-DOM `<textarea>` or the bounded contenteditable grammar in the specification. Unsupported or ambiguous surfaces fail closed before inference or mutation.

For contenteditable, the mapper records the DOM source span of every emitted logical scalar. In collapsed whitespace modes, one emitted logical space records the complete underlying whitespace run, and replacement boundaries map to the beginning and end of the recorded spans. `<br>` emits one logical LF; a boundary between two permitted top-level blocks emits one logical LF; no synthetic leading or trailing LF is added. Element-generated newlines also receive deterministic DOM spans.

An accepted surface must round-trip every scalar boundary and yield one unique safe DOM replacement span for every accepted correction range. The adapter rejects any structure, whitespace behavior, or boundary that cannot satisfy both conditions.

## 10. Origin lifecycle

V0.1 requires Chrome 140 or newer and uses one dynamic registration:

```text
emenda-enabled-origins
```

Enablement validates a top-level HTTP(S) tab, requests its exact optional origin permission, persists the origin, then creates or updates that registration. The worker pings the current tab and injects the packaged content script into the top frame only if it does not respond. Content-script initialization is idempotent. An empty `enabledOrigins` set always means zero dynamic registrations.

Revocation first marks the origin disabled so new messages fail. It then cancels associated worker requests, sends versioned `Deactivate` messages to live tabs on the origin, and requires each content script to invalidate its revision, cancel debounce and inference, detach input and composition listeners, remove its overlay host, clear source and snapshot registries, and become inert. The worker then updates or removes the registration and removes the optional permission. Registration removal alone is never treated as teardown of already-injected code.

## 11. Provider boundary

The worker alone owns the endpoint, credential, trusted model setting, routing, cancellation, response reading, external Zod parsing, and conversion from the external result into the trusted derived correction. It implements the canonical model-facing and provider contracts in [`SPEC.md`](../SPEC.md#8-model-facing-contract-and-local-derivation) and [`SPEC.md`](../SPEC.md#9-provider-request) without adding payload fields.

Provider fallback remains inside the single OpenRouter request. Emenda neither retries at application level nor sends a `models` array. The adapter applies the canonical deadline and response bound, exposes only typed redacted outcomes, and keeps the selected model identifier available only where sanitized live evidence requires it.

## 12. Gate ownership

There are six gates in this order:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

| Gate | Architectural scope |
| --- | --- |
| Documentation | Frozen Markdown identity, consistency, links, staged hashes, and documentation-only ancestry |
| Mock Product | Complete reducer-and-effects behavior through deterministic ports and mocks |
| Architecture | Strict core compilation, prohibited type absence, Zod placement, import direction, semantic ports, dependency allowlist, and absence of native scaffolding |
| Provider | Runtime-message and external-result schema enforcement, worker/provider boundary behavior, and live structured-output compatibility |
| Browser Integration | Manifest, permissions, registrations, trusted-storage isolation, lifecycle, DOM safety, overlay accessibility, and bundled-Chromium runtime behavior |
| V0.1 Conformance | Clean final build, minimum-runtime and current-Stable evidence, personal-device evidence, final audit, pushed identity, and stop condition |

A later-gate failure does not erase earlier evidence unless the underlying tested invariant changed.

## 13. Deferred architecture

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, native packaging and signing, store publication, release automation, commercial services, and general cross-platform claims are outside V0.1. They must not shape current ports, packages, or placeholders.

The constitution resolves all product, safety, architecture, and acceptance decisions. The implementation agent retains ordinary discretion over local naming and code organization within the locked boundaries.
