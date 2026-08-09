# Emenda Agent Guide

> **Frozen agent governance, version 1.0.0**

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

## Non-negotiable architecture

The current host operating system is evidence context only.

Shared product code uses platform-neutral semantic types and application-owned ports.

Platform APIs, identifiers, timing assumptions, accessibility objects, input simulation, clipboard mechanics, and focus strategies live exclusively inside leaf bindings.

The shared product must pass against `MockTextSurface` before any native binding is introduced.

## Active-gate rule

State the active gate before beginning work:

```text
Mock Product
Provider
Presentation
Architecture
Current-Host Binding
V0.1 Conformance
Release
```

Classify every failure by the gate and subsystem that own it.

A later-gate failure preserves the verified status of earlier gates.

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

Update factual status when evidence changes.

Preserve the frozen principles and contracts. A material constitutional change creates a new documentation-package version rather than a silent in-place reinterpretation.

## Stop rule

When the active gate's definition of done is satisfied:

```text
verify
→ document factual evidence
→ commit
→ push
→ report
→ stop
```

Further work begins under a new explicitly named objective.
