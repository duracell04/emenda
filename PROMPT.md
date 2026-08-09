# Build Emenda

> **Frozen clean-room build objective, version 1.0.0**

Build Emenda from this documentation package.

Read the complete documentation set before implementation. Treat it as the product constitution.

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
→ short debounce
→ bounded TextContext
→ immutable Revision
→ OpenRouter
→ validated Correction[]
→ Suggestion
→ explicit Apply or Dismiss
→ TextSurface.replace_if_current(...)
→ continue writing
```

## Application-owned semantics

The shared product owns:

```text
ObservedChange
TextContext
ContextRequest
TextRange
TextGeometry
SourceReference
SourceDisplay
RevisionId
Correction
Suggestion
controller state
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

The application must operate unchanged against:

```text
MockTextSurface
WindowsTextSurface
MacTextSurface
LinuxTextSurface
BrowserTextSurface
```

Desktop bindings implement the Rust semantic port. A browser implementation provides the equivalent contract in strict TypeScript.

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
→ debounce
→ context selection
→ revision
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
- Serde and Zod for runtime boundaries;
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

The active gate controls scope:

```text
Mock Product Gate
→ Provider Gate
→ Presentation Gate
→ Architecture Gate
→ Current-Host Binding Gate
→ V0.1 Conformance Gate
```

A later gate cannot redefine the result of an earlier verified gate.

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
mock product conformance
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

Report the exact commits and evidence that establish the working product.
