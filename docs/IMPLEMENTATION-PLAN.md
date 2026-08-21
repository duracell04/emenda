# Emenda V0.1 Implementation Plan

> **Frozen implementation plan, version 2.0.3**

## 1. Objective boundary

The current objective is documentation only: preserve v2.0.2 at `6a4ddc65fa9067f94023f87aebe48840e1b88bc2`, rewrite, verify, hash, commit, and push the 13 Markdown files as its direct-child v2.0.3 freeze, then stop. It creates no implementation source. Building V0.1 requires a separate future objective that identifies the implementation repository, baseline, branch, and draft-PR target.

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

- preserve v2.0.1 at `d70b277998a23663ee6befc77dd6bb0da50ebcca` and v2.0.2 at `6a4ddc65fa9067f94023f87aebe48840e1b88bc2`, then create one documentation-only direct child of v2.0.2;
- freeze exactly 13 tracked Markdown files as `emenda-clean-room-v2.0.3-2026-08-21`;
- verify version and freeze identity, source-of-truth ownership, local links, and byte-identical canonical-sequence occurrences;
- calculate and verify the 11 immutable documents' individual SHA-256 values from their staged Git blobs;
- leave [`EVIDENCE.md`](EVIDENCE.md) as an empty template and place the inventory and checksums in [`PACKAGE-MANIFEST.md`](../PACKAGE-MANIFEST.md);
- inspect the documentation-only diff and pass `git diff --check`;
- commit and push the one documentation decision, verify direct v2.0.2 parentage and preserved v2.0.1 ancestry, and confirm a clean worktree.

Passing this gate ends the current objective. No package, script, extension asset, or implementation file is created during the freeze.

## 4. Seven future implementation increments

### Increment 1: Pure Core & Simulation

Establish one npm package with exact direct versions and a committed lockfile, strict TypeScript domain values, typed failures, deterministic Unicode-scalar and text rules, the canonical provider schema, local one-hunk correction derivation, semantic ports, a minimal scheduler seam, and deterministic surface and provider simulations. Keep browser/runtime types out of the core and keep Zod at the permitted trust boundaries.

Verify scalar behavior, context selection, provider outcomes, cancellation races, surface changes, replacement acknowledgements, and refusals before building controller behavior.

### Increment 2: Unified State Machine

Implement one pure reducer for revisions, debounce, checking, validation, suggestions, Apply, Dismiss, and errors. Effects perform timers, capture, inference, storage, messaging, and mutation.

Prove immediate revision authority, exact trailing debounce, cached-settings authority and resynchronization, composition baseline and exact terminal-input identity, bounded context and focus, malformed-Unicode refusal, strict external validation, deterministic local scalar derivation, stale silence, and the one-shot self-mutation contract under fake clocks.

### Increment 3: Mock Product + Architecture Gates

Compose the reducer, effects, and simulations to prove the complete writer loop through suggestion, Apply or Dismiss, post-edit authority, refusal, cancellation, and error recovery without browser APIs.

Pass the **Mock Product Gate** first. Then pass the separate **Architecture Gate**, limited to strict core compilation, absence of browser/runtime types, permitted Zod placement, semantic ports, import direction, the dependency allowlist, and absence of native scaffolding.

### Increment 4: Unified DOM Integration

Implement one `BrowserTextSurface` for a visible, foreground, midpoint-exposed, writable, sequentially keyboard-focusable light-DOM textarea. Prove paired trusted input/IME provenance, baseline-only unpaired changes, transient lossless noncollapsed IME candidate ranges between collapsed start/end states, exact scalar/UTF-16 conversion, value and ordinary collapsed-selection snapshots, caret and foreground invalidation, controlled approval focus handoff, scoped correction-range selection, boundary round trips, source refusal without reading rejected editors, and fail-closed unsupported-surface handling.

Integrate centralized IME suppression, one-shot self-authored input consumption, runtime-gated `document.execCommand("insertText", false, replacement)`, safe Apply, and exact one-step browser Undo.

### Increment 5: MV3 Shell + Provider

Build the Chrome 140 Manifest V3 shell: trusted worker settings, synchronous listener registration and Chrome-140 response bridging, validated one-shot protocol, sender and immediate pre-Apply authorization, serialized explicit-port origin lifecycle and recovery, external permission reconciliation, zero-or-one dynamic registration, idempotent activation, BFCache/prerender teardown and reauthorization, options messaging, and the accessible text-only shadow overlay.

Add the OpenRouter adapter using the required writer-configured base model ID, canonical JSON serialization and schema, explicit within-request provider fallback, zero application retries, fixed generation and response bounds, cancellation, local derivation, and redacted failures. Pass the **Provider Gate**, including deterministic message and transport enforcement plus a successful sequential live qualification corpus defined in [`ACCEPTANCE.md`](ACCEPTANCE.md).

### Increment 6: Browser Integration

Integrate the supported textarea flow in the built unpacked extension. Exercise refused editor classes, exact-port permission grant, external addition and revocation, trusted-storage isolation, worker restart, navigation and document lifecycle, settings updates, typing, IME, staleness, suggestion, authorized Apply, Dismiss, refusal, teardown, focus, safe text rendering, keyboard access, reduced motion, and one-step Undo.

Pass the **Browser Integration Gate**, which owns the manifest, permissions, registrations, storage isolation, browser behavior, and overlay accessibility evidence.

### Increment 7: V0.1 Conformance

Run the complete deterministic and browser suites, inspect the production bundle, dependencies, permissions, secret and text leakage, and known limitations, then complete the required runtime and personal-device evidence.

Commit the final implementation tree, test that exact commit, then push and verify it. Create a later documentation-only evidence commit that names the tested tree and commit, push and verify that evidence commit, confirm a clean worktree, declare the **V0.1 Conformance Gate** passed, and stop. Any branch or draft-PR action follows the separately supplied implementation objective. Distribution and deferred work require another objective.

## 5. Future execution policy

Implementation commits should each express a coherent decision; there is no mandated commit per component, helper, or file. Every gate must be evaluated against [`ACCEPTANCE.md`](ACCEPTANCE.md), and factual results belong only in the mutable evidence ledger.

The future implementation provides the cross-platform audit capability specified in [`ENGINEERING.md`](ENGINEERING.md). Its filename, path, internal organization, and output format remain implementation choices.

The constitution resolves all product, safety, architecture, and acceptance decisions. The implementation agent retains ordinary discretion over local naming and code organization within the locked boundaries.
