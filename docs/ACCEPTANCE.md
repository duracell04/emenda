# Emenda V0.1 Acceptance

> **Frozen acceptance contract, version 2.0.0**

## 1. Evidence standard

A gate passes only through direct, reproducible evidence recorded in `docs/EVIDENCE.md`. Claims distinguish:

```text
inspected
compiled
deterministically tested
integration tested
live provider verified
runtime verified
```

Later success does not erase an earlier failure. Environment, commands, commit identity, exact results, and limitations are recorded.

## 2. Active gates

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

Presentation and accessibility are part of Browser Integration. There is no separate presentation gate.

## 3. Canonical implementation sequence

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

## 4. Documentation Gate

Required evidence:

- repository HEAD contains exactly the 13 constitutional Markdown files and no implementation source;
- all 13 files identify version 2.0.0;
- freeze ID is `emenda-clean-room-v2.0.0-2026-08-14`;
- version 1.0.1 is preserved at `d3192b7`;
- every local Markdown link resolves;
- all canonical-sequence occurrences are identical;
- active gates use only the six locked names and order;
- all native-related references are explicitly deferred;
- the 11 immutable documents match SHA-256 values computed from exact staged Git-blob bytes;
- `PACKAGE-MANIFEST.md` and `docs/EVIDENCE.md` are excluded from the checksum table;
- the evidence ledger contains a blank v2.0.0 template and no implementation claim;
- `git diff --check` passes;
- the documentation commit is pushed and local/remote identity matches.

## 5. Mock Product Gate

### 5.1 Timing and authority

Fake-clock tests prove:

- eligible committed input reserves a revision synchronously;
- composition activity invalidates synchronously;
- inference waits for `compositionend`;
- no request at 599 ms and exactly one eligible request at 600 ms;
- a new input restarts the trailing-edge timer;
- one request maximum per current controller revision;
- a newer revision calls cancellation best-effort;
- a stale result and stale failure remain silent;
- the current revision wins regardless of completion order.

### 5.2 Context and Unicode

Deterministic tests cover:

- Georgian and Russian;
- combining marks;
- emoji and supplementary-plane scalars;
- half-open scalar offsets;
- the sentence containing the post-edit caret;
- paragraph context when it fits;
- evenly balanced and edge-clamped windows;
- the exact 1,200-scalar upper bound;
- overlong focus refusal;
- correction containment inside focus;
- empty, whitespace-only, and nonlinguistic silence.

### 5.3 Schema and validation

Strict-schema tests cover:

- clean response;
- one valid correction;
- malformed JSON and malformed shape;
- extra properties;
- multiple corrections;
- unsupported language;
- insertion;
- deletion;
- replacement;
- no-op;
- out-of-bounds and out-of-focus ranges;
- mismatched original substring;
- stale adapter-copied revision;
- concise explanation and allowed categories.

### 5.4 Complete product

Mock composition proves the full flow from signal through Apply or Dismiss. It includes typed capture failures, changed sources, changed snapshots, changed text, failed mapping, stale Apply, exact replacement recording, and no mutation after Dismiss or refusal.

## 6. Architecture Gate

Required evidence:

- `core/` compiles under strict TypeScript with DOM, Chrome, Node, React, and extension types unavailable;
- import checks prove `core/` never imports `extension/`;
- public core declarations contain no browser or runtime mechanism;
- repository shape is one npm package;
- Zod is the only direct runtime dependency;
- development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, and Chrome/Node types;
- no React, Vite, Tailwind, extension framework, OpenRouter SDK, monorepo tool, backend, database, or code generation exists;
- no geometry, credential-store, native host, or accessibility port exists;
- no deferred-runtime placeholder exists.

## 7. Provider Gate

### 7.1 Deterministic provider tests

Tests prove:

