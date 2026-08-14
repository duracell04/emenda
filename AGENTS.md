# Emenda Agent Guide

> **Frozen agent governance, version 1.0.1**

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

Build the smallest complete writing assistant that preserves the author's Duktus and keeps shared product behavior independent from every operating system.

Own V0.1 as one autonomous objective. Complete every checkpoint in sequence without treating an intermediate gate as a new authorization boundary.

## Non-negotiable architecture

The current host operating system is evidence context only.

Shared product code uses platform-neutral semantic types and application-owned ports.

Platform APIs, identifiers, timing assumptions, accessibility objects, input simulation, clipboard mechanics, and focus strategies live exclusively inside leaf bindings.

The shared product must pass against `MockTextSurface` before any native binding is introduced.

## Active-gate rule

State the active gate before beginning work:

```text
Documentation Gate
→ Mock Product Gate
→ Provider Gate
→ Presentation Gate
→ Architecture Gate
→ Current-Host Binding Gate
→ V0.1 Conformance Gate
```

Classify every failure by the gate and subsystem that own it.

A later-gate failure preserves the verified status of earlier gates.

Passing a gate advances the same objective to the next checkpoint. Release is a later explicitly named objective.

## Canonical implementation sequence

```text
documentation baseline + Documentation Gate
→ domain
→ TextSurface
→ MockTextSurface
→ InferenceProvider + MockInferenceProvider
→ controller, debounce, context, and revision
→ validator + presentation state
→ complete mock product + Mock Product Gate
→ OpenRouterProvider + Provider Gate
→ Tauri UI + Presentation Gate
→ Architecture Gate
→ current-host leaf + Current-Host Binding Gate
→ two-app runtime + V0.1 Conformance Gate
```

## Increment rule

Implement one independently verifiable product invariant or architectural decision at a time.

```text
inspect
→ implement
→ verify
→ inspect diff
→ commit
→ push
→ verify pushed state
→ continue
```

A commit may touch several files when those files jointly express one decision.

## Complexity rule

Every active dependency, feature flag, script, build mode, abstraction, and test harness earns its existence through a current product requirement or a necessary invariant.

Use deletion and deferral before adding machinery.

## Evidence rule

Report precisely:

```text
what compiled
what ran
what passed
what was inspected only
what remains unverified
```

Use precise terms:

```text
implemented
compiled
deterministically tested
integration tested
runtime verified
supported
distribution-ready
```

## Documentation rule

Keep the supplied frozen constitution immutable during implementation.

After baseline verification, initialize the supplied `docs/EVIDENCE.md` ledger and update it when factual status or gate evidence changes. This mutable implementation ledger is excluded from the frozen constitution and its checksums.

Preserve the frozen principles and contracts. A material constitutional change creates a newly versioned documentation package rather than a silent in-place reinterpretation.

## Stop rule

When an intermediate gate's definition of done is satisfied:

```text
verify
→ document factual evidence
→ commit
→ push
→ verify pushed state
→ continue to the next gate
```

Stop only when the top-level V0.1 objective and V0.1 Conformance Gate pass.

If completion is genuinely blocked, exhaust safe in-scope work and alternatives before reporting the precise blocker, preserved state, evidence, and authority or external change required. Distribution and Release begin under a new explicitly named objective.
