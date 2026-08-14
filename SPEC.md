# Emenda V0.1 Product Specification

> **Frozen product authority, version 2.0.1**

## 1. Authority and objective boundary

This file is authoritative for what Emenda V0.1 does. [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) owns architectural boundaries, and [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md) owns future build order.

The v2.0.1 objective is documentation-only. It ends after the 13-document package is rewritten, verified, hashed, committed, and pushed. Product implementation requires a separate future objective.

## 2. Product goal

Emenda is a personal browser writing assistant. It proposes at most one exact local correction while preserving the writer's meaning, voice, rhythm, register, terminology, names, quotations, and Duktus. It never translates.

The writer's page remains the primary writing surface. Observation begins only after explicit permission for the current origin, and page text changes only after explicit Apply.

## 3. V0.1 runtime and limits

V0.1 is one strict-TypeScript product core and one Chromium Manifest V3 extension with:

```text
minimum_chrome_version: "140"
DEBOUNCE_MS: 600
MAX_CONTEXT_SCALARS: 1200
PROVIDER_TIMEOUT_MS: 8000
MAX_RESPONSE_BYTES: 32768
```

The state union is:

```text
Idle | Debouncing | Checking | Suggestion | Applying | Error
```

There is no persistent Clean state. One eligible revision causes at most one provider request and one response containing zero or one correction.

The supported profile modes are:

```text
auto | de-CH | en-GB | en-US | fr-FR | ka-GE | ru-RU
```

`profileMode` defaults to `auto`. A provider result may additionally identify `unsupported`.

## 4. State, effects, and authority

One pure reducer controls revisions, debounce state, checking, validation outcomes, suggestions, Apply, Dismiss, and errors. Effects execute timers, inference, storage, messaging, and DOM operations and report results back to that reducer.

The core compiles without DOM, Chrome, Node, React, or extension types. Domain values, context policy, and reducer state use pure TypeScript. Zod is permitted only in `core/provider-schema/`, `extension/protocol/`, and the worker-owned trusted-settings boundary. Every runtime message is versioned and strictly validated.

Each eligible committed change synchronously reserves a new monotonically increasing `RevisionId`. It clears any current error or suggestion, invalidates the older Apply capability and timer, best-effort cancels older inference, and starts one trailing-edge 600 ms debounce.

A newer revision is authoritative even when cancellation is unavailable or races. Stale completions, failures, and commands cannot change state, presentation, or page text.

A `SuggestionId` is an opaque capability for one current suggestion. Dismiss accepts only that current capability, preserves page text, invalidates the suggestion, and returns to `Idle`.

The controller verifies before Apply:

```text
current SuggestionId
+ current RevisionId
+ suggestion belongs to that revision
```

Source and snapshot references remain opaque to the core. Source identity, raw DOM data, full document metadata, and page URL remain in the content script.

## 5. Settings authority

Trusted settings are worker-owned and stored in `chrome.storage.local`:

```text
schemaVersion
apiKey
model
profileMode
settingsRevision
enabledOrigins
```

The options page communicates with the worker and never reads or writes this storage area directly.

At worker initialization, before any trusted-settings read or write, the worker must await:

```ts
chrome.storage.local.setAccessLevel({
  accessLevel: "TRUSTED_CONTEXTS",
});
```

