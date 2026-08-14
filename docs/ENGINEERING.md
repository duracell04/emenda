# Emenda Engineering Standard

> **Frozen engineering standard, version 2.0.0**

## 1. Engineering objective

Build the smallest inspectable system that makes incorrect state, stale authority, invalid external data, overbroad permissions, and unsafe mutation difficult to express.

V0.1 optimizes for:

```text
semantic clarity
→ deterministic behavior
→ narrow browser authority
→ strict boundaries
→ direct runtime evidence
→ low maintenance burden
```

## 2. Canonical implementation sequence

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

## 3. TypeScript standard

- Enable strict TypeScript and the strongest practical unchecked-access, override, fallthrough, and exact-optional-property checks.
- Model states and failures as exhaustive discriminated unions.
- Use branded or opaque types for revision, source, snapshot, suggestion, and message identifiers.
- Prefer immutable readonly values and pure policy functions.
- Keep scalar/UTF-16 conversion explicit at leaf boundaries.
- Avoid type assertions at trust boundaries; parse through strict Zod schemas.
- Make impossible transitions compile-time or local runtime failures.

`core/` has its own compilation target with no DOM, Chrome, Node, React, or extension ambient types.

## 4. Boundary validation

Validate at:

- runtime message send and receive;
- trusted settings read;
- model response parse;
- provider error conversion;
- DOM-to-semantic capture;
- semantic-to-DOM range mapping;
- pre-mutation current-state checks.

Schemas reject unknown properties. Validation never repairs, guesses, relocates, or widens authority.

## 5. Deterministic time and concurrency

The scheduler seam exists only to make trailing-edge timing deterministic. Tests use fake clocks.

Revision equality is the authority mechanism. Cancellation reduces wasted work but never establishes correctness. Every asynchronous continuation checks current revision before changing state or text.

Provider requests, body reads, and message replies have explicit ownership and completion paths. Worker suspension or restart cannot grant stale authority.

## 6. Unicode and text mapping

Core logic uses Unicode scalar offsets. Tests include Georgian, Russian, combining marks, emoji, and boundary cases.

Browser adapters may use UTF-16 and DOM Range internally, but each conversion must:

- preserve exact logical text;
- round-trip start and end positions;
- distinguish insertion, deletion, and replacement;
- refuse ambiguous node, line-break, or normalization mappings;
- verify the exact original substring before mutation.

## 7. Testing layers

### Unit and property-focused tests

Cover scalar utilities, context selection, schemas, validation, state transitions, message parsing, and redaction.

### Deterministic product tests

Compose `MockTextSurface`, `MockInferenceProvider`, and fake clocks. Prove the entire writer loop, races, cancellation, staleness, Apply, Dismiss, and refusals.

### Provider adapter tests

Use controlled fetch and message doubles to prove endpoint, minimal payload, strict schema, timeout, body limit, cancellation, typed failures, and redaction.

### Browser integration tests

Use Playwright persistent Chromium following the official [extension-testing setup](https://playwright.dev/docs/chrome-extensions). Exercise the built unpacked extension, permission lifecycle, real editing surfaces, overlay behavior, focus, IME, and one-step Undo.

### Live provider checks

Use a dedicated spend-limited key only at the Provider Gate. Record the concrete model and latency. Keep credentials and raw private text out of evidence.

### Final smoke

Load the unpacked production build in current Chrome Stable and exercise one clean and one correction path on each supported surface class.

## 8. Test quality

Tests assert product invariants and observable outcomes rather than private implementation shape. Fixtures are synthetic and domain-neutral. Each regression test states the invariant it protects.

Timing tests assert exact boundaries. Stale-work tests control completion order. Undo tests compare complete original text after one browser Undo.

Flaky browser tests are failures. Diagnose the underlying event, focus, permission, or lifecycle contract rather than increasing arbitrary waits.

## 9. Security and privacy

- Request the narrowest exact-origin permission only through writer action.
- Keep the OpenRouter host as the sole required host permission.
- Restrict storage to trusted extension contexts.
- Keep source identity and DOM data inside the content script.
- Send only bounded context required for the current check.
- Redact secrets, headers, raw context, and raw response bodies.
- Keep executable code local.
- Maintain zero telemetry, analytics, and persistent text cache.

Treat browser-profile storage as a disclosed local convenience boundary, not as a secret vault.

## 10. Provider discipline

Use the fixed endpoint and a user-configured concrete structured-output model. Keep the payload minimal and non-streaming, set `provider.require_parameters: true`, and apply strict JSON Schema plus local Zod validation.

Enforce:

```text
one request per current eligible revision
eight-second timeout
32 KiB response limit
best-effort cancellation
zero retry
zero response healing
zero fallback model
```

The adapter copies revision identity; the model never authors it.

## 11. DOM mutation discipline

`document.execCommand("insertText")` is a deliberately isolated, runtime-gated leaf because V0.1 requires a browser undo-aware edit. The product positively supports only fixtures and real surfaces where integration evidence proves one Undo restores exact original text.

No direct-value assignment, DOM rewrite, clipboard path, simulated key path, or fallback mutation is permitted. When the leaf is unavailable or mapping is uncertain, return a typed refusal.

## 12. Dependencies and build

Use one npm package.

Runtime:

- Zod.

Development:

- TypeScript;
- esbuild;
- Vitest;
- Playwright;
- Chrome types;
- Node types.

Plain TypeScript, HTML, and CSS implement options and overlay. Keep React, Vite, Tailwind, extension frameworks, OpenRouter SDKs, monorepo tooling, backends, databases, and code generation outside V0.1.

`scripts/build-extension.mjs` is the single explicit build entry. Review emitted assets for local executable code and minimal permissions.

## 13. Change and commit discipline

Each increment:

- expresses one product invariant or architectural decision;
- adds the smallest implementation and evidence needed;
- runs focused checks before broad checks;
- receives diff and dependency inspection;
- appends factual evidence;
- commits and pushes with verified identity.

The constitution is immutable during implementation. A material product or architecture change creates a new version rather than an evidence-ledger edit.

## 14. Evidence hygiene

Evidence records:

```text
UTC time
gate or increment
commit
commands
exact result
environment
evidence level
limitations
next checkpoint
```

Record what was not tested. Separate compiler proof, deterministic proof, integration proof, live-provider proof, and runtime smoke.

## 15. Maintenance rule

Prefer a small explicit core and browser leaf over generalized frameworks. Remove unused code, scripts, settings, permissions, and dependencies immediately. Maintain one obvious path for build, configuration, messaging, provider fetch, and mutation.

## 16. Deferred engineering

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, packaging, signing, Chrome Web Store publication, release automation, native placeholders, and cross-OS runtime matrices are deferred to separately versioned objectives.
