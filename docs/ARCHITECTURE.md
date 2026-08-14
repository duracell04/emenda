# Emenda V0.1 Architecture

> **Frozen architecture, version 2.0.0**

## 1. Architectural thesis

V0.1 is one browser product:

```text
OS-agnostic strict-TypeScript core
→ browser leaf adapters
→ Chromium Manifest V3 extension
```

The core owns product meaning. The extension owns browser mechanisms. No second runtime, native placeholder, or cross-platform adapter tree exists in V0.1.

## 2. Dependency direction

```text
extension/options ───────────────┐
extension/worker ────────────────┼─→ validated messages and composition
extension/content ───────────────┘
          │
          ├─ BrowserTextSurface
          ├─ shadow-root overlay
          └─ core public ports and values
                    │
                    ▼
      domain + controller + policies + validator
```

Allowed direction:

```text
extension → core
tests → core and extension test seams
core → TypeScript standard language features only
```

Forbidden direction:

```text
core → DOM
core → Chrome APIs
core → Node APIs
core → React
core → extension message types
content script → API key or OpenRouter fetch
service worker → DOM, source identity, or raw page structures
```

## 3. Repository shape

```text
core/
  domain/
  schemas/
  context/
  controller/
  presentation/
extension/
  content/
  worker/
  options/
  manifest.json
tests/
  core/
  provider/
  browser/
  fixtures/
scripts/
  build-extension.mjs
package.json
package-lock.json
```

This is one npm package. Directory names below the required top level may be flattened when that makes the implementation clearer, provided dependency direction remains enforced.

## 4. Core ports

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

A minimal scheduler seam exposes scheduled trailing-edge work and cancellation in semantic terms. Fake clocks own deterministic time in tests.

`SurfaceSnapshot` contains:

```text
opaque SourceReference
opaque SnapshotReference
exact logical text
focus TextRange
```

It never contains a DOM node, selector, frame, document object, geometry, or browser identifier.

## 5. Controller ownership

The content script creates one controller for its document. The controller owns:

- monotonically increasing `RevisionId` values;
- immediate authority changes;
- the one active debounce handle;
- the one active `PendingCheck`;
- immutable current `Revision`;
- validator outcomes;
- current `Suggestion` and `SuggestionId`;
- `Idle | Debouncing | Checking | Suggestion | Applying | Error`;
- Apply and Dismiss commands.

The controller is core code despite being composed in the content script. It knows only ports and domain values.

Every committed input reserves a revision before scheduling. Composition reserves and invalidates immediately, while request eligibility begins only after `compositionend`. Cancellation is an optimization; revision equality is authority.

## 6. Context ownership

The browser adapter captures exact logical text and the post-edit caret. The core derives:

- the sentence containing the caret as focus;
- the enclosing paragraph as context when it fits;
- otherwise an evenly balanced, clamped window;
- an exact upper bound of 1,200 Unicode scalars;
- mappings between context-relative and snapshot-relative scalar ranges.

Browser code converts DOM and UTF-16 positions into scalar offsets. A surface is unsupported when conversion or round-trip mapping is ambiguous or lossy.

## 7. BrowserTextSurface

`BrowserTextSurface` is a leaf adapter in the content script. It owns:

- top-document event observation;
- eligibility, visibility, focus, connectedness, and writability checks;
- composition signals;
- DOM node identity in a private registry;
- logical-text extraction;
- textarea selection conversion;
- conventional contenteditable selection and text-node mapping;
- document and snapshot tokens;
- final current-state verification;
- selection restoration and the mutation leaf;
- typed refusals.

It emits only semantic signals and values to the core. It never sends source references or raw DOM data through extension messaging.

## 8. Safe replacement

The controller resolves `SuggestionId` to its private current suggestion and asks the same surface to replace. Immediately before mutation, the adapter verifies:

```text
current revision
+ same connected writable source
+ same document and opaque snapshot
+ exact current logical text
+ lossless range mapping
+ exact original substring
```

Only then may it restore the exact target selection and invoke the runtime-gated leaf:

```ts
document.execCommand("insertText", false, replacement);
```

The adapter reports unsupported if the operation cannot be proven undo-aware. There is no direct assignment to `value`, DOM rewrite, clipboard path, simulated keyboard path, fuzzy match, or fallback mutation.

