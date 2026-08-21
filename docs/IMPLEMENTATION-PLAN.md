# Emenda V0.1 Implementation Plan

> **Frozen implementation plan, version 2.1.1**

## 1. Objective boundary

[`PROMPT.md`](../PROMPT.md) owns objective authorization and completion. This document orders work after the separate human authorization defined there and grants no implementation authorization by itself.

[`SPEC.md`](../SPEC.md) owns product behavior, [`ARCHITECTURE.md`](ARCHITECTURE.md) owns boundaries and responsibility, and this document owns build order. [`PACKAGE-MANIFEST.md`](../PACKAGE-MANIFEST.md) owns documentation-freeze identity and lineage. The Documentation Gate establishes the prerequisite baseline; it is not an implementation increment.

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

[`PACKAGE-MANIFEST.md`](../PACKAGE-MANIFEST.md) defines the exact v2.1.1 baseline, lineage, classification, and integrity data. The Documentation Gate in [`ACCEPTANCE.md`](ACCEPTANCE.md) verifies that candidate. Passing it completes the documentation objective; product increments begin only under the separate authorization described above.

### Future implementation intake

Before product source mutation, the future implementation repository copies every path in this freeze into `constitution/` with paths preserved. It commits one strict `constitution.lock.json` object with no extra properties and exactly these five fields: `schemaVersion` is the integer `1`; `repository` is the string `https://github.com/duracell04/emenda`; `freezeId` is the exact manifest freeze-ID string; and `commit` and `tree` are the corresponding 40-character lowercase hexadecimal Git object IDs. Document paths and individual hashes come from the copied `constitution/PACKAGE-MANIFEST.md` and are not duplicated in the lock. The implementation audit verifies the lock, inventory, byte identity, and every manifest checksum locally without network access. That snapshot remains read-only for the implementation objective.

## 4. Seven future implementation increments

### Increment 1: Pure Core & Simulation

Establish one npm package, select and commit the one exact Node/npm/TypeScript toolchain tuple, exact direct dependency versions, package-manager and engine metadata, and its npm lockfile. Add strict TypeScript domain values, typed failures, deterministic Unicode-scalar and text rules, the canonical provider schema, local one-hunk correction derivation, semantic ports, a minimal scheduler seam, and deterministic surface and provider simulations. Keep browser/runtime types out of the core and keep Zod at the permitted trust boundaries.

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

Commit the final implementation tree, test that exact commit, then push and verify it in the implementation repository. Return to this constitution repository and create a later commit whose sole file change appends the factual result to `docs/EVIDENCE.md`, naming that already-existing tested tree and commit while every frozen file remains unchanged. Push and verify the evidence commit, confirm both tracked worktrees are clean, declare the **V0.1 Conformance Gate** passed, and stop. Any branch or draft-PR action follows the separately supplied implementation objective. Distribution and Deferred work require another objective.

## 5. Future execution policy

Repository and commit discipline follows [`AGENTS.md`](../AGENTS.md). Every gate is evaluated against [`ACCEPTANCE.md`](ACCEPTANCE.md), and factual results belong only in the mutable evidence ledger.

The future implementation provides the cross-platform audit capability specified in [`ENGINEERING.md`](ENGINEERING.md). Its filename, internal organization, and output format remain builder choices; its coverage and clean-checkout behavior are required.

Builder discretion follows [`AGENTS.md`](../AGENTS.md) and [`ENGINEERING.md`](ENGINEERING.md) within the frozen product and architecture contracts.