- the endpoint is exactly `https://openrouter.ai/api/v1/chat/completions`;
- the request is non-streaming and contains only the bounded product payload;
- a concrete configured model is required;
- `provider.require_parameters` is exactly `true`;
- strict JSON Schema is requested;
- local Zod validation follows transport parsing;
- revision identity is adapter-copied and never model-authored;
- runtime messages are versioned, discriminated, minimal, and strict;
- unknown versions, extra properties, and malformed messages fail closed;
- timeout occurs at eight seconds;
- body reading stops above 32 KiB;
- cancellation maps to a typed outcome;
- HTTP, transport, timeout, size, parse, schema, and unsupported failures are typed;
- secrets, authorization headers, raw context, raw response bodies, source identity, and DOM data are absent from logs and failures;
- retry, healing, fallback, streaming, persistent cache, telemetry, and analytics are absent.

### 7.2 Live provider evidence

With one dedicated spend-limited key and one concrete structured-output model, record:

- model identifier;
- UTC timestamp;
- one correction and one clean case for each of `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`;
- one unsupported-language case;
- sanitized structured outcome and end-to-end latency for each case;
- no credential or raw private text.

A live case passes only when the same schema and validation path used by the product accepts the result.

## 8. Browser Integration Gate

Use Playwright's [persistent-Chromium extension setup](https://playwright.dev/docs/chrome-extensions) against the built unpacked extension.

### 8.1 Permission lifecycle

Prove:

- toolbar activation requests the exact current HTTP(S) origin;
- grant persists and updates one dynamic content-script registration;
- revocation removes the origin from that registration;
- another origin remains inactive until explicitly enabled;
- restricted pages, file URLs, PDFs, and incognito stay unsupported;
- there is no static all-sites script or `<all_urls>` grant.

### 8.2 Textarea runtime

Prove on a visible, focused, writable light-DOM `<textarea>`:

- committed input, debounce, request, suggestion, Apply, and Dismiss;
- insertion, deletion, and replacement;
- IME composition;
- stale result silence;
- changed-source, changed-snapshot, changed-text, and original-mismatch refusal;
- exact focus preservation;
- one native Undo restores the exact original text.

### 8.3 Contenteditable runtime

Prove the same contract on simple `contenteditable="true"` and `plaintext-only` fixtures. Include multi-node lossless mapping where supported and explicit refusal where mapping is ambiguous.

### 8.4 Exclusion behavior

Verify fail-closed behavior for inputs, iframes, shadow DOM, rich/virtualized/canvas editors, Google Docs-style fixtures, readonly or disabled surfaces, invisible or unfocused surfaces, and unsupported mappings.

### 8.5 Presentation and accessibility

Prove:

- the overlay is fixed, unanchored, and shadow-root isolated;
- it appears only for current suggestions or writer-triggered failures;
- it never autofocuses or steals page focus;
- before/after text, category, explanation, Apply, and Dismiss are understandable;
- Escape dismisses and Alt+Enter applies only the current suggestion;
- controls have accessible names;
- keyboard focus is visible;
- focus order is coherent;
- reduced-motion preference is honored;
- normal text, controls, and meaningful non-text indicators meet WCAG 2.2 AA;
- color is not the only carrier of meaning.

### 8.6 Storage and message isolation

Prove:

- the API key and model are written and read only by trusted extension contexts;
- content scripts receive only `hasApiKey`;
- source references and raw DOM data never leave the content script;
- worker restart preserves only intended durable settings and permissions;
- executable code is local to the package.

## 9. V0.1 Conformance Gate

Required final evidence:

- all prior gates remain passing at the final commit;
- full deterministic and persistent-Chromium suites pass from a clean install;
- production extension build succeeds;
- unpacked-extension smoke passes on current Chrome Stable, with exact version and host OS recorded;
- manifest permissions and dynamic-registration behavior match the constitution;
- dependency and bundled-code inventories match the allowlists;
- no secret, private text, telemetry, analytics, or persistent text cache exists;
- supported and excluded surface claims are precise;
- cross-OS runtime support is not claimed without separate evidence;
- final commit is pushed, local and remote commit identity match, and the worktree is clean.

Then stop.

## 10. Explicitly deferred evidence

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, packaging, signing, Chrome Web Store publication, release automation, native placeholders, and general cross-OS runtime evidence belong to later versioned objectives.
