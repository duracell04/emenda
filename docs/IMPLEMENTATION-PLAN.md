# Emenda V0.2 Implementation Plan

> **implementation plan, version 2.3.1**

## 1. Objective boundary

[PROMPT](../PROMPT.md) owns authorization and completion; this document owns successor sequence and gate placement subject to [SPEC](../SPEC.md) and [ARCHITECTURE](ARCHITECTURE.md). [PACKAGE-MANIFEST](../PACKAGE-MANIFEST.md) owns version and source provenance. Documentation validates each authorized revision; PROMPT defines the current Sprint 4 scope.

## 2. Canonical sequence

```text
Documentation baseline + Documentation Gate
→ Sprint 3: Resource-aware deep-operation core (Mock Product → Architecture → Provider)
→ Sprint 4: Proofread interaction and browser integration (Browser Integration)
→ Sprint 5: Real-Mac acceptance and release (V0.2 Conformance)
→ stop
```

Six gates remain in order: Documentation → Mock Product → Architecture → Provider → Browser Integration → V0.2 Conformance. This successor continuation replaces the predecessor greenfield seven-increment prescription for V0.2. Prior increments remain historical evidence in preserved ancestry, not a second active sequence. Reuse unchanged foundations; reverify every affected invariant at its owning gate.

## 3. Documentation baseline and implementation intake

The specification is revised through ordinary authorized versioned commits. Documentation verifies inventory, links, traceability, preserved corpus/contracts, reviewed delta, history and publication.

Import every raw path/byte from the exact published source commit into `constitution/`, including its historical ledger snapshot. `constitution.source.json` is one strict five-field object: `schemaVersion: 1`, `repository: "https://github.com/duracell04/emenda"`, `version: "2.3.1"`, and the corresponding forty-character lowercase hexadecimal `commit` and `tree`. Audit verifies the imported raw Git tree, inventory, links and twenty-three critical-ID mappings without network. Future authorized revisions update the source record and imported documents together; later blueprint ledger appends do not change an earlier import.

Sprint 4 descends from `822d1fcbf759e80a8b4251e8ca12e0200ab5fb9c` / tree `d8c4586bc3084c8d95d7da12012aaed1e4b46419` on `build/v0.2`. Preserve predecessors, installed bytes, historical intakes and qualification artifacts. Import the published revision and update governance/audit provenance before product changes. Keep settings schema 2 and the canonical toolchain/dependencies.

## 4. Sprint 3: Resource-aware deep-operation core

Keep the pure content reducer and cheap settling timer source-local. Implement immediate revision/presentation invalidation, provider-free trusted typing/IME, local 600 ms bookkeeping, explicit intent capture, unchanged bounded focus/context and independent controller timing. Extend strict one-shot protocol 2 with the one-use intent/source-correlation/operation-token contract and typed Busy/unavailable outcomes. Core policy uses semantic ports and no DOM/Chrome/Node types.

Implement one worker operation registry for admission, exclusive bounded lease, exact identity/coalescing, zero backlog, four-entry/60-second/64-KiB transient reuse, completion-order eviction and fresh authority. Separate presentation retirement from provider completion knowledge. Implement marker-before-POST persistence, token-checked transitions, text-free restart state, capability-driven local status inspection and bounded remote uncertainty. The existing origin/settings FIFO retains its responsibility. Provider payload construction remains unchanged.

Pass Mock Product, then Architecture, then Provider. Use fake clocks, controlled messaging/transports, synthetic surface/provider simulations, cross-tab/diagnostic races, cache boundaries, restart/uncertainty and late-response fencing. Replace automatic-debounce assertions rather than keeping obsolete behavior. Prove qualified model-facing equivalence under Acceptance Section 6.3 and inherit the sealed 15/15 run only when that proof passes. No exploratory live model testing belongs to this sprint.

Deliver tested coherent implementation commits and a provider/concurrency/resource-safe core for Sprint 4. Commit granularity follows engineering decisions and their verification.

