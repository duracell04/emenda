# Emenda V0.1 Architecture

> **Frozen architecture, version 2.1.0**

## 1. Authority and objective boundary

[`SPEC.md`](../SPEC.md) defines product behavior and the authoritative trust model. This document defines ownership, boundaries, import direction, and runtime data flow. [`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md) defines build order, and [`PACKAGE-MANIFEST.md`](../PACKAGE-MANIFEST.md) defines freeze identity and lineage.

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
  closed-shadow overlay
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

There is one `BrowserTextSurface` implementation for the supported light-DOM `<textarea>` surface. Contenteditable and every other editor class are outside V0.1.

### 2.1 Deterministic authority boundary

Emenda is deterministic software around one narrow probabilistic judgment boundary:

```text
deterministic state and bounded input
→ canonical model contract
→ probabilistic linguistic judgment
→ strict untrusted result
→ deterministic validation and local derivation
→ writer approval
→ deterministically authorized side effect
```

The model proposes semantic data. Core and extension software retain all execution authority, and the writer retains final semantic authority. Component ownership below enforces the trust model and critical invariants in [`SPEC.md`](../SPEC.md#trust-and-threat-model).

## 3. Ownership

| Concern | Owner | Boundary rule |
| --- | --- | --- |
| Domain values, deterministic text policy, reducer, context, scalar correction derivation, validation, and semantic ports | `core/` | Pure TypeScript; no DOM, Chrome, Node, React, or extension types; no Zod |
| Model-facing input and model-authored result schemas | `core/provider-schema/` | Zod is permitted only for this external model boundary |
| Runtime message schemas | `extension/protocol/` | Versioned, discriminated, strict Zod schemas |
| Controller instance, revision lifetime, cached public configuration, source registries, document lifecycle, and presentation | content script | Raw editor identity, unbounded text, snapshot state, and DOM data remain here; only bounded context is copied out |
| Capture, scalar/UTF-16 mapping, selection identity, and mutation safety | `BrowserTextSurface` in `extension/content/` | Browser types are confined to the adapter |
| Trusted settings schema, storage, sender authorization, permissions, origin lifecycle, request cancellation, and OpenRouter traffic | service worker | Secrets and the configured model never enter content scripts; browser sender metadata is ephemeral and confined |
| Settings interaction | options page through the worker | The options page never accesses trusted storage directly |
| Visible suggestion and error UI | content-script closed-shadow overlay | It uses text-only sinks, accepts only trusted control events, renders state, and emits semantic commands; it does not own authority |

Source and snapshot references are opaque core values backed by content-script-private registries. They are never serialized to the worker, provider, logs, or durable storage.

## 4. Core state and effects

One pure reducer owns the complete product state:

```text
Idle | Debouncing | Checking | Suggestion | Applying | Error
```

It controls revision reservation, the 600 ms trailing debounce, request authority, validation, suggestions, Apply, Dismiss, and failure transitions. Inputs are semantic events; outputs are declarative effects. Effect handlers perform timers, inference, messaging, storage interaction, and DOM operations and return typed events to the reducer.

Each eligible writer-committed change reserves a `RevisionId` synchronously. Ordinary input requires one same-source, same-generation trusted `beforeinput`/`input` ticket with exact pre/post tuples, collapsed selections, and the complete foreground/exposure predicate; its synchronous post-state becomes the latest accepted baseline for that source and generation, and is bound to a new revision only when text changed. The first input or the private queued expiry callback clears the ticket; listener microtasks do not. An eligible composition generation starts from a collapsed caret, admits only trusted paired composing changes with lossless in-bounds selections, allows their transient IME-owned candidate ranges to be noncollapsed, and ends eligible only at a collapsed caret. Delayed or coalesced selection notification is self-authored only when source, current value and selection, generation, and revision identity if one exists equal the latest applicable ordinary, composition, or Apply baseline. Other input on an otherwise supported textarea may update the local baseline and invalidate stale authority without requesting inference; rejected editor classes are ignored without reading their text. A newer revision cancels older work best-effort and is always authoritative. Stale results, failures, and commands cannot change presentation or text.

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

Zod is the only direct runtime dependency. The exact development dependency set is TypeScript, esbuild, Vitest, Playwright, Chrome types, and Node types. Exact direct versions, the canonical Node/npm/TypeScript tuple, package-manager metadata, and the npm lockfile are committed, and clean verification installs with `npm ci`. Each architectural mechanism serves a present V0.1 requirement; the product remains one npm package implemented with plain TypeScript, HTML, and CSS.

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

`profileMode` defaults to `auto`; API key and model ID both default to missing. The strict version, types, canonical origin representation, validation rules, and sole explicit-port origin-pattern derivation are owned by [`SPEC.md`](../SPEC.md#5-settings-authority).

At worker initialization, it must await:

```text
chrome.storage.local.setAccessLevel({ accessLevel: "TRUSTED_CONTEXTS" })
```

Action, message, and permission listeners are registered synchronously at worker module evaluation. Every handler awaits one shared initialization promise that first establishes the storage access level, then reads and validates settings and reconciles permissions and dynamic registration. The Chrome-140 message listener uses the synchronous `return true` plus `sendResponse` bridge and is never itself `async`. No trusted-settings read or write may occur first. Initialization failure is sticky and leaves configuration, content authority, and provider work closed for that worker lifetime. Content scripts must be unable both to read the storage area and to receive its change events.

The options page reads and changes settings through validated worker messages authorized only from the exact packaged options-page URL and extension origin. Its safe read view omits the raw key; revision-aware saves express keep, replace, or clear for that key and merge validated model/profile changes with the worker's current origin state. Content-origin message types require the separate active top-level HTTP(S) sender predicate; the worker rejects every sender/message-class mismatch. On content-script initialization, the worker returns only:

```text
isConfigured
settingsRevision
```

The content script caches that public configuration. Validated settings-change messages to every live enabled content script replace the cache; capture does not fetch configuration again. API-key, model, and profile changes increment `settingsRevision`, cancel active inference, invalidate visible suggestions and obsolete errors, and never retry old text. Origin changes use the activation and revocation lifecycle instead.

Every content-to-worker check carries the cached `settingsRevision`. Before using its private API key and model, the worker validates the browser-supplied sender, current exact origin permission, and revision. A stale request returns current validated public configuration and the originating controller replaces its cache; the rejected revision is not retried. `settingsRevision` and sender metadata are Emenda-internal and are never sent to OpenRouter.

## 7. Check and presentation flow

For an eligible revision:

1. The content script captures an opaque snapshot through `BrowserTextSurface` after debounce.
2. Pure core policy selects the deterministic focus and bounded context under the canonical limits in [`SPEC.md`](../SPEC.md#3-v01-runtime-and-limits).
3. The content script sends a one-shot authored payload containing only that bounded logical context, its exact focus range, request identity, and `settingsRevision`; Chrome separately supplies fresh `MessageSender` metadata. V0.1 uses no long-lived messaging port.
4. The worker projects the sender to the minimum active-top-level authority fields, confirms the exact optional permission and enabled origin, validates configuration revision and message shape, splits the context into model-facing `before`, `focus`, and `after`, then adds its trusted profile and uses its private model and credential for one OpenRouter request.
5. The worker strictly validates the external result, invokes pure scalar derivation, and returns only the trusted derived correction or typed failure in a versioned outcome. Model-authored `correctedFocus` never crosses into the content script.
6. Before any provider outcome changes presentation, the content script proves the current revision plus exact captured source, document, snapshot value and selection, foreground focus, and exposure. A mismatch refreshes only a supported local baseline, invalidates the revision, and stays silent.
7. Only then does the reducer accept the outcome and either return to `Idle`, present one suggestion, or enter `Error` according to the specification.

No separate or unbounded full-document field, editor source identity, snapshot identity, DOM structure, API key, or model identifier crosses a boundary that does not own it. The bounded context may equal all text of a short document. Chrome-supplied URL and document metadata necessarily reach the worker listener, but are read only for current sender authority and are never persisted, logged, placed in errors, or forwarded.

## 8. Revision and mutation authority

Apply is split deliberately:

- The controller verifies the current `SuggestionId`, current `RevisionId`, and that the suggestion belongs to that revision.
- A one-shot `AuthorizeApply` carrying only `settingsRevision` makes the worker repeat sender, enabled-origin, current exact-permission, and revision authorization immediately before local mutation; denial invalidates the suggestion.
- `BrowserTextSurface` restores the captured textarea and collapsed selection after the controlled approval handoff, verifies the same connected source and document, opaque snapshot, foreground-visible writable exposed surface, exact expected logical text, lossless mapping, and exact original substring.
- Inside one scoped synchronous internal selection phase, the surface suspends selection observation only for target selection and immediate readback, then re-verifies the unchanged value, exact target selection, and original substring before mutation. It does not await or require queued or coalesced selection events.

The authorized replacement request contains the opaque source and snapshot references, expected logical text, snapshot-relative scalar range, original, and replacement. The surface does not query or reproduce reducer revision policy.

Immediately before the sole mutation leaf, runtime-gated `document.execCommand("insertText", false, replacement)`, the surface registers a one-use expected self-mutation containing the source, pre-edit text, post-edit text, target range, and replacement. Success requires `true`, the exact synchronous input, and the exact post-state; that input becomes `AppliedChange`, updates the text and resulting-selection baseline, emits no `ObservedChange`, advances authority without inference, and returns to `Idle`. A later queued or coalesced selection notification is self-authored only when current source and selection still equal that baseline. Any unexpected changed state is external and refreshes baseline/authority, but starts inference only with independent eligible paired provenance; an unchanged failure is a typed refusal and restores the captured caret only when source and value remain exact. No direct value assignment, DOM rewrite, clipboard operation, simulated input, fuzzy matching, or recovery mutation is allowed.

Composition and foreground handling are centralized at the adapter/controller boundary. `compositionstart` invalidates current authority immediately, binds the pre-composition tuple, and creates an eligible generation only from a trusted event on a qualifying surface with a collapsed caret. Trusted same-generation `beforeinput`/`input` pairs refresh the exact text/selection baseline only; their in-bounds, losslessly mapped IME candidate ranges may be noncollapsed and matching delayed selection notifications are ignored. An untrusted, unpaired, malformed, or mismatching event disqualifies the generation. A trusted qualifying `compositionend` emits the sole committed change only at a losslessly mapped collapsed caret and when terminal text differs from the bound pre-composition text; that exact terminal state synchronously becomes the baseline bound to the new revision. Cancelled/no-op generations remain silent. Only an identical later paired text, source, and selection tuple is suppressed within that generation; any mismatch is external and must independently satisfy ordinary pairing. A hidden-document transition or window blur invalidates authority, clears provenance, cancels work best-effort, and removes presentation without retrying when focus returns.

## 9. Browser text mapping

`BrowserTextSurface` owns the exact textarea value, connected source and document identity, foreground focus and exposure state, exact selection baseline, and bidirectional Unicode-scalar/UTF-16 conversion. It accepts only the visible, window-focused, active, writable, midpoint-exposed, sequentially keyboard-focusable light-DOM textarea predicate in the specification. Ordinary capture and mutation require a collapsed selection; only baseline-only intermediate IME pairs may carry an in-bounds, losslessly mapped noncollapsed candidate range. The same `elementsFromPoint` predicate runs at input, capture, and Apply while skipping only Emenda's current host. Every accepted scalar boundary and correction range round-trips exactly; a covered or background surface, window blur, malformed Unicode, raw CR, a surrogate-interior boundary, changed selection, or any refused surface fails closed before inference or mutation.

The adapter has no DOM-tree text reconstruction or contenteditable mapping. The snapshot binds `value`, `selectionStart`, `selectionEnd`, and `selectionDirection`; selection and focus changes invalidate authority except for the current approval-UI handoff and the scoped internal correction-range selection.

## 10. Origin lifecycle

V0.1 requires Chrome 140 or newer and uses one dynamic registration:

```text
emenda-enabled-origins
```

That registration is persistent, isolated-world, top-frame-only, excludes fallback-origin matching, runs at document idle, and contains only the exact derived origin matches plus the packaged content entry. Direct recovery injection uses the same entry and isolated world for frame 0 only.

The worker accepts content messages only from its own extension, an active outermost HTTP(S) document, an enabled canonical origin, and a currently granted exact permission. One origin-pattern function emits an explicit port and owns permission and registration calls, so default-port and nondefault-port origins are not broadened. One FIFO serializes startup reconciliation, options saves, and every post-prompt origin mutation; each operation rereads current state, and a pending-prompt set protects only the exact grant being requested. `permissions.onAdded` removes externally acquired or broader optional grants, `permissions.onRemoved` disables externally revoked origins, both reconcile the one persistent registration, and every check and Apply authorization repeats permission validation.

Enablement invokes the permission prompt synchronously in `action.onClicked` before awaiting initialization, then follows the ordered lifecycle and rollback contract in [`SPEC.md`](../SPEC.md#12-origin-activation-and-revocation). The worker pings the active top-level document and injects only when no control listener responds; otherwise validated origin-bound `Activate` state reinitializes the existing script. Revocation persists disabled authority first, then cancels, sends origin-bound document-targeted `Deactivate`, updates registration, and removes permission. When permission is already absent or known targets may be incomplete, an unfiltered all-tab frame-0 broadcast supplies best-effort cleanup without reading tab URLs; receivers compare the control origin with current `location.origin`, so a navigation race cannot affect another origin. Cleanup failure cannot restore authority and is repaired during startup reconciliation.

The content script's permanent control and lifecycle bootstrap remain inert when not authorized. `pagehide` tears down observation and clears registries; initial load and `pageshow` reauthorize before observation or UI. A document that starts prerendering installs one `prerenderingchange` listener and defers its handshake and active features until that event reauthorizes it. This covers external permission removal, re-enable in an already-injected document, BFCache restoration, and prerender activation without treating registration removal as live teardown.

## 11. Provider boundary

The worker alone owns the endpoint, credential, required model setting, routing, cancellation, bounded response reading, outer-envelope checks, external Zod parsing, and conversion from the external result into the trusted derived correction. It implements the canonical serialization, JSON Schema, request fields, response projection, and prompt in [`SPEC.md`](../SPEC.md#8-model-facing-contract-and-local-derivation) and [`SPEC.md`](../SPEC.md#9-provider-request) without adding payload fields.

Provider fallback remains inside the single OpenRouter request for the configured ID. Emenda neither retries at application level nor sends a `models` array. The adapter disables request plugins, redirects, credentials, cache, and referrer; omits returned reasoning traces; requires JSON media type, fatal UTF-8, and exact requested/returned model-ID equality; applies the canonical deadline and response bound; exposes only typed redacted outcomes; and keeps the model identifier available only where sanitized live evidence requires it. A model-shaped catalog entry may itself be a routing service; syntax does not claim otherwise.

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
| V0.1 Conformance | Clean final build, minimum-runtime and current-Stable evidence, personal-device evidence, final audit, pushed implementation and evidence identities, and stop condition |

A later-gate failure does not erase earlier evidence unless the underlying tested invariant changed.

## 13. Deferred architecture

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, contenteditable and broader editor support, native packaging and signing, store publication, release automation, commercial services, and general cross-platform claims are outside V0.1. They must not shape current ports, packages, or placeholders.

Builder choices remain those defined by [`AGENTS.md`](../AGENTS.md) and [`ENGINEERING.md`](ENGINEERING.md); they preserve every required ownership and observable boundary in this document.
