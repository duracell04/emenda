# Emenda Agent Guide

> **Frozen agent governance, version 2.0.2**

Emenda is governed by repository-local documentation. The existence of this package does not authorize product implementation.

## Objective boundary

The active v2.0.2 objective is documentation only. Preserve v2.0.1 at Git commit `d70b277998a23663ee6befc77dd6bb0da50ebcca`, create one documentation-only child commit containing the 13 Markdown files, verify it, hash it, push it, confirm the remote commit and a clean worktree, then stop. Implementation requires a separate future objective.

When that future objective is explicitly supplied, own the complete V0.1 outcome through the seven increments and six gates defined in [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md) and [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md).

## Reading order

1. `PROMPT.md`
2. `AGENTS.md`
3. `SPEC.md`
4. `docs/ARCHITECTURE.md`
5. `docs/IMPLEMENTATION-PLAN.md`
6. `docs/ACCEPTANCE.md`
7. `docs/ENGINEERING.md`
8. `UX.md`
9. `ROADMAP.md`
10. `BRAND.md`
11. `README.md`

`docs/EVIDENCE.md` records facts only. `PACKAGE-MANIFEST.md` identifies the freeze and its checksums.

## Subject authority

- `SPEC.md` controls product behavior, safety, compatibility, failures, and the canonical provider prompt, schema, and constants.
- `docs/ARCHITECTURE.md` controls ownership, dependency direction, and runtime boundaries, subject to the specification.
- `docs/IMPLEMENTATION-PLAN.md` controls build order and gate placement, subject to both.
- `docs/ACCEPTANCE.md` controls the gate criteria and canonical live-provider corpus, subject to those authorities.
- Supporting documents verify or summarize these authorities and cannot override them.

## Active-gate discipline

State the active gate before future implementation work. The six gate names and order are:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

Classify a failure by its owning gate and subsystem. Later-gate failure preserves earlier evidence unless the tested invariant changed. Gates do not expand scope or authority.

## Operating rules

- Inspect before changing and verify in proportion to risk.
- Use deterministic mocks and fake clocks before browser integration.
- Keep commits coherent and attributable to meaningful decisions or increments; do not require one commit per helper or component.
- Inspect every diff and dependency change before committing.
- Push and verify remote identity at required checkpoints.
- Preserve unrelated and ignored workspace state.
- Record failures and later recoveries as separate factual evidence.
- Implement observable contracts exactly, while choosing the simplest internal technique when equivalent implementations do not change behavior, safety, privacy, compatibility, or reliability.

## Locked boundaries

- Core product behavior is strict TypeScript without DOM, Chrome, Node, React, or extension types.
- One pure reducer owns product state; effects own timers and external I/O.
- Zod is confined to the declared model, protocol, and trusted-settings boundaries.
- The service worker owns permissions, trusted settings, cancellation, and OpenRouter traffic.
- The content script owns page text, source identity, controller state, DOM mapping, and presentation.
- Controller revision authority and surface mutation safety remain separate.
- Stale work is silent and cannot mutate page text.
- Unsupported or ambiguous surfaces fail closed.

## Dependency and scope rule

The future product is one npm package. The only permitted direct runtime dependency is Zod. Development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, and Chrome/Node types.

Do not add native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, packaging, signing, store publication, release automation, commercial infrastructure, or placeholders for deferred runtimes.

## Evidence rule

Use the exact levels `inspected`, `compiled`, `deterministic`, `integration`, `live`, and `runtime`. Report what ran, the already-existing implementation tree and commit tested, exact environment, failures, limitations, and what remains unverified.

Keep credentials, raw private text, URLs, source identity, DOM structures, authorization headers, and raw provider bodies out of logs, fixtures, snapshots, commits, and evidence.

## Constitution changes

Product, architecture, UX, acceptance, implementation-order, brand, or governance changes require a new versioned documentation freeze and new staged checksums. The evidence ledger cannot amend the constitution.

## Stop rules

- Present objective: stop after the verified v2.0.2 documentation commit is pushed and the worktree is clean.
- Future implementation objective: stop after V0.1 Conformance passes, the tested implementation commit is pushed and verified, and the worktree is clean.