## 5. Sprint 4: Proofread interaction and browser integration

Add one **Proofread with Emenda** context-menu item and only the `contextMenus` permission; retain toolbar Enable/Reactivate and the existing native menu. Register the worker listener synchronously, reconcile menu patterns/removal with enabled origins and recover idempotently after worker restart. UI visibility never substitutes for authorization.

Implement the trusted content-side candidate and browser-owned one-use intent handoff. Verify intended textarea, source generation/revision, document, exact value/collapsed selection, configuration, site authority, foreground and exposure before dispatch. Cover untouched prefilled text, immediate action during settling, multiple textareas, native menu focus transitions, stale candidates, unsupported editable targets, navigation, revocation and worker restart. Fail closed without switching source or forcing an ineligible selection.

Reuse validation/derivation, single suggestion, complete identifiable before/after display, trusted Apply/Dismiss, immediate worker authorization, native insertion and exact one-step Undo. Integrate concise Busy and Deep check temporarily unavailable outcomes. Every edit/lifecycle/configuration change retires obsolete authority; no suggestion action initiates inference.

Pass the full deterministic and Browser Integration suites with mocked/synthetic providers. Preserve minimum Chrome 140, exact-port permissions, trusted storage isolation, IME, lifecycle, refused-surface, accessibility and rendering checks. This yields one exact release-candidate identity for Sprint 5.

## 6. Sprint 5: Real-Mac acceptance and release

ChatGPT Work owns the supplied roadmap's real-machine acceptance phase under its separate authorization. Bind the exact implementation/specification commits, installed production bytes/extension identity, actual Brave/Chromium, Mac chip/RAM/OS, oMLX build/configuration and qualified model artifact. Use the existing successful predecessor evidence only for unchanged, explicitly identified invariants.

Run cheap deterministic/audit/clean-install/browser/resource checks first without model warm-up or loading. Confirm ordinary typing, idle, focus, navigation, startup and worker wakeup have zero provider traffic. Verify native German and English spelling with oMLX unavailable. Preserve actual toolbar grant/denial, exact-origin lifecycle, navigation/revoke/restart and editing checks; use mocked providers for breadth rather than extra live corpus runs.

Then perform one short synthetic explicit local deep smoke on the exact installed release candidate: at most one inference POST (plus required catalog and recovery GET only when applicable), exact model, production parsing, one suggestion, Apply, native Undo, local-only traffic and no duplicate/retry. Preserve Dismiss and insertion/deletion/replacement coverage through deterministic/browser fixtures. A memory-admission failure records a known deep-availability limitation and ends the smoke; do not relax guards, search models or enter an optimization loop. Do not declare full V0.2 Conformance if required native success evidence remains open.

Retain inherited 15/15 qualification only with proved equivalence and unchanged model/weights. Publish the exact tested implementation, then append factual results in a later blueprint commit changing exactly `docs/EVIDENCE.md`; cite the already-existing tested commit/tree and distinguish inherited live qualification, fresh deterministic/integration evidence and real-Mac smoke. Verify remote refs, installed bytes, clean tracked worktrees and cleanup of task-owned fixture processes/site access. Report one explicit readiness conclusion with limitations and stop.

## 7. Future execution policy

Use the single cross-platform audit command from [ENGINEERING](ENGINEERING.md), exact gate criteria from [ACCEPTANCE](ACCEPTANCE.md) and execution/Git discipline from [AGENTS](../AGENTS.md). The sole audit verifies the active imported source record and cumulative Browser coverage. Its automated gate ends at Browser Integration; Conformance combines that evidence with Sprint 5 installed-environment assessment. Internal names, helpers and equivalent techniques are Builder choices that preserve every observable contract.

Contenteditable, rich editors, multiple suggestions, grammar fast engines, spelling runtimes, floating buttons, shortcuts, extra writing actions, CLI/MCP, native integration, oMLX modifications and broader support remain separate future objectives.
