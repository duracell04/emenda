# Emenda Roadmap

> **Frozen roadmap, version 1.0.0**

The roadmap follows dependency direction. Product semantics become complete before environment-specific mechanisms enter the repository.

## North star

```text
writer continues using the original application
→ Emenda notices a useful issue quietly
→ the exact proposed change appears
→ one deliberate action applies or dismisses it
→ authorship and flow remain with the writer
```

## Roadmap rule

Each milestone starts from verified output produced by the previous milestone.

A milestone adds one new axis of product value or evidence.

Infrastructure and alternate workflows enter only when measured product limitations make them necessary.

## Milestone 0: Frozen clean-room constitution

Deliverables:

```text
Markdown-only canonical package
hard operating-system invariant
semantic domain model
mock-first requirement
acceptance gates
roadmap and implementation plan
brand and UX system
```

Exit evidence:

```text
all package files present
cross-references valid
checksums recorded
archive contains Markdown only
```

## Milestone 1: Semantic foundation

Build the smallest platform-neutral Rust core scaffold and implement:

```text
RevisionId
SourceReference
SourceDisplay
ObservedChange
ContextRequest
TextContext
TextRange
TextGeometry
Correction
Suggestion
error taxonomy
```

Exit evidence:

```text
pure domain tests pass
Unicode scalar semantics pass
no Tauri, native text dependency, target branch, or platform-named product type shapes the core
```

## Milestone 2: Application-owned ports and mocks

Build:

```text
TextSurface
MockTextSurface
InferenceProvider
MockInferenceProvider
```

Exit evidence:

```text
mock can emit changes
mock can return context and geometry
mock can record safe replacement requests
mock can simulate changed and protected sources
provider can return deterministic success and failure outcomes
```

## Milestone 3: Deterministic product engine

Build:

```text
meaningful-change handling
debounce
revision authority
context policy
correction validation
suggestion session
Apply / Dismiss transitions
```

Exit evidence:

```text
rapid changes coalesce
stale results are rejected
valid corrections become suggestions
invalid corrections remain fail-closed
```

## Milestone 4: OpenRouter provider boundary

Build:

```text
OpenRouterProvider
minimal structured request
bounded response handling
typed transport/protocol/semantic outcomes
environment configuration
```

Exit evidence:

```text
provider request and parsing tests pass
one strict live request records a valid correction or correctly typed external failure
source text remains untouched on invalid output
```

## Milestone 5: Tiny presentation

Build:

```text
minimal Tauri composition shell
compact suggestion surface
strict TypeScript state
Zod boundary
Apply
Dismiss
keyboard access
brand styling
```

Exit evidence:

```text
presentation states render correctly
frontend receives display-safe data only
Tauri shell contains composition rather than text-surface mechanisms
```

## Milestone 6: Complete mock product

Connect:

```text
MockTextSurface
→ controller
→ MockInferenceProvider
→ validator
→ presentation state
→ Apply / Dismiss
→ MockTextSurface replacement
```

Exit evidence:

```text
Mock Product Gate passes
Presentation Gate passes
complete product behavior exists without any native binding
```

This is the critical architecture milestone.

## Milestone 7: Architecture freeze before native work

Audit:

```text
shared domain
controller
context policy
provider port
presentation DTOs
Tauri composition
tests
dependencies
```

Exit evidence:

```text
shared code has zero OS mechanics
native source identity cannot cross the port
complete product still passes through mocks
no speculative capability or alternate-workflow framework exists
```

Native binding work begins only after this gate passes.

## Milestone 8: Current-host binding

Implement one leaf binding for the available runtime verification environment.

The owner's present environment is Windows, so current runtime work may produce `WindowsTextSurface` after Milestone 7.

Binding responsibilities:

```text
observe editable-text changes
retain native identity privately
retrieve requested context
return optional geometry
verify current source and text
perform one exact replacement
report typed protection and support outcomes
```

Exit evidence:

```text
binding-level tests pass
one real editable application produces ObservedChange and safe replacement
shared contracts remain unchanged
```

## Milestone 9: V0.1 runtime validation

Verify the complete loop in:

```text
one simple editable application
+
one additional ordinary editable application
```

Exit evidence:

```text
ambient observation
one debounced current request
valid suggestion
Apply
Dismiss
stale-result safety
changed-source safety
```

## Milestone 10: V0.1 conformance freeze

Complete:

```text
architecture audit
dependency audit
UX audit
brand audit
documentation status update
clean worktree
final verified commit
```

Then stop.

## Future roadmap

Future milestones are chosen from measured product evidence:

```text
additional host bindings
browser binding
richer geometry and inline indication
per-application behavior
personal vocabulary
model-quality controls
local inference
public distribution
```

Each future milestone begins under a new explicit objective and preserves the shared product semantics established in V0.1.
