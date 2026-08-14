# Emenda V0.1 Specification

> **Frozen product specification, version 2.0.0**

## 1. Product goal

Emenda is a personal writing assistant that improves browser text while preserving the author's meaning, voice, rhythm, register, terminology, and Duktus.

The writer's page remains the primary writing surface. Emenda is a small deterministic control layer around OpenRouter:

```text
observe
→ reserve revision authority
→ debounce
→ capture
→ derive context
→ infer
→ validate
→ suggest
→ decide
→ apply safely
```

OpenRouter performs linguistic judgment. Emenda owns observation policy, context selection, revision authority, strict validation, presentation, explicit consent, and safe application.

## 2. Active V0.1 runtime

V0.1 has one OS-agnostic strict-TypeScript product core and one active runtime: a Chromium Manifest V3 extension requiring Chrome 102 or newer.

The core contains no DOM, Chrome, Node, React, or extension types. Browser APIs and nodes exist only in extension leaf code. This constitution makes no cross-OS runtime claim beyond separately recorded evidence.

## 3. User outcome

```text
writer edits an eligible browser surface
→ committed input reserves a RevisionId immediately
→ writer pauses for 600 ms
→ Emenda captures the current logical text and caret
→ Emenda derives a focus sentence and bounded context
→ one current request is sent to OpenRouter
→ zero corrections returns silently to Idle
→ one exact valid correction creates a Suggestion
→ writer applies or dismisses
→ Apply mutates only the verified current surface in one undo step
```

Observation is ambient after explicit per-origin enablement. Application is always explicit.

## 4. Constants and scalar model

```ts
const DEBOUNCE_MS = 600;
const MAX_CONTEXT_SCALARS = 1200;
const PROVIDER_TIMEOUT_MS = 8000;
const MAX_RESPONSE_BYTES = 32 * 1024;
```

All text ranges are half-open offsets over Unicode scalar values, not UTF-16 code units, bytes, grapheme clusters, or DOM offsets. Conversions at browser boundaries must be explicit and lossless.

## 5. Semantic ports

```ts
interface TextSurface {
  observe(sink: (signal: SurfaceSignal) => void): Disposable;
  capture(change: ObservedChange): Promise<SurfaceSnapshot>;
  replaceIfCurrent(request: ReplacementRequest): Promise<ReplacementResult>;
}

interface InferenceProvider {
  check(request: CheckRequest): PendingCheck;
}

type PendingCheck = {
  result: Promise<CheckResult>;
  cancel(): void;
};
```

The core also owns a minimal scheduler seam sufficient to schedule and cancel the debounce and to make fake-clock tests deterministic. It exposes time semantics rather than browser timer objects.

No geometry port, credential-store port, native host port, or accessibility port enters V0.1.

## 6. Shared types

The shared domain includes:

- `RevisionId`: monotonically increasing session authority reserved synchronously.
- `Revision`: immutable current check input, including its ID, snapshot references, and derived context.
- `SourceReference`: opaque equality-capable identity understood by the active `TextSurface`.
- `SnapshotReference`: opaque identity for one captured document state.
- `TextRange`: half-open Unicode scalar range.
- `ObservedChange`: minimal semantic notification that eligible committed text may have changed.
- `SurfaceSignal`: a committed change, composition invalidation/end signal, or typed unavailability.
- `SurfaceSnapshot`: opaque source and snapshot references, exact logical text, and a focus range; never a DOM reference.
- `TextContext`: bounded context text, focus text, language profile, and range mapping back to the snapshot.
- `Correction`: one exact proposed replacement.
- `SuggestionId`: opaque current-presentation capability.
- `Suggestion`: current revision plus validated correction and display-safe fields.
- `CheckRequest` and `CheckResult`: provider boundary values.
- `ReplacementRequest` and `ReplacementResult`: application boundary values.
- `Disposable`, scheduler handles, state values, and typed failures.

All public values are immutable. Opaque references are never serialized to the model and source identity never leaves the content script.

## 7. Observation, composition, and revision authority

An eligible committed `input` event reserves the next `RevisionId` synchronously, clears any visible suggestion, invalidates every older timer or Apply capability, best-effort cancels older provider work, and starts one trailing-edge 600 ms debounce.

Composition behavior is authoritative:

1. `compositionstart` and composing input reserve a new revision and invalidate current work immediately.
2. No inference request begins while composition is active.
3. `compositionend` is treated as the committed change that starts the trailing-edge debounce.

Each controller revision may produce at most one provider request. A newer revision always wins, even if cancellation is unavailable or races with completion. Stale completion and stale failure are silent and cannot alter state, presentation, or text.

## 8. Capture and context selection

After debounce, the current change is captured. Capture fails closed if the page, document, source, focus, writability, visibility, or mapping is no longer eligible.

