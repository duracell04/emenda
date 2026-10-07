# Emenda V0.2 Architecture

> **Frozen architecture, version 2.3.0**

## 1. Authority and objective boundary

[`SPEC.md`](../SPEC.md) defines product behavior and the authoritative trust model. This document defines ownership, boundaries, import direction, and runtime data flow. [`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md) defines build order, and [`PACKAGE-MANIFEST.md`](../PACKAGE-MANIFEST.md) defines freeze identity and lineage.

## 2. System shape

Emenda V0.2 is one npm package with two architectural regions:

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
  exclusive operation registry, fencing and recovery
  bounded transient result cache
  context-menu intent dispatch
  shared WorkerProvider processing
  selected local oMLX or OpenRouter transport
        ^
        |
options page
```

There is one `BrowserTextSurface` implementation for the supported light-DOM `<textarea>` surface. Contenteditable and every other editor class are outside V0.2.

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
| Trusted settings, sender/origin authorization, context-menu intent, global lease/cache/recovery, local discovery/readiness and selected-provider traffic | service worker | Secrets/models remain worker-only; sender and correlation metadata remain ephemeral; durable operation markers are text-free |
| Settings interaction | options page through the worker | The options page never accesses trusted storage directly |
| Visible suggestion and error UI | content-script closed-shadow overlay | It uses text-only sinks, accepts only trusted control events, renders state, and emits semantic commands; it does not own authority |

Actual source and snapshot references are opaque core values backed by content-script-private registries and never serialized. A separate nontext document-scoped opaque correlation token crosses only the strict worker protocol for exact operation/cache identity; it identifies no DOM object by itself and reaches neither provider nor durable/observable records.

## 4. Core state and effects

One pure reducer owns the complete product state:

```text
Disabled | Idle | Settling | Checking | Suggestion | Applying | Error
```

It controls revision reservation, one local 600 ms settling timer, explicit intent, request authority, validation, suggestions, Apply, Dismiss and failure transitions. Timer expiry returns to Idle without capture or dispatch. A separate worker registry controls `Idle | Running | Recovering`, one global lease, exact identity, coalescing, cache and uncertainty. DOM state and cheap timers remain source-local; only the shared dispatch resource is globally coordinated. Inputs are semantic events; outputs are declarative effects. Effect handlers perform timers, inference, messaging, storage interaction, and DOM operations and return typed events to the reducer.

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

Zod is the only direct runtime dependency. The exact development dependency set is TypeScript, esbuild, Vitest, Playwright, Chrome types, and Node types. Exact direct versions, the canonical Node/npm/TypeScript tuple, package-manager metadata, and the npm lockfile are committed, and clean verification installs with `npm ci`. Each architectural mechanism serves a present V0.2 requirement; the product remains one npm package implemented with plain TypeScript, HTML, and CSS.

## 6. Trusted configuration flow

The worker owns the strict schema-version-2 record:

```text
schemaVersion
provider
localOmlx: { apiKey, model }
openrouter: { apiKey, model }
profileMode
settingsRevision
enabledOrigins
```

Local oMLX is active by default, both model/key configurations start missing, and `profileMode` defaults to `auto`. The sole exact v1 migration preserves remote settings as inactive, profile and origins, then increments revision once; unknown/corrupt records fail closed. Record and model validation, migration, options views/actions, active-configuration predicate, and sole explicit-port origin function are owned by [`SPEC.md`](../SPEC.md#5-settings-authority).

One shared sticky initialization promise establishes `TRUSTED_CONTEXTS` storage isolation before any read/write, validates or migrates settings, then reconciles permissions and registration. Listeners register synchronously; the Chrome-140 message bridge returns literal `true` and later calls `sendResponse`. Content scripts cannot read the storage area or receive its change events.

Options reads/saves, origin revocation, local discovery, and synthetic readiness use strict sender-class messages from the exact packaged options page. Read views expose model and key-presence flags without keys; expected-revision saves use independent Keep/Replace/Clear key actions and merge current origins. Content scripts cache only:

```text
isConfigured
settingsRevision
```

One worker-owned active-configuration predicate is reused for inference, public configuration, and immediate pre-Apply authorization. Provider, either model, either credential, or profile changes increment the revision, cancel inference, invalidate suggestions/errors, and broadcast only that public view. Origin transitions remain separate. A stale check resynchronizes configuration without retrying the revision.

The worker alone discovers local model IDs and performs bounded synthetic readiness calls. A dedicated trusted-worker local method handles only the explicit packaged-options Test command, using the immutable fixture and fixed internal 25,000 ms full-processing bound from SPEC; it cannot accept a caller-selected deadline or alter the core WorkerProvider writing port. Discovery and every writing/corpus request retain 15,000 ms. Ephemeral results are keyed to settings revision and worker lifetime, become invalid after settings changes/restart, contain no raw response or credential, and authorize no page text or mutation. A production-parsed/derived synthetic success establishes observed connectivity/compatibility rather than linguistic qualification, pinned residency or future latency.

## 7. Check and presentation flow

1. Trusted input advances content revision, invalidates stale authority and resets that controller's settling timer. Expiry is local bookkeeping only. Idle, focus, navigation, startup and worker wakeup cause no provider traffic.
2. A trusted content-side context-menu candidate and browser-owned `contextMenus.onClicked` invocation establish a one-use intent handoff under [SPEC Section 4.1](../SPEC.md#41-explicit-action-and-context-menu-handoff). Menu lifecycle uses the existing origin reconciliation owner; it confers no permission itself.
3. The content adapter restores/verifies only the intended source, cancels settling, captures current eligible text and uses unchanged pure focus/context algorithms. It sends only bounded context/focus, source correlation/revision, intent/request token and settings revision in a strict protocol-2 one-shot message. Chrome supplies fresh sender identity separately.
4. The worker reauthorizes sender, enabled origin, exact permission and configuration, then derives exact identity from its trusted provider/model/profile. The operation registry coalesces an identical owner or returns Busy for a distinct owner. No queue exists.
5. One capability-driven policy handles any retained uncertainty. Local recovery performs one bounded status GET and permits dispatch only after a valid idle snapshot. Remote uncertainty excludes dispatch only until the original deadline. Recovery and cache behavior resolve through SPEC; transport termination never implies server termination.
6. An exact eligible transient cache hit returns a validated derived outcome with fresh authority. Otherwise the worker acquires one lease, writes its text-free marker before possible inference, and uses the unchanged shared WorkerProvider processing and selected adapter. Recovery/catalog/response processing share the operation deadline.
7. Transport separates known terminal from uncertain completion. Registry owner/fencing checks reject retired events before any cache/readiness/state update. The worker returns only trusted derived correction or typed outcome, never raw model content.
8. Content rechecks source, document, revision, configuration, exact value/selection, focus and exposure before current presentation. The existing suggestion/Apply/Dismiss path supplies fresh capabilities and writer approval.

Provider capability values are worker-composed semantic facts: status inspection is available for local oMLX and unavailable for OpenRouter; cancellation is best-effort for both. The registry does not own DOM or provider payload construction. Concrete transports own network uncertainty and local status projection. Keep these additions inside the existing worker/provider composition, without a generalized provider framework.

Raw DOM/editor identity, actual source/snapshot references and unbounded text remain content-local. Opaque correlation and Chrome sender data are confined to worker-memory authorization and exact cache identity. Provider input remains exactly the four linguistic fields. The worker-memory cache is the only bounded completed-result retention exception; raw response bodies and private durable records remain prohibited.

## 8. Revision and mutation authority

Apply is split deliberately:

- The controller verifies the current `SuggestionId`, current `RevisionId`, and that the suggestion belongs to that revision.
- A one-shot `AuthorizeApply` carrying only `settingsRevision` makes the worker repeat sender, enabled-origin, current exact-permission, active configuration, and revision authorization immediately before local mutation; denial invalidates the suggestion.
- `BrowserTextSurface` restores the captured textarea and collapsed selection after the controlled approval handoff, verifies the same connected source and document, opaque snapshot, foreground-visible writable exposed surface, exact expected logical text, lossless mapping, and exact original substring.
- Inside one scoped synchronous internal selection phase, the surface suspends selection observation only for target selection and immediate readback, then re-verifies the unchanged value, exact target selection, and original substring before mutation. It does not await or require queued or coalesced selection events.

The authorized replacement request contains the opaque source and snapshot references, expected logical text, snapshot-relative scalar range, original, and replacement. The surface does not query or reproduce reducer revision policy.

Immediately before the sole mutation leaf, runtime-gated `document.execCommand("insertText", false, replacement)`, the surface registers a one-use expected self-mutation containing the source, pre-edit text, post-edit text, target range, and replacement. Success requires `true`, the exact synchronous input, and the exact post-state; that input becomes `AppliedChange`, updates the text and resulting-selection baseline, emits no `ObservedChange`, advances authority without inference, and returns to `Idle`. A later queued or coalesced selection notification is self-authored only when current source and selection still equal that baseline. Any unexpected changed state is external and refreshes baseline/authority, and ordinary input starts only local settling; inference additionally requires fresh explicit intent; an unchanged failure is a typed refusal and restores the captured caret only when source and value remain exact. No direct value assignment, DOM rewrite, clipboard operation, simulated input, fuzzy matching, or recovery mutation is allowed.

Composition and foreground handling are centralized at the adapter/controller boundary. `compositionstart` invalidates current authority immediately, binds the pre-composition tuple, and creates an eligible generation only from a trusted event on a qualifying surface with a collapsed caret. Trusted same-generation `beforeinput`/`input` pairs refresh the exact text/selection baseline only; their in-bounds, losslessly mapped IME candidate ranges may be noncollapsed and matching delayed selection notifications are ignored. An untrusted, unpaired, malformed, or mismatching event disqualifies the generation. A trusted qualifying `compositionend` emits the sole committed change only at a losslessly mapped collapsed caret and when terminal text differs from the bound pre-composition text; that exact terminal state synchronously becomes the baseline bound to the new revision. Cancelled/no-op generations remain silent. Only an identical later paired text, source, and selection tuple is suppressed within that generation; any mismatch is external and must independently satisfy ordinary pairing. A hidden-document transition or window blur invalidates authority, clears provenance, cancels work best-effort, and removes presentation without retrying when focus returns.

## 9. Browser text mapping

`BrowserTextSurface` owns the exact textarea value, connected source and document identity, foreground focus and exposure state, exact selection baseline, and bidirectional Unicode-scalar/UTF-16 conversion. It accepts only the visible, window-focused, active, writable, midpoint-exposed, sequentially keyboard-focusable light-DOM textarea predicate in the specification. Ordinary capture and mutation require a collapsed selection; only baseline-only intermediate IME pairs may carry an in-bounds, losslessly mapped noncollapsed candidate range. The same `elementsFromPoint` predicate runs at input, capture, and Apply while skipping only Emenda's current host. Every accepted scalar boundary and correction range round-trips exactly; a covered or background surface, window blur, malformed Unicode, raw CR, a surrogate-interior boundary, changed selection, or any refused surface fails closed before inference or mutation.

The adapter has no DOM-tree text reconstruction or contenteditable mapping. The snapshot binds `value`, `selectionStart`, `selectionEnd`, and `selectionDirection`; selection and focus changes invalidate authority except for the current approval-UI handoff and the scoped internal correction-range selection.

## 10. Origin lifecycle

V0.2 requires Chrome 140 or newer and uses one dynamic registration:

```text
emenda-enabled-origins
```

That registration is persistent, isolated-world, top-frame-only, excludes fallback-origin matching, runs at document idle, and contains only the exact derived origin matches plus the packaged content entry. Direct recovery injection uses the same entry and isolated world for frame 0 only.

The worker accepts content messages only from its own extension, an active outermost HTTP(S) document, an enabled canonical origin, and a currently granted exact permission. One origin-pattern function emits an explicit port and owns permission and registration calls, so default-port and nondefault-port origins are not broadened. One FIFO serializes startup reconciliation, options saves, and every post-prompt origin mutation; each operation rereads current state, and a pending-prompt set protects only the exact grant being requested. `permissions.onAdded` removes externally acquired or broader optional grants, `permissions.onRemoved` disables externally revoked origins, both reconcile the one persistent registration, and every check and Apply authorization repeats permission validation. The exact required local-loopback and OpenRouter provider-pattern set is excluded from optional-grant cleanup and never supplies writing-site authority by itself.

Enablement invokes the permission prompt synchronously in `action.onClicked` before awaiting initialization, then follows the ordered lifecycle and rollback contract in [`SPEC.md`](../SPEC.md#12-origin-activation-and-revocation). The worker pings the active top-level document and injects only when no control listener responds; otherwise validated origin-bound `Activate` state reinitializes the existing script. Revocation persists disabled authority first, then cancels, sends origin-bound document-targeted `Deactivate`, updates registration, and removes permission. When permission is already absent or known targets may be incomplete, an unfiltered all-tab frame-0 broadcast supplies best-effort cleanup without reading tab URLs; receivers compare the control origin with current `location.origin`, so a navigation race cannot affect another origin. Cleanup failure cannot restore authority and is repaired during startup reconciliation.

The content script's permanent control and lifecycle bootstrap remain inert when not authorized. `pagehide` tears down observation and clears registries; initial load and `pageshow` reauthorize before observation or UI. A document that starts prerendering installs one `prerenderingchange` listener and defers its handshake and active features until that event reauthorizes it. This covers external permission removal, re-enable in an already-injected document, BFCache restoration, and prerender activation without treating registration removal as live teardown.

## 11. Provider boundary

Reuse `WorkerProvider` rather than introducing another core provider abstraction. Shared processing owns canonical prompt, bounded input, JSON schema, response bound, fatal decoding, outer/model validation, pure derivation, cancellation, and the full-processing deadline selected by the trusted operation: 15 seconds for writing/discovery and 25 seconds only for the explicit local Settings test. Separate concrete local oMLX and OpenRouter transport adapters own only their fixed endpoint, selected credential, and provider-specific request fields. A worker composition chooses exactly the active adapter.

The local adapter performs one fresh bounded authenticated catalog GET before inference, verifies exact case-sensitive membership inside the same deadline, then uses the exact loopback completion endpoint, direct catalog model, optional auth, nonstreaming strict schema, bounded completion, no thinking/tools, and no OpenRouter fields. The server remains loopback-bound with model fallback disabled. Grammar-downgrade warnings and HTTP-200 error envelopes fail closed. Local discovery and synthetic readiness are worker-owned options operations under the same confinement, cancellation and revision policy. The explicit synthetic test alone receives the fixed startup bound; it makes no separate preload or background call and cannot retry or select another model/provider.

OpenRouter retains its explicit remote endpoint, required key/base-model ID, disabled plugins, routing constraints, omitted reasoning trace, and within-request eligible-endpoint fallback for the same model. That provider-specific behavior never enters local traffic or supplies application-level/cross-provider fallback.

Both paths use the exact request and response contract in [`SPEC.md`](../SPEC.md#9-provider-request), including no credentials/cache/redirect/referrer fetch controls, strict media/envelope/model checks, typed redacted outcomes, 15-second writing/discovery/corpus deadline, the sole fixed 25-second explicit local Settings test exception, and the unchanged 32-KiB response bound. Content scripts never learn provider selection, model IDs, credentials, or raw model content.

Local server logging remains `critical` and canary-tested; local model KV state is allowed. Raw response bodies and persistent text/history remain prohibited. Only SPEC's four-entry/60-second/64-KiB worker-memory cache retains bounded input and validated outcomes; one text-free trusted-local marker survives restart. Local status inspection is read-only and provides an idle observation, not atomic reservation. The operation deadline and permanent fencing preserve the capability-specific recovery contract.

## 12. Gate ownership

There are six gates in this order:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.2 Conformance
```

| Gate | Architectural scope |
| --- | --- |
| Documentation | Frozen Markdown identity, consistency, links, staged hashes, and documentation-only ancestry |
| Mock Product | Complete reducer-and-effects behavior through deterministic ports and mocks |
| Architecture | Strict core compilation, prohibited type absence, Zod placement, import direction, semantic ports, dependency allowlist, and absence of native scaffolding |
| Provider | Runtime-message and external-result schema enforcement, worker/provider boundary behavior, and live structured-output compatibility |
| Browser Integration | Manifest, permissions, registrations, trusted-storage isolation, lifecycle, DOM safety, overlay accessibility, and bundled-Chromium runtime behavior |
| V0.2 Conformance | Clean final build, installed-Brave and personal-Mac evidence, separately identified compatibility evidence, final audit, pushed implementation and evidence identities, and stop condition |

A later-gate failure does not erase earlier evidence unless the underlying tested invariant changed.

## 13. Deferred architecture

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, contenteditable and broader editor support, native packaging and signing, store publication, release automation, commercial services, and general cross-platform claims are outside V0.2. They must not shape current ports, packages, or placeholders.

Builder choices remain those defined by [`AGENTS.md`](../AGENTS.md) and [`ENGINEERING.md`](ENGINEERING.md); they preserve every required ownership and observable boundary in this document.
