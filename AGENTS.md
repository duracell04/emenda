# Emenda Agent Guide

> **Frozen agent governance, version 2.0.0**

Emenda is built from repository-local sources of truth.

## Reading order

1. `PROMPT.md`
2. `AGENTS.md`
3. `SPEC.md`
4. `docs/ARCHITECTURE.md`
5. `ROADMAP.md`
6. `docs/IMPLEMENTATION-PLAN.md`
7. `docs/ACCEPTANCE.md`
8. `docs/ENGINEERING.md`
9. `UX.md`
10. `BRAND.md`
11. `README.md`

## Principal objective

Build the smallest complete Emenda V0.1 as one strict-TypeScript product core and one Chromium Manifest V3 extension. Own the full outcome as one objective. Gates are verification checkpoints, not new authorization boundaries.

## Constitutional authority

The source-of-truth order is:

```text
PROMPT.md
→ AGENTS.md
→ SPEC.md
→ docs/ARCHITECTURE.md
→ ROADMAP.md
→ docs/IMPLEMENTATION-PLAN.md
→ docs/ACCEPTANCE.md
→ docs/ENGINEERING.md
→ UX.md
→ BRAND.md
→ README.md
```

`docs/EVIDENCE.md` records facts only. It cannot change the constitution.

## Active architecture

- Shared product behavior lives in `core/`.
- `core/` compiles without DOM, Chrome, Node, React, or extension types.
- Browser mechanisms live in `extension/`.
- The content script owns controller state, revision lifetime, `BrowserTextSurface`, and the shadow-root overlay.
- The service worker owns permissions, trusted settings, cancellation, and the fixed OpenRouter fetch.
- Source identity and raw DOM data remain in the content script.
- Runtime messages are versioned and strictly validated.
- The package is one npm package, not a monorepo.

## Active-gate rule

State the active gate before implementation work:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

Classify every failure by the gate and subsystem that own it. A later-gate failure preserves earlier verified evidence unless the underlying invariant changed.

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

This sequence is binding. Presentation and accessibility evidence is gathered at Browser Integration.

## Increment rule

Implement one independently verifiable invariant or architectural decision at a time:

```text
inspect
→ implement
→ verify
→ inspect diff
→ update factual evidence
→ commit
→ push
→ verify pushed state
→ continue
```

Use fake clocks and deterministic mocks before browser integration. Keep every commit attributable to one decision.

## Authority and staleness

- Each eligible committed input reserves a new `RevisionId` synchronously.
- Composition input invalidates current work immediately; inference waits for `compositionend`.
- A newer revision cancels older work best-effort and always wins authoritatively.
- An Apply command accepts only the current `SuggestionId`.
- Stale results and stale commands are silent and cannot mutate the page.

## Dependency rule

Direct runtime dependencies are limited to Zod. Development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, and Chrome/Node types.

Every dependency, abstraction, script, permission, and build output must serve a current V0.1 requirement. Prefer deletion, explicit code, and platform capabilities over new machinery.

## Safety rule

The browser binding performs replacement only after verifying current revision, the same connected writable source, the same document and opaque snapshot, exact current logical text, lossless range mapping, and the exact original substring. Its only mutation leaf is a runtime-gated `document.execCommand("insertText")` operation that produces one browser undo step.

Unsupported or ambiguous surfaces fail closed. No direct-value assignment, DOM rewrite, clipboard operation, simulated key input, fuzzy matching, or unique-match recovery is permitted.

## Evidence rule

Report precisely:

```text
what compiled
what ran deterministically
what ran in persistent Chromium
what was verified live
what was inspected only
what remains unsupported or unverified
```

Use exact evidence levels: `inspected`, `compiled`, `deterministic`, `integration`, `live`, and `runtime`.

Keep secrets and raw text out of logs, snapshots, fixtures, commits, and error reports. Use synthetic domain-neutral test text.

## Documentation rule

After the Documentation Gate, initialize the mutable `docs/EVIDENCE.md` ledger with the frozen constitution commit, environment facts, and validation results. Append evidence; preserve failures and later recoveries as separate entries.

Changing product behavior, architecture, UX, brand, acceptance requirements, or agent governance requires a new versioned constitution and new checksums.

## Deferred-work rule

Native hosts, Tauri, Rust, accessibility APIs, native credential stores, packaging, signing, store publication, release automation, native placeholders, and cross-OS runtime claims are outside V0.1. Do not scaffold them. Native work requires browser-usage evidence and a separately versioned objective.

## Stop rule

Continue automatically through every active gate. Stop after the V0.1 Conformance Gate passes, the final verified commit is pushed, and the worktree is clean. If a genuine blocker remains after safe in-scope alternatives are exhausted, record the preserved state, evidence, exact blocker, and external authority required.
