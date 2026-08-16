# Emenda V0.1

> **Frozen clean-room constitution, version 2.0.2**

This repository contains the documentation-only constitution for Emenda V0.1. It does not authorize implementation by itself.

## Current objective

The v2.0.2 objective ends when the 13 Markdown documents are rewritten, verified, hashed, committed, pushed, and the worktree is clean. Do not create implementation files during this objective. Building the product requires a separate future objective in the separate implementation repository.

Version 2.0.1 remains preserved at Git commit `d70b277998a23663ee6befc77dd6bb0da50ebcca`. Version 2.0.0 remains preserved at Git commit `a1a13607867db8e6eb2ea904f6387ba130f22ce7`.

## Future product objective

When separately authorized, build the smallest complete Emenda V0.1 as:

- one strict-TypeScript product core;
- one Chromium Manifest V3 extension requiring Chrome 140 or newer;
- one bounded correction at a time through the default `openrouter/free` route or an advanced concrete-model override;
- one locally derived Unicode-scalar edit from the model's complete corrected focus;
- explicit writer approval before any verified, one-step-undoable edit.

Emenda preserves the writer's meaning, terminology, register, rhythm, and Duktus. It performs no translation, telemetry, persistent text logging, automatic rewriting, or unsupported-surface fallback.

## Authority

Read the complete package before implementation. The three subject authorities are:

1. [`SPEC.md`](SPEC.md) — product behavior, safety, compatibility, and failures.
2. [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — ownership, boundaries, and dependency direction.
3. [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md) — implementation order and gate placement.

Supporting documents may summarize or verify these authorities but cannot change them. [`SPEC.md`](SPEC.md) is the sole canonical home of the model prompt, provider schemas, limits, and provider contract. [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md) is the sole canonical home of the live qualification corpus. [`AGENTS.md`](AGENTS.md) defines repository operating constraints.

## Canonical future sequence

The Documentation Gate is a prerequisite. It is not an implementation increment.

```text
Documentation baseline + Documentation Gate
→ Increment 1: Pure Core & Simulation
→ Increment 2: Unified State Machine
→ Increment 3: Mock Product + Architecture Gates
→ Increment 4: Unified DOM Integration
→ Increment 5: MV3 Shell + Provider
→ Increment 6: Browser Integration
→ Increment 7: V0.1 Conformance
→ stop
```

The six gates remain separate:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

Gates are evidence checkpoints within a future implementation objective, not new authorization boundaries.

## Hard limits

- Shared behavior lives in browser-independent strict TypeScript.
- Browser authority remains in the extension leaves.
- The service worker alone owns trusted settings, credentials, and OpenRouter traffic.
- The content script alone owns page text, source identity, controller state, DOM mapping, and the overlay.
- Apply requires current controller authority and complete surface verification.
- Unsupported or ambiguous conditions fail closed.
- Native runtimes, release packaging, signing, store publication, and commercial expansion are deferred.

## Completion rules

The present documentation objective stops after its verified documentation-only commit is pushed. A future implementation objective stops after all seven increments and six gates pass, its final tested commit is pushed and verified, and the worktree is clean.

The constitution resolves all product, safety, architecture, and acceptance decisions. The implementation agent retains ordinary discretion over local naming and code organization within the locked boundaries.
