# Emenda Roadmap

> **Frozen product roadmap, version 2.0.0**

## Roadmap rule

V0.1 proves one complete browser correction loop before expanding product breadth. Gates measure evidence inside one objective. They are not releases and do not authorize parallel feature tracks.

## Active V0.1 sequence

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

Active gates:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

## Milestone 0: frozen baseline

Outcome:

- the 13-file Markdown package is coherent and versioned 2.0.0;
- freeze ID is `emenda-clean-room-v2.0.0-2026-08-14`;
- version 1.0.1 remains at `d3192b7`;
- 11 immutable documents match staged Git-blob checksums;
- the evidence ledger is an empty mutable template;
- no implementation source is present.

Gate: Documentation.

## Milestone 1: strict-TypeScript semantics

Outcome:

- immutable domain values and typed failures;
- strict Zod boundary schemas;
- scalar-range utilities;
- `TextSurface`, `InferenceProvider`, and minimal scheduler seams;
- deterministic mocks;
- no runtime-specific core types.

This milestone creates foundations, not a partial product claim.

## Milestone 2: complete mock product

Outcome:

- immediate revision authority;
- exact 600 ms trailing-edge debounce;
- composition invalidation;
- sentence focus and paragraph/window context;
- one-request current-revision policy;
- cancellation and stale silence;
- strict correction validation;
- presentation state;
- Apply and Dismiss through mocks;
- complete deterministic product loop.

Gate: Mock Product.

## Milestone 3: architecture proof

Outcome:

- `core/` compiles without DOM, Chrome, Node, React, or extension types;
- dependency and import direction is enforced;
- top-level package shape and dependency allowlist are satisfied;
- browser and message concerns remain outside the core.

Gate: Architecture.

## Milestone 4: browser shell and provider

Outcome:

- `BrowserTextSurface` supports lossless capture and refusal;
- MV3 worker manages exact-origin permissions and one dynamic registration;
- options page stores the key and concrete model in trusted extension storage;
- fixed shadow-root overlay presents display-safe state;
- versioned messages validate strictly;
- `OpenRouterProvider` enforces endpoint, schema, timeout, body limit, cancellation, and redaction;
- live checks establish one correction and one clean result for every supported profile plus unsupported-language behavior.

Gate: Provider. Presentation evidence remains pending until integrated browser use.

## Milestone 5: browser integration

Outcome:

- textarea capture, suggestion, Apply, Dismiss, and one-step Undo;
- conventional contenteditable capture, suggestion, Apply, Dismiss, and one-step Undo;
- IME, stale-result, changed-source, focus, permission, and storage behavior;
- accessible overlay, keyboard actions, visible focus, reduced motion, and WCAG 2.2 AA styling;
- unsupported surfaces fail closed.

Gate: Browser Integration.

## Milestone 6: V0.1 conformance

Outcome:

- all earlier gate evidence remains current;
- persistent-Chromium suite passes from the unpacked build;
- live provider evidence identifies concrete model and latency;
- current Chrome Stable unpacked-extension smoke passes;
- dependency and permission inventories are final;
- limitations and environment facts are recorded;
- final commit is pushed and verified;
- worktree is clean.

Gate: V0.1 Conformance. Then stop.

## Post-V0.1 evidence horizon

Real browser use may justify later, separately versioned objectives for broader browser-surface compatibility, interaction refinements, release automation, or store publication. Each addition requires measured benefit and explicit acceptance evidence.

Native hosts, Tauri, Rust, operating-system accessibility APIs, credential vaults, packaging, signing, native placeholders, and cross-OS runtime claims remain deferred until browser usage demonstrates a material unmet need.
