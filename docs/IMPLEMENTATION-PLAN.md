# Emenda V0.1 Implementation Plan

> **Frozen implementation plan, version 1.0.0**

## 1. Execution objective

Build Emenda from an empty implementation context through small verified commits.

The complete writer-facing product passes through mocks before a native binding enters the repository.

## 2. Baseline handling

When earlier code exists:

```text
preserve history once
→ establish documentation-only clean-room baseline
→ verify baseline
→ start Increment 1
```

When the repository is already clean:

```text
verify documentation package
→ start Increment 1
```

Recovery and rebuilding remain separate commits and objectives.

## 3. Increment 1: Platform-neutral core scaffold and domain

Create the smallest Rust library scaffold required for pure product code and tests.

Use:

```text
safe Rust
#![forbid(unsafe_code)]
standard test runner
no Tauri dependency yet
no native text dependency
no target-specific source module
```

Implement:

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
SuggestionId
SuggestionView
core error enums
```

Verification:

```text
Rust compiles
pure domain tests pass
Unicode scalar ranges pass
display-safe serialization excludes SourceReference
no platform-specific field or name shapes a product type
```

Commit.

## 4. Increment 2: `TextSurface` port

Implement the application-owned semantic port:

```text
subscribe
context
geometry
replace_if_current
```

Define typed `SurfaceError` outcomes.

Verification:

```text
port compiles
public API contains semantic vocabulary only
controller-facing types contain no environment mechanism
```

Commit.

## 5. Increment 3: `MockTextSurface`

Implement deterministic behavior:

```text
emit ObservedChange
return TextContext
return optional TextGeometry
record replacement requests
simulate changed source
simulate protected source
simulate unsupported operation
```

Verification:

```text
mock contract tests pass
all outcomes are deterministic
```

Commit.

## 6. Increment 4: Controller, revision, debounce, and context policy

Implement:

```text
current RevisionId
one debounce timer
500 ms default
ContextRequest policy
stale-result rejection
controller state machine
```

Verification:

```text
rapid changes produce one current context request
newer change supersedes older work immediately
stale result cannot enter suggestion state
context limits are deterministic
```

Commit.

## 7. Increment 5: `InferenceProvider` and provider implementations

Implement:

```text
CheckRequest
CheckResult
InferenceProvider
MockInferenceProvider
OpenRouterProvider
environment configuration
```

Keep provider protocol details outside the controller.

Verification:

```text
deterministic mock success and failure outcomes
OpenRouter request-construction tests
bounded envelope parsing
typed transport/protocol/semantic failures
one strict live request when credentials and network are available
```

Commit.

## 8. Increment 6: Correction validator

Implement:

```text
structured parsing boundary
Unicode scalar range validation
original-text identity
unique exact-original recovery
overlap rejection
non-applicable outcomes
```

Verification:

```text
liek → like
Georgian or Russian Unicode example
ambiguous original
out-of-range correction
overlapping corrections
no-op replacement
```

Commit.

## 9. Increment 7: Minimal Tauri composition and presentation

Introduce Tauri only after the platform-neutral engine and ports exist.

Implement:

```text
thin Tauri composition root
strict TypeScript
HTML
CSS
Quiet
Checking
Suggestion
Clean
Error
SuggestionView
Apply
Dismiss
keyboard paths
brand tokens
```

The Tauri shell composes dependencies and routes typed state. It contains no text-surface mechanism.

The frontend receives no opaque source reference or native identity.

Verification:

```text
Rust and Tauri compile
strict TypeScript passes
Zod boundary tests pass
state rendering passes
keyboard focus passes
Apply/Dismiss intent events pass
```

Commit.

## 10. Increment 8: Complete mock product loop

Connect:

```text
MockTextSurface change
→ controller debounce
→ ContextRequest
→ TextContext
→ MockInferenceProvider
→ validator
→ SuggestionView
→ Apply or Dismiss
→ MockTextSurface.replace_if_current
```

Verification:

```text
Apply records one exact safe replacement
Dismiss records no replacement
changed source returns typed replacement failure
protected source returns typed state
invalid model output leaves source untouched
newer change invalidates older suggestion
presentation exposes the complete writer interaction
```

Run the complete Mock Product Gate and Presentation Gate.

Commit.

## 11. Increment 9: Architecture gate

Audit the repository before native work.

Verify:

```text
complete mock product passes
shared code contains no OS mechanics
frontend receives display-safe state only
Tauri shell is composition only
no native text dependency exists
no target-specific module has shaped the port
the implementation contains one ambient workflow and no generalized capability framework
```

Record the gate result.

Commit only factual documentation or mechanically useful architecture checks required by this gate.

## 12. Increment 10: Current-host binding

Select the binding that matches the available runtime verification environment.

In the owner's present environment, implement `WindowsTextSurface` as a leaf after Increment 9.

Responsibilities:

```text
subscribe to eligible editable-text events
create private source identity
retrieve requested context
translate native positions to Unicode scalar ranges
return optional geometry
verify current source and expected context
perform one exact replacement
return typed outcomes
```

The binding preserves the existing semantic port without modification.

Verification:

```text
binding-level deterministic tests
protected-surface outcome
changed-source outcome
one real editable application observation and replacement
```

Commit.

## 13. Increment 11: Complete current-host loop

Run the real V0.1 flow in:

```text
one simple editable application
+
one additional ordinary editable application
```

Verify:

```text
ambient observation
one debounced current request
OpenRouter result
exact suggestion
Apply
Dismiss
changed-source safety
stale-result safety
```

Commit each product fix separately when runtime evidence reveals a violated invariant.

## 14. Increment 12: Final conformance

Run:

```text
docs/ACCEPTANCE.md
architecture review
dependency review
UX review
brand review
relevant health checks
documentation status review
```

Confirm the current host appears only in binding-specific code, tests, and evidence.

Commit the verified V0.1 conformance state.

## 15. Completion report

Report:

```text
final commit
all implementation commits
active-gate evidence
checks run
runtime applications verified
architecture result
dependency result
remaining future milestones
```

Finish with a clean worktree and stop.
