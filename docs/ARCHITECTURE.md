# Emenda Architecture

> **Frozen architecture, version 1.0.0**

## 1. Architectural objective

Emenda is an operating-system-independent product with replaceable environment bindings.

> **The shared product contains zero knowledge of how any operating system exposes, identifies, observes, positions, focuses, protects, or replaces text.**

The current host is a runtime verification environment only.

## 2. Dependency direction

```text
                    EMENDA PRODUCT

  semantic domain
        │
        ▼
  Controller ──────────────── Presentation state
        │
        ├───────────────┐
        ▼               ▼
InferenceProvider    TextSurface
        │               │
        ▼               ▼
OpenRouterProvider   MockTextSurface
                        │
           after Mock Product Gate only
                        │
            ┌───────────┼───────────┐
            ▼           ▼           ▼
     native binding native binding browser binding
```

The application owns the ports.

Bindings implement them.

Mocks prove the application without external technology.

## 3. Core invariant

Shared product code uses only semantic types:

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
SuggestionView
product states
typed errors
```

Shared code contains no reference to:

```text
Windows
macOS
Linux
HWND
process ID
executable path
UI Automation
AXUIElement
AT-SPI
clipboard formats
copy/paste shortcuts
keyboard simulation
DOM nodes
tab or frame identity
```

Binding-specific code may use the mechanisms of its environment privately.

## 4. Responsibility table

| Component | Owns | Depends on |
|---|---|---|
| Domain | semantic types and invariants | standard library only where practical |
| Context policy | smallest useful context rule | semantic text/range data |
| Validator | correction applicability | `TextContext`, `Correction` |
| Controller | debounce, revision, orchestration, writer decisions | `TextSurface`, `InferenceProvider`, validator |
| InferenceProvider | model request/result semantics | domain types |
| OpenRouterProvider | external OpenRouter protocol | HTTP client, Serde/JSON schema |
| TextSurface | observation/context/geometry/replacement semantics | domain types |
| MockTextSurface | deterministic product evidence | `TextSurface` |
| Native binding | platform APIs and private native identity | `TextSurface` |
| Presentation state | display-safe writer interaction | `SuggestionView`, typed states |
| Tauri shell | composition and window lifecycle | controller and presentation boundaries |

## 5. Semantic `TextSurface` port

The port exposes guarantees rather than procedures.

Conceptual operations:

```text
subscribe to eligible editable-text changes
request bounded current context
request optional range geometry
replace only while source and expected context remain current
```

A conceptual interface appears in `SPEC.md`.

The exact implementation may use channels, callbacks, streams, async traits, or explicit polling when those choices remain private and preserve the same semantics.

## 6. Source identity

`SourceReference` is opaque.

A binding may implement it through:

```text
registry token
random session token
opaque serialized token
binding-private lookup key
```

Shared code stores it and returns it to the same `TextSurface` without interpretation.

`SourceDisplay` is a separate display-safe value.

The frontend receives `SourceDisplay`, never `SourceReference` or native identity.

## 7. Context and ranges

The core owns context-selection policy.

The binding owns text retrieval.

The semantic exchange is:

```text
ObservedChange
+ ContextRequest
→ TextContext
```

`TextRange` always refers to Unicode scalar positions in `TextContext.text`.

The binding translates between these semantic positions and native positions internally.

## 8. Geometry

`TextGeometry` is optional semantic data for presentation placement.

Geometry enriches the same suggestion interaction. It does not create a separate correction workflow.

Absence of geometry remains an ordinary typed result represented without a generalized capability framework.

## 9. Atomic replacement

The controller asks for:

```text
replace_if_current(source, expected_context, range, replacement)
```

The binding owns the atomic native strategy:

```text
resolve private source identity
→ verify current surface
→ verify expected text or version
→ translate semantic range
→ apply one edit
→ return typed outcome
```

The controller never performs focus, recapture, process comparison, clipboard preparation, keyboard injection, or native selection logic.

## 10. Controller

The controller contains only product policy:

```text
receive ObservedChange
→ increment RevisionId
→ restart debounce
→ request TextContext
→ call InferenceProvider
→ reject stale result
→ validate Correction[]
→ publish SuggestionView
→ process Apply or Dismiss
```

The controller can be understood and tested without knowing which binding is active.

## 11. Inference boundary

The controller depends on `InferenceProvider`.

Implementations:

```text
MockInferenceProvider
OpenRouterProvider
```

The provider boundary isolates external probabilistic behavior from deterministic product state.

## 12. Presentation boundary

The UI receives only display-safe values:

```text
SuggestionId
SourceDisplay
category
original
replacement
explanation
optional TextGeometry
current status
```

Apply and Dismiss send identifiers and writer intent back to the controller.

The UI never reconstructs application state or source identity.

## 13. Mock-first architecture gate

Before a native binding enters the repository, the active implementation contains:

```text
semantic domain
TextSurface port
MockTextSurface
InferenceProvider
MockInferenceProvider
controller
validator
presentation state
compact UI
complete deterministic product loop
```

The architecture gate verifies:

```text
complete product loop passes through mocks
shared code contains no platform mechanism
shared tests contain no fake native handle or path
no target-specific dependency shapes shared contracts
```

This ordering prevents a host API from becoming the source of application abstractions.

## 14. Native bindings

A native binding is a leaf.

It may contain substantial environment-specific complexity when that complexity protects a demonstrated invariant.

Its public surface remains the unchanged `TextSurface` semantics.

The available verification host determines which binding is implemented first in time. That order has no architectural meaning.

## 15. Browser binding

The browser implementation uses strict TypeScript and browser-native events.

It preserves equivalent semantics:

```text
ObservedChange
TextContext
TextRange
TextGeometry
SourceReference
replace_if_current
```

It is not a Rust `target_os` branch.

## 16. Tauri shell

The Tauri shell is a thin composition root.

It owns:

```text
construct dependencies
start and stop application services
open and position Emenda windows
route typed commands/events
apply Tauri capability policy
```

It contains no binding mechanism or platform-specific text-recovery strategy.

## 17. Physical architecture

Keep one modular monolith.

A likely Rust decomposition is:

```text
domain
controller
context
correction
inference
text_surface
presentation
app composition
```

A new crate, service, feature mode, plugin, or framework enters only when it reduces total measured complexity.

Empty platform modules provide no evidence and remain absent.

## 18. Architecture anti-drift tests

Mechanically review:

```text
shared imports
shared public types
frontend schemas
controller logic
Tauri composition
Cargo target dependencies
mock fixtures
```

The architecture fails when shared code interprets a native identifier or implements a native procedure.

The architecture passes when the complete shared product is testable through mocks and a binding can be replaced without changing controller, validator, provider, or presentation behavior.

## 19. Gate separation

```text
Product correctness
→ shared product passes through mocks

Provider compatibility
→ OpenRouter path satisfies or fails its contract explicitly

Host support
→ one binding passes environment-specific integration tests

Distribution readiness
→ packaging, signing, trust, and installation pass separately
```

A failure remains owned by its gate.

## 20. Final architecture test

The architecture is correct when the answer to both questions is yes:

> Could a developer understand and test the entire product without knowing which operating system is active?

> Could a binding change its native event, identity, geometry, focus, or replacement mechanism without changing shared product code?