Initialization fails closed if the method is unavailable or rejects. Runtime evidence must prove that a content script can neither read the local storage area nor receive its change events. Chrome 140 is the first supported milestone for the relevant [Chromium storage implementation](https://chromium.googlesource.com/chromium/src/+/a8f1f337c692360aaec9470a0a91f965011d37a3) and [Chrome 140 release](https://developer.chrome.com/release-notes/140); compatibility is tested directly against that milestone and current Chrome.

At initialization, a content script requests and caches only:

```text
hasApiKey
profileMode
settingsRevision
```

The model and API key never enter the content script. `protocolVersion` belongs to every validated message envelope, not this configuration payload. The worker sends validated settings-change messages to every live enabled content script to update the cache; the content script does not request configuration before each capture.

Changing the API key, model, or profile increments `settingsRevision`, cancels active inference, invalidates visible suggestions, and leaves processing paused until the next committed input. Origin changes do not increment it; enablement and revocation govern origin authority.

Every content-to-worker check carries the cached `settingsRevision`. The worker compares it with current trusted settings before using the worker-owned key or model. A stale request is rejected, the content cache is resynchronized, and that revision is not retried.

## 6. Observation and IME

Only committed changes start debounce and inference.

Composition handling is centralized:

1. `compositionstart` immediately invalidates current authority, timers, inference, and suggestions.
2. Composing input updates only the surface's current baseline and never starts inference.
3. `compositionend` emits the sole committed change for that composition generation.
4. A later terminal non-composing input with identical text, source, and selection is deduplicated for that generation.
5. A divergent later input is an ordinary external committed change.

A non-collapsed selection is ineligible and returns silently to `Idle`. V0.1 checks only a collapsed caret.

## 7. Scalar text model and focus

All text coordinates are half-open Unicode scalar offsets, not UTF-16 code units, bytes, grapheme clusters, or DOM offsets. Browser-boundary conversion must be explicit and lossless. Logical newlines are LF. Browser soft wrapping never creates a logical newline.

A paragraph is the maximal scalar range between explicit newline boundaries. Within that paragraph, sentence selection uses this deterministic scalar scan:

- terminators are `.`, `!`, and `?`;
- trailing Unicode closing punctuation (`Pe` and `Pf`) and ASCII quotation marks U+0022 and U+0027 belong to the sentence;
- a sentence boundary exists only when the trailing sequence is followed by a Unicode `White_Space` scalar, newline, or the end of text;
- when the caret is in intersentence whitespace, it belongs to the preceding nonempty sentence;
- at the beginning of a paragraph, including its leading whitespace, the caret belongs to the following sentence.

The core does not use `Intl.Segmenter`.

A focus is nonlinguistic when it contains no Unicode Letter scalar (`\p{L}`). Empty, whitespace-only, and nonlinguistic focus cause no request.

Context contains the complete focus and at most 1,200 scalars. It is exactly 1,200 only when at least that much logical document context is available. If the focus itself exceeds 1,200 scalars, capture fails closed and no inference occurs.

If the complete paragraph fits, it is the context. Otherwise, after including the focus, divide remaining capacity evenly between preceding and trailing text. An odd spare scalar goes to the trailing side. Clamp at document boundaries and backfill unused capacity from the available side.

## 8. Model result and local validation

The model-authored result has exactly this information:

```ts
type ModelResult = {
  languageProfile:
    | "de-CH"
    | "en-GB"
    | "en-US"
    | "fr-FR"
    | "ka-GE"
    | "ru-RU"
    | "unsupported";
  corrections:
    | []
    | [{
        range: { start: number; end: number };
        original: string;
        replacement: string;
        category: "spelling" | "grammar" | "punctuation" | "style";
        explanation: string;
      }];
};
```

`range` is half-open, uses Unicode scalar offsets relative to `TextContext.text`, and must remain wholly inside the context's focus range. Revision identity is never model-authored; the provider adapter attaches the request's authoritative revision identity to the local result.

Local validation is strict and rejects extra properties. A correction is accepted only when:

- the attached revision is current;
- the configured and returned language rules pass;
- the result contains exactly one correction;
- scalar offsets are integral, ordered, in bounds, and inside focus;
- `original` exactly equals the context substring at the range;
- `replacement` differs from `original`;
- category is allowed and explanation is nonempty;
- the insertion, deletion, or replacement maps losslessly to the captured snapshot.

The controller maps an accepted context-relative correction once to snapshot-relative scalar coordinates.

In `auto`, any supported returned profile or `unsupported` is valid. In a fixed mode, the returned profile must be that exact profile or `unsupported`. `unsupported` is accepted only with `corrections: []` and returns silently to `Idle`. A different supported profile is the typed failure `LanguageMismatch` and produces no suggestion.

There is no fuzzy match, unique-match search, offset recovery, correction relocation, confidence threshold, or response healing.

## 9. Provider request

The worker sends one non-streaming request to:

```text
POST https://openrouter.ai/api/v1/chat/completions
```

The request contains one concrete user-configured model, the bounded context and focus coordinates, the selected profile, one system instruction, and strict structured-output schema. Emenda has no compiled model default. The request contains no `models` fallback array and uses:

```text
provider.require_parameters: true
provider.allow_fallbacks: false
provider.data_collection: "deny"
```

`settingsRevision` is an internal Emenda authority value and is never sent to OpenRouter.

The worker incrementally enforces the eight-second timeout and 32 KiB response-body limit, then validates locally with Zod. Cancellation is best-effort. Provider, transport, timeout, size, HTTP, parse, and schema failures are typed and redacted.

Only a route that supports the required structured-output parameters and denies provider data collection is eligible. Emenda performs no retry, response repair, streaming, caching, telemetry, analytics, provider failover, or model substitution. The routing values remain explicit because OpenRouter's defaults differ; see [provider routing](https://openrouter.ai/docs/guides/routing/provider-selection).

## 10. Apply contract

`ReplacementRequest` contains the controller-authorized source reference, snapshot reference, expected logical text, snapshot-relative correction range, original, and replacement. It contains no revision oracle for the surface to evaluate.

Immediately before mutation, `BrowserTextSurface` verifies:

```text
same connected source
+ same document
+ same opaque snapshot
+ visible, focused, writable surface
+ exact expected logical text
+ lossless range mapping
+ exact original substring
```

Failure returns a typed refusal without mutation. A refusal after the writer chooses Apply enters `Error`.

Before mutation, the surface registers one expected self-mutation containing:

```text
source
expected pre-edit text
expected post-edit text
expected target range
replacement
```

The only mutation leaf is a runtime-gated `document.execCommand("insertText")` after restoring the verified textarea selection or DOM range. Direct value assignment, DOM rewriting, clipboard operations, simulated keys, and fallback mutation strategies are forbidden.

The exact matching input event is consumed internally as `AppliedChange`: it updates the adapter's snapshot and logical-text baseline and does not emit `ObservedChange`. Successful replacement returns the post-edit snapshot. The controller advances authority, invalidates the suggestion, and returns to `Idle` without debounce or inference.

A mismatching event is an external committed change. It reserves a new revision, invalidates the Apply result, and follows the ordinary input path.

A surface is supported for Apply only after browser evidence proves that one native Undo restores the exact original text.

## 11. Supported surfaces and mappings

V0.1 supports only explicitly enabled top-level HTTP(S) pages with one visible, focused, writable light-DOM `<textarea>` or one bounded contenteditable host. The exact textarea value and collapsed selection must map losslessly between UTF-16 DOM offsets and scalar offsets.

A contenteditable host must use `contenteditable="true"` or `contenteditable="plaintext-only"`. A `<span>` is transparent only when it is visible, has inline computed display, uses the accepted whitespace behavior, introduces no editing boundary, and contributes no generated visual content. The host may contain either:

- the inline grammar: text nodes, `<br>`, and recursively transparent inline `<span>` elements; or
- simple top-level `<div>` or `<p>` blocks whose contents use only that inline grammar.

Mixing top-level inline and block forms is unsupported. Nested blocks or editing hosts, `contenteditable="false"` islands, hidden descendants, replaced elements, generated visual text, unsupported nodes, shadow content, and ambiguous mappings are unsupported.

Computed whitespace modes `pre`, `pre-wrap`, `break-spaces`, `normal`, `nowrap`, and `pre-line` are eligible only when exact visible-text mapping can be proven. Text-node scalars follow the accepted mode.

The mapper records a DOM source span for every emitted logical scalar. In a collapsed whitespace mode, one emitted logical space maps to the complete underlying whitespace run. A nonempty replacement maps from the beginning of its first covered scalar span to the end of its last covered scalar span; an insertion requires one unique recorded boundary.

Logical contenteditable newlines are exact:

- `<br>` emits one newline;
- a boundary between two permitted top-level blocks emits one newline;
- no synthetic leading or trailing newline is added.

Every `<br>` and block-boundary newline maps to a deterministic DOM boundary span. Before a surface is accepted, every logical scalar boundary must round-trip to a DOM boundary. The surface is rejected when a logical range cannot produce one unique safe replacement span.

V0.1 excludes inputs, iframes, shadow-DOM editors, rich, virtualized, canvas, and Google Docs-style editors, restricted or extension pages, file URLs, PDFs, readonly or disabled surfaces, and incognito. Excluded surfaces fail closed rather than operating partially.

## 12. Origin activation and revocation

The manifest disables incognito and declares only:

```json
{
  "permissions": ["activeTab", "scripting", "storage"],
  "host_permissions": ["https://openrouter.ai/*"],
  "optional_host_permissions": ["http://*/*", "https://*/*"]
}
```

There is no static all-sites content script and no all-sites grant. One dynamic content-script registration uses the ID:

```text
emenda-enabled-origins
```

Enable follows this lifecycle:

1. Validate that the active tab is a top-level HTTP(S) page.
2. Request optional permission for its exact origin.
3. Add the origin to worker-owned `enabledOrigins`.
4. Create the registration if this is the first enabled origin; otherwise update its `matches`.
5. Ping the current tab.
6. If no content script responds, inject the packaged content script into the top frame.

Content-script initialization is idempotent: duplicate injection cannot create duplicate listeners, controllers, or overlays.

When `enabledOrigins` is empty there are zero dynamic content-script registrations. The fixed registration is unregistered because Chrome does not accept an empty `matches` list.

Revoke follows this lifecycle:

1. Mark the origin disabled in trusted settings so new work and messages from it are rejected.
2. Cancel worker requests associated with the origin.
3. Send a versioned `Deactivate` message to all live tabs on that origin.
4. Each content script invalidates its current revision, cancels debounce and inference, removes input and composition listeners, removes the overlay host, clears source and snapshot registries, and becomes inert.
5. Update the dynamic registration, or remove it when no origins remain.
6. Remove the optional origin permission.

Unregistering does not remove code already injected into a page, so teardown is a separate required contract; see the [Chrome scripting API](https://developer.chrome.com/docs/extensions/reference/api/scripting).

## 13. Failure transitions

| Outcome | Required result |
| --- | --- |
| Clean result, empty focus, nonlinguistic focus, unsupported language, over-limit focus, non-collapsed selection, or unsupported capture during ordinary typing | `Idle`, silent |
| Stale completion, stale failure, stale command, or cancellation | No presentation change |
| Stale settings revision | Resynchronize cached settings, do not retry that revision |
| Missing configuration | `Error`, with Open Settings |
| Current provider timeout or failure | `Error` |
| Invalid current provider response or `LanguageMismatch` | `Error` |
| Current Apply refusal after writer action | `Error` |
| New committed input | Clear current `Error`, reserve a revision, and debounce |

Errors contain no API key, authorization header, raw context, model body, source identity, or DOM data.

## 14. Presentation and privacy

The content script owns a fixed, unanchored shadow-root overlay. It appears only for a current suggestion or writer-visible error, never autofocuses, and offers exact before/after text, category, concise explanation, Apply, and Dismiss. `Escape` dismisses and `Alt+Enter` applies only a current suggestion without interfering with IME or host editing.

The options page must display this disclosure verbatim:

> Emenda sends only the current bounded text context, up to 1,200 Unicode scalars, to OpenRouter and the provider serving the configured model. It does not send the page URL, full document, source identity or DOM structure. Processing remains subject to OpenRouter’s and the model provider’s policies. The API key is stored in the browser profile, not in an operating-system secret vault.

Emenda stores no text history or persistent text cache and emits no telemetry or analytics. Tests and evidence use synthetic domain-neutral text.

Visible interaction and accessibility details are authoritative in [`UX.md`](UX.md).

## 15. Deferred scope and completion

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, native packaging and signing, store publication, release automation, native placeholders, general cross-OS claims, multiple suggestions, and complex editors are outside V0.1 and must not be scaffolded.

Future implementation is complete only when all six gates in [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md) pass and the factual evidence distinguishes deterministic, bundled-Chromium, minimum-Chrome-140, current-Chrome, and personal-device results.

The constitution resolves all product, safety, architecture, and acceptance decisions. The implementation agent retains ordinary discretion over local naming and code organization within the locked boundaries.
