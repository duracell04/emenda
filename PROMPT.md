# Build Emenda

> **Frozen clean-room build objective, version 1.0.1**

Build Emenda from this documentation package.

Read the complete documentation set before implementation. Treat it as the product constitution.

Own the complete V0.1 outcome as one autonomous implementation objective. Acceptance gates are checkpoints inside that objective, not reasons to stop and request a new objective.

## Hard architectural invariant

> **Operating-system independence is a hard architectural invariant. Design every shared type, state transition, interface, test, and presentation behavior without reference to any particular operating system. Platform APIs exist exclusively in replaceable leaf bindings. The current host operating system is a runtime verification environment, not an architectural input.**

The first implementation phase contains only platform-neutral product code, provider code, presentation code, deterministic mocks, and tests.

A native binding begins only after the complete product loop passes the **Mock Product Gate** in `docs/ACCEPTANCE.md` and the architecture conformance review confirms that shared code contains no platform mechanism.

## Goal

Create the smallest coherent Emenda V0.1.

Emenda is a quiet local writing assistant that:

- observes meaningful editable-text changes;
- waits briefly for typing to settle;
- selects the smallest useful context;
- delegates linguistic judgment to OpenRouter;
- validates every response deterministically;
- shows one exact, reviewable suggestion;
- applies a change only after explicit writer approval;
- preserves the author's meaning, voice, terminology, rhythm, register, and Duktus.

The complete product outcome is:

```text
editable text changes
→ ObservedChange
→ reserve RevisionId
→ short debounce
→ bounded TextContext
→ seal immutable Revision
→ OpenRouter
→ validated corrections: [] | [Correction]
├─ [] → Clean → continue writing
└─ [Correction] → Suggestion
   → explicit Apply or Dismiss
   → TextSurface.replace_if_current(...) or no edit
   → continue writing
```

Reserving a newer `RevisionId` immediately makes all older work stale. The bounded context captured after debounce seals the immutable `Revision` associated with that reserved identifier.

## Application-owned semantics

The shared product owns:

```text
ObservedChange
SurfaceSignal
TextContext
ContextRequest
TextRange
TextGeometry
SourceReference
SourceDisplay
RevisionId
Revision
Correction
CheckRequest
CheckResult
Suggestion
State
validation
presentation behavior
typed outcomes
```

The `TextSurface` port exposes Emenda semantics only.

Replaceable bindings own:

```text
native event subscription
native source identity
native text access
native geometry
focus mechanics
source and text revalidation
replacement mechanics
native permissions and protected surfaces
```

The Rust desktop core operates unchanged against:

```text
MockTextSurface
WindowsTextSurface
MacTextSurface
LinuxTextSurface
```

Desktop bindings implement the Rust semantic port.

A browser product is a separate strict-TypeScript implementation of versioned, language-neutral semantic schemas. It is not the same executable as the Rust desktop core. Shared conformance fixtures verify that the browser implementation preserves the equivalent `TextSurface` behavior.

## Mock-first product gate

Build the shared product completely against:

```text
MockTextSurface
+
MockInferenceProvider
```

Before native binding work begins, prove this complete deterministic loop:

```text
mock text change
→ reserve RevisionId
→ debounce
→ context selection
→ seal immutable Revision
→ inference result
→ validation
→ suggestion view
→ Apply or Dismiss
→ safe mock replacement outcome
```

The Mock Product Gate is the dependency-direction proof. Native mechanisms plug into an already complete product rather than shaping the product from the outside inward.

## Technology

Use the documented stack:

- Tauri 2 for the desktop shell;
- safe Rust for the shared desktop application and native bindings;
- strict TypeScript, HTML, and CSS for the compact presentation;
- Serde, Zod, and JSON Schema for runtime boundaries;
- OpenRouter for linguistic intelligence.

Use:

```rust
#![forbid(unsafe_code)]
```

at the Emenda application crate root.

Keep the dependency graph proportional to current product capability.

## Decision function

When an implementation choice remains open, optimize in this order:

1. deliver the documented writer outcome;
2. preserve operating-system independence in all shared product code;
3. expose semantic guarantees and keep mechanisms inside leaf bindings;
4. complete the mock product before native integration;
5. keep local intelligence small and deterministic;
6. maximize compiler, type-system, schema, and permission enforcement;
7. minimize state, dependencies, build modes, scripts, and custom infrastructure;
8. keep the repository understandable in one pass;
9. produce direct evidence for the active acceptance gate.

Use this simplification order:

```text
remove an unnecessary requirement
→ defer a later requirement
→ use an existing stack capability
→ use one small explicit abstraction
→ add new machinery when measured product evidence requires it
```

## Execution

Implement through small verified increments defined in `docs/IMPLEMENTATION-PLAN.md`.

Use this canonical implementation sequence:

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

For each independently meaningful increment:

```text
inspect
→ implement one invariant
→ run the smallest relevant verification
→ inspect the diff
→ commit with what / why / architectural fit / evidence
→ push
→ verify the pushed state
→ continue
```

Advance through these checkpoints within the same V0.1 objective:

```text
Documentation Gate
→ Mock Product Gate
→ Provider Gate
→ Presentation Gate
→ Architecture Gate
→ Current-Host Binding Gate
→ V0.1 Conformance Gate
```

A later gate cannot redefine the result of an earlier verified gate. When a checkpoint passes, record its evidence and continue automatically to the next checkpoint.

The supplied frozen constitution remains immutable during implementation. After verifying the baseline, initialize the supplied `docs/EVIDENCE.md` ledger with the starting commit and use it for factual status and gate evidence; this mutable ledger is not part of the frozen constitution and is excluded from its checksums. A constitutional change requires a newly versioned documentation package.

## Scope boundary

V0.1 implements one product model:

```text
text changes
→ debounce
→ context
→ correction
→ suggestion
→ explicit application
```

V0.1 implements the single ambient correction workflow. Additional interaction models, distribution signing, installers, and public-release systems begin only under later explicitly named objectives supported by measured need.

## Completion

Complete every V0.1 criterion in `docs/ACCEPTANCE.md`.

Finish with:

```text
documentation conformance
→ mock product conformance
→ provider conformance
→ presentation conformance
→ architecture conformance
→ current-host binding evidence
→ UX and brand conformance
→ dependency review
→ clean worktree
→ final verified commit
→ stop
```

Stop only after the top-level V0.1 objective passes. If completion is genuinely blocked, first exhaust safe in-scope work and alternatives, then report the precise blocker, preserved state, evidence collected, and authority or external change required to continue. Release and distribution remain a later explicit objective.

Report the exact commits and evidence that establish the working product.