The post-edit caret determines focus:

1. Select the sentence containing the caret as the focus.
2. If its paragraph fits within 1,200 Unicode scalars, use that paragraph as surrounding context.
3. Otherwise construct a 1,200-scalar window around the focus, dividing remaining capacity evenly before and after it, clamping at document edges, and backfilling from the available side.
4. If the focus itself exceeds 1,200 scalars, return a typed context-limit failure and make no provider request.

The focus range is expressed relative to both snapshot logical text and request context. Corrections must remain entirely inside the focus.

Sentence and paragraph selection is deterministic and language-neutral at the core boundary. Line-break conventions are normalized only if the browser adapter can preserve exact bidirectional mapping. Empty, whitespace-only, or nonlinguistic focus produces no request and returns silently to `Idle`.

## 9. Language profiles

The configured profile is one of:

```text
auto
de-CH
en-GB
en-US
fr-FR
ka-GE
ru-RU
unsupported
```

`unsupported` fails closed and makes no inference request. `auto` asks the provider to identify one supported profile or return `unsupported`. Emenda never translates. It preserves names, quotations, specialist terms, and short embedded passages.

## 10. Correction contract

```ts
type Correction = {
  range: TextRange;
  original: string;
  replacement: string;
  category: "spelling" | "grammar" | "punctuation" | "style";
  explanation: string;
};
```

The style category is restrained. It may correct a clear local defect while preserving meaning and Duktus. Confidence is absent.

A response contains:

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
  corrections: [] | [Correction];
};
```

Revision identity is never model-authored. The provider adapter copies the authoritative `RevisionId` from `CheckRequest` into `CheckResult`.

## 11. Model request and response

The user must configure a concrete OpenRouter model that supports structured output. There is no compiled model default, and `openrouter/free` is not a daily-product default.

The service worker uses the fixed endpoint:

```text
POST https://openrouter.ai/api/v1/chat/completions
```

The request is non-streaming and minimal: configured model, one system instruction, the bounded context and focus/range data, the selected profile, `provider.require_parameters: true`, temperature only if required by the chosen model, and strict JSON Schema response formatting.

The model instruction requires zero or one correction, exact scalar range and original text, a replacement, a category, a concise explanation, preservation of meaning and Duktus, no translation, and `unsupported` when the text cannot be handled safely.

Use strict JSON Schema as documented by [OpenRouter structured outputs](https://openrouter.ai/docs/guides/features/structured-outputs), followed by local Zod validation. No response healing, extraction from prose, second-pass repair, retry, fallback model, or streaming is permitted.

The service worker enforces an eight-second deadline and a 32 KiB response limit while reading the body. Cancellation is best-effort. Provider, transport, timeout, size, HTTP, parse, schema, and unsupported-language failures are typed and contain no key or raw request text.

## 12. Local validation

The validator accepts exactly one suggestion only when all conditions hold:

- the result carries the current adapter-copied revision ID;
- `languageProfile` is supported;
- the schema is exact and has no extra properties;
- `corrections` contains exactly one item;
- scalar offsets are integers, ordered, in bounds, and fully inside focus;
- `original` equals the exact substring at the range;
- `replacement` differs from `original`;
- category is allowed and explanation is concise and nonempty;
- the operation is a replacement, insertion, or deletion that can map losslessly.

Malformed, multiple-correction, unsupported-language, out-of-focus, mismatched-range, no-op, or stale results create no suggestion. Writer-triggered provider or surface failures may enter `Error`; stale failures remain silent.

There is no fuzzy match, unique-match search, offset recovery, correction relocation, or confidence threshold.

## 13. State machine

```ts
type State =
  | { kind: "Idle" }
  | { kind: "Debouncing"; revisionId: RevisionId }
  | { kind: "Checking"; revisionId: RevisionId }
  | { kind: "Suggestion"; suggestion: Suggestion }
  | { kind: "Applying"; suggestionId: SuggestionId }
  | { kind: "Error"; failure: WriterVisibleFailure };