## 9. Content-script presentation

The content script owns one fixed-position, unanchored overlay rendered in a closed or otherwise isolated shadow root. Presentation receives display-safe state only:

```text
SuggestionId
exact before text
exact after text
category
concise explanation
writer-visible failure
```

Presentation never receives source identity, snapshot identity, full context, API credentials, or DOM nodes. It emits only `Apply(SuggestionId)`, `Dismiss(SuggestionId)`, and retry/configuration navigation actions explicitly defined by UX.

## 10. Service worker

The ephemeral MV3 worker owns only:

- toolbar activation;
- exact-origin optional permission requests and removals;
- one dynamic content-script registration;
- trusted settings reads and writes;
- strict versioned message validation;
- `AbortController` instances keyed by message-level request IDs;
- the fixed OpenRouter network request.

Worker restarts are normal. Durable truth is limited to enabled origins and trusted settings. Request cancellation is best-effort and never supplies authority; the content-script revision check remains decisive.

## 11. Message boundary

Every runtime message has:

```text
protocol version
discriminated message kind
minimal payload
```

Zod validates messages at both sending and receiving boundaries. Unknown versions, kinds, extra properties, malformed sizes, and unexpected data fail closed.

Permitted content-to-worker provider data consists only of a message request ID and the bounded `CheckRequest` fields needed for the fixed fetch. The worker response contains the copied request/revision correlation and a typed result or typed failure.

Source references, snapshot references, DOM details, full-document text, selection objects, and writer page metadata never cross this boundary.

## 12. Provider adapter

`OpenRouterProvider` implements the core `InferenceProvider` from the content-script perspective through validated worker messaging. It:

- creates a cancelable message request;
- maps worker outcomes to typed core failures;
- copies the current `RevisionId` into the accepted `CheckResult`;
- never trusts model-authored revision identity;
- never heals responses.

The worker sends a non-streaming request to `https://openrouter.ai/api/v1/chat/completions`, sets `provider.require_parameters: true`, uses strict JSON Schema, enforces eight seconds and 32 KiB, and validates the body locally with Zod.

## 13. Permissions and storage

The manifest declares:

```text
permissions: activeTab, scripting, storage
required host: https://openrouter.ai/*
optional hosts: http://*/* and https://*/*
minimum Chrome: 102
incognito: disabled
```

Toolbar activation requests persistent permission for the exact current origin and updates one dynamic registration. Chrome's [optional permissions](https://developer.chrome.com/docs/extensions/develop/concepts/declare-permissions) and [scripting API](https://developer.chrome.com/docs/extensions/reference/api/scripting) define the platform mechanism.

The API key and concrete model are stored in `chrome.storage.local`, restricted to trusted extension contexts. Content code receives only `hasApiKey`. This uses Chrome's documented [storage access-level controls](https://developer.chrome.com/docs/extensions/reference/api/storage). Browser-profile storage is not an operating-system secret vault.

## 14. Build and executable-code policy

`scripts/build-extension.mjs` drives esbuild directly. It emits only local extension assets and bundled executable code. Manifest V3's [remote-code prohibition](https://developer.chrome.com/docs/extensions/develop/migrate/what-is-mv3) is an architectural requirement.

Runtime dependency:

```text
zod
```

Development dependencies:

```text
typescript
esbuild
vitest
playwright
Chrome types
Node types
```

React, Vite, Tailwind, extension frameworks, the OpenRouter SDK, monorepo tools, backends, databases, and code generation are outside the graph.

## 15. Architecture enforcement

The Architecture Gate requires:

- a dedicated core TypeScript compilation with libraries and types that exclude DOM, Chrome, Node, React, and extension globals;
- import-boundary checks preventing `core/` from importing `extension/`;
- a dependency inventory matching the allowlist;
- a manifest permission inventory matching the locked contract;
- message-schema tests;
- source and raw-DOM confinement inspection;
- no placeholder for deferred runtimes.

## 16. Canonical implementation sequence

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

## 17. Deferred architecture

Native hosts, Tauri, Rust, operating-system accessibility APIs, credential-vault adapters, native packaging and signing, store publication, release automation, and native placeholders are deferred. They must not influence V0.1 types or repository structure.

Any later native objective begins from measured browser limitations and defines its own versioned architecture rather than being pre-shaped here.
