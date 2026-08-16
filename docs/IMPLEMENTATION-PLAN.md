# Emenda V0.1 Implementation Plan

> **Frozen implementation plan, version 2.0.2**

## 1. Objective boundary

The current objective is documentation only: preserve v2.0.1 at `d70b277998a23663ee6befc77dd6bb0da50ebcca`, rewrite, verify, hash, commit, and push the 13 Markdown files as its direct-child v2.0.2 freeze, then stop. It creates no implementation source. Building V0.1 requires a separate future objective that identifies the implementation repository, baseline, branch, and draft-PR target.

[`SPEC.md`](../SPEC.md) owns product behavior, [`ARCHITECTURE.md`](ARCHITECTURE.md) owns boundaries and responsibility, and this document owns build order. The Documentation Gate establishes the prerequisite baseline; it is not an implementation increment.

## 2. Canonical sequence

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

The sequence is binding for the future implementation objective. The six gates remain distinct verification checkpoints: Documentation, Mock Product, Architecture, Provider, Browser Integration, and V0.1 Conformance.

## 3. Documentation baseline and Documentation Gate

The present objective completes only this baseline:

- preserve v2.0.0 at `a1a13607867db8e6eb2ea904f6387ba130f22ce7` and v2.0.1 at `d70b277998a23663ee6befc77dd6bb0da50ebcca`, then create one documentation-only direct child of v2.0.1;
- freeze exactly 13 tracked Markdown files as `emenda-clean-room-v2.0.2-2026-08-16`;
- verify version and freeze identity, source-of-truth ownership, local links, and byte-identical canonical-sequence occurrences;
- calculate and verify the 11 immutable documents' individual SHA-256 values from their staged Git blobs;
- leave [`EVIDENCE.md`](EVIDENCE.md) as an empty template and place the inventory and checksums in [`PACKAGE-MANIFEST.md`](../PACKAGE-MANIFEST.md);
- inspect the documentation-only diff and pass `git diff --check`;
- commit and push the one documentation decision, verify direct v2.0.1 parentage and preserved v2.0.0 ancestry, and confirm a clean worktree.

Passing this gate ends the current objective. No package, script, extension asset, or implementation file is created during the freeze.

## 4. Seven future implementation increments

### Increment 1: Pure Core & Simulation

Establish one npm package, strict TypeScript domain values, typed failures, deterministic Unicode-scalar and text rules, the model-facing schemas, local one-hunk correction derivation, semantic ports, a minimal scheduler seam, and deterministic surface and provider simulations. Keep browser/runtime types out of the core and keep Zod at the permitted trust boundaries.

Verify scalar behavior, context selection, provider outcomes, cancellation races, surface changes, replacement acknowledgements, and refusals before building controller behavior.

### Increment 2: Unified State Machine

Implement one pure reducer for revisions, debounce, checking, validation, suggestions, Apply, Dismiss, and errors. Effects perform timers, capture, inference, storage, messaging, and mutation.

Prove immediate revision authority, exact trailing debounce, cached-settings authority, settings-revision invalidation, composition-end commit semantics, terminal-input deduplication, bounded context and focus, strict external validation, deterministic local scalar derivation, stale silence, and the one-shot self-mutation contract under fake clocks.

### Increment 3: Mock Product + Architecture Gates

Compose the reducer, effects, and simulations to prove the complete writer loop through suggestion, Apply or Dismiss, post-edit authority, refusal, cancellation, and error recovery without browser APIs.

Pass the **Mock Product Gate** first. Then pass the separate **Architecture Gate**, limited to strict core compilation, absence of browser/runtime types, permitted Zod placement, semantic ports, import direction, the dependency allowlist, and absence of native scaffolding.

### Increment 4: Unified DOM Integration

Implement one `BrowserTextSurface` for textarea and the constrained contenteditable grammar. Prove exact scalar/UTF-16 conversion, recorded DOM source spans, collapsed-whitespace replacement spans, deterministic `<br>` and block-boundary newlines, boundary round trips, snapshot and source refusal, and fail-closed unsupported DOM handling.

Integrate centralized IME suppression, one-shot self-authored input consumption, runtime-gated `document.execCommand("insertText")`, safe Apply, and exact one-step browser Undo.

### Increment 5: MV3 Shell + Provider

Build the Chrome 140 Manifest V3 shell: trusted worker settings, validated protocol, exact-origin activation, zero-or-one dynamic registration, immediate idempotent injection, revocation teardown, options messaging, and the accessible fixed shadow overlay.

Add the OpenRouter adapter using the canonical default route and advanced concrete-model override, split model-facing context, strict structured output, explicit within-request provider fallback, zero application retries, bounded generation and response reading, cancellation, local derivation, and redacted failures. Pass the **Provider Gate**, including deterministic message and transport enforcement plus the single sequential live qualification corpus defined in [`ACCEPTANCE.md`](ACCEPTANCE.md).

### Increment 6: Browser Integration

Integrate textarea and constrained contenteditable flows in the built unpacked extension. Exercise permission grant and revocation, trusted-storage isolation, worker lifecycle, settings updates, typing, IME, staleness, suggestion, Apply, Dismiss, refusal, teardown, focus, keyboard access, reduced motion, and one-step Undo.

Pass the **Browser Integration Gate**, which owns the manifest, permissions, registrations, storage isolation, browser behavior, and overlay accessibility evidence.

### Increment 7: V0.1 Conformance

Run the complete deterministic and browser suites, inspect the production bundle, dependencies, permissions, secret and text leakage, and known limitations, then complete the required runtime and personal-device evidence.

Pass the **V0.1 Conformance Gate**, append factual evidence, push the final implementation commit, verify remote identity and a clean worktree, and stop. Any branch or draft-PR action follows the separately supplied implementation objective. Distribution and deferred work require another objective.

## 5. Future execution policy

Implementation commits should each express a coherent decision; there is no mandated commit per component, helper, or file. Every gate must be evaluated against [`ACCEPTANCE.md`](ACCEPTANCE.md), and factual results belong only in the mutable evidence ledger.

The future implementation provides the cross-platform audit capability specified in [`ENGINEERING.md`](ENGINEERING.md). Its filename, path, internal organization, and output format remain implementation choices.

The constitution resolves all product, safety, architecture, and acceptance decisions. The implementation agent retains ordinary discretion over local naming and code organization within the locked boundaries.