```

There is no persistent `Clean` state or clean-success UI. Zero corrections returns silently to `Idle`.

Dismiss accepts the current `SuggestionId`, invalidates that suggestion, and returns to `Idle` without mutation. Apply accepts only `SuggestionId`; presentation never supplies source, range, original, replacement, revision, or DOM data.

## 14. Apply safety

Immediately before mutation, `BrowserTextSurface` verifies:

```text
current revision
+ same connected writable source
+ same document and opaque snapshot
+ exact current logical text
+ lossless range mapping
+ exact original substring
```

Failure returns a typed refusal without mutation.

The only V0.1 mutation leaf is a runtime-gated `document.execCommand("insertText")` call after restoring the verified selection or textarea range. Direct-value assignment, DOM rewriting, clipboard use, simulated keys, and alternative mutation fallbacks are forbidden.

A surface is positively supported only when runtime integration evidence proves that one native browser Undo restores the exact original text after Apply.

## 15. Supported browser surfaces

V0.1 positively supports only:

- top-level HTTP(S) pages;
- origins explicitly enabled by the writer;
- visible, focused, writable light-DOM `<textarea>`;
- simple light-DOM `contenteditable="true"` or `contenteditable="plaintext-only"`;
- surfaces whose complete logical text, focus, target range, selection, and replacement map losslessly.

V0.1 excludes:

- `<input>` elements;
- iframes;
- shadow DOM editors;
- rich, virtualized, or canvas editors;
- Google Docs-style surfaces;
- restricted browser and extension pages;
- file URLs and PDFs;
- readonly or disabled surfaces;
- incognito.

Excluded surfaces are unsupported rather than partially supported.

## 16. Extension permissions and activation

The manifest requires Chrome 102 or newer and disables incognito. It declares only:

```json
{
  "permissions": ["activeTab", "scripting", "storage"],
  "host_permissions": ["https://openrouter.ai/*"],
  "optional_host_permissions": ["http://*/*", "https://*/*"]
}
```

Toolbar activation requests persistent optional permission for the exact current origin. The worker then maintains one dynamic content-script registration whose matches equal the enabled-origin set. Revocation removes the origin and updates the registration. There is no static all-sites content script and no `<all_urls>` grant.

This follows Chrome's [optional-permission model](https://developer.chrome.com/docs/extensions/develop/concepts/declare-permissions) and [dynamic scripting API](https://developer.chrome.com/docs/extensions/reference/api/scripting).

## 17. Extension boundaries

The content script owns:

- controller and scheduler composition;
- revision lifetime and current suggestions;
- `BrowserTextSurface` and all DOM references;
- logical-text and scalar mapping;
- fixed-position shadow-root overlay;
- focus-neutral keyboard handling.

The ephemeral service worker owns only:

- optional permission lifecycle and dynamic registration;
- trusted settings access;
- request cancellation;
- the fixed OpenRouter fetch;
- strictly validated versioned runtime messages.

The options page writes the API key and model. `chrome.storage.local` is restricted to trusted extension contexts. Content scripts receive only `hasApiKey`, never the key or model. Chrome documents default content-script storage exposure and trusted-context restriction in the [storage API](https://developer.chrome.com/docs/extensions/reference/api/storage).

Browser-profile storage is disclosed as browser storage and not represented as an operating-system secret vault.

All executable code is bundled locally as required by [Manifest V3](https://developer.chrome.com/docs/extensions/develop/migrate/what-is-mv3). There is no remotely hosted script.

## 18. Presentation contract

The overlay:

- is fixed to the viewport and deliberately unanchored;
- renders inside a content-script-owned shadow root;
- appears only for a current suggestion or writer-triggered failure;
- never autofocuses or steals page focus;
- shows exact before and after text, category, and concise explanation;
- provides Apply, Dismiss, Escape, and Alt+Enter;
- uses accessible names, visible focus, reduced motion, and WCAG 2.2 AA styling.

Escape dismisses the current suggestion. Alt+Enter applies it only when a current suggestion exists. Key handling must preserve composition and host editing behavior.

## 19. Data, privacy, and observability

Only the bounded request context is sent to OpenRouter. There is no persistent text cache, telemetry, analytics, request logging, correction history, or source-identity export.

Logs and typed failures redact the API key, authorization header, raw context, raw model body, and DOM data. Tests use synthetic domain-neutral fixtures.

## 20. Package and dependencies

V0.1 is one npm package with this top-level implementation shape:

```text
core/
extension/
tests/
scripts/build-extension.mjs
package.json
package-lock.json
```

Direct dependencies are limited to Zod. Development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, and Chrome/Node types.

The extension uses plain TypeScript, HTML, and CSS. V0.1 contains no React, Vite, Tailwind, extension framework, OpenRouter SDK, monorepo tooling, backend, database, or code generation.

## 21. Canonical implementation sequence

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

## 22. Deferred scope

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, native packaging, signing, Chrome Web Store publication, release automation, native placeholders, and general cross-OS runtime claims are explicitly deferred.

Native work begins only after real browser usage demonstrates a material unmet need and a separately versioned objective authorizes it.

## 23. Definition of Done

V0.1 is complete only when every gate in [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md) passes, including deterministic mock behavior, core dependency isolation, provider conformance, persistent-Chromium browser integration, one-step Undo, live supported-profile evidence, and an unpacked-extension smoke on current Chrome Stable.

Implementation then records the final evidence, commits, pushes, verifies remote identity and a clean worktree, and stops.
