# Emenda Architecture

> **Frozen architecture, version 1.0.1**

## 1. Architectural objective

Emenda is an operating-system-independent product with replaceable environment bindings.

> **The shared product contains zero knowledge of how any operating system exposes, identifies, observes, positions, focuses, protects, or replaces text.**

The current host is a runtime verification environment only.

## 2. Dependency direction

```text
                    EMENDA DESKTOP PRODUCT

semantic domain
      |
      v
Controller ---------------- Presentation state
      |
      +------------------+
      v                  v
InferenceProvider     TextSurface
      |                  |
      v                  v
OpenRouterProvider    MockTextSurface
                         |
            after Mock Product Gate only
                         |
              +----------+----------+
              v          v          v
           Windows     macOS      Linux
           binding     binding    binding

              VERSIONED LANGUAGE-NEUTRAL CONTRACT

               schemas + conformance fixtures
                         /        \
                        v          v
              Rust desktop      strict-TS browser
```

The Rust desktop application owns the Rust ports. Its controller and domain run unchanged with `MockTextSurface`, `WindowsTextSurface`, `MacTextSurface`, or `LinuxTextSurface`.

Desktop bindings implement those ports.

Mocks prove the application without external technology.

The browser is a separate strict-TypeScript product implementation. It does not implement the Rust `TextSurface` trait and is not a branch of the desktop binary. Desktop and browser compatibility is established by the same explicitly versioned, language-neutral serialized schemas and conformance fixtures.

For this constitution, those schemas and fixtures are version `1.0.1`. A breaking contract change requires a new version rather than runtime-specific reinterpretation.

## 3. Core invariant

Shared product code uses only semantic types:

```text
RevisionId
Revision
SourceReference
SourceDisplay
ObservedChange
SurfaceSignal
ContextRequest
TextContext
TextRange
TextGeometry
Correction
CheckRequest
CheckResult
Suggestion
SuggestionView
State = Quiet | Checking | Suggestion | Clean | Error(ErrorKind)
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
| Context policy | deterministic sentence, paragraph, changed-range window selection; 2000-scalar bound | semantic text/range data |
| Validator | correction applicability | `TextContext`, `Correction` |
| Controller | immediate ID reservation, debounce, immutable revision sealing, orchestration, writer decisions | `TextSurface`, `InferenceProvider`, validator |
| InferenceProvider | exact `CheckRequest`/`CheckResult` semantics | domain types and strict result schema |
| OpenRouterProvider | external OpenRouter protocol | HTTP client, Serde/JSON schema |
| TextSurface | change/unavailable signals, context, geometry, and replacement semantics | domain types |
| MockTextSurface | deterministic product evidence | `TextSurface` |
| Native binding | eligibility, platform APIs, private native identity, geometry normalization, coherent replacement where supported | `TextSurface` |
| Contract schemas and fixtures | versioned language-neutral serialization and cross-runtime conformance cases | semantic contracts |
| Browser implementation | strict-TypeScript browser product and private DOM binding | versioned schemas and conformance fixtures |
| Presentation state | display-safe writer interaction | `SuggestionView`, typed states |
| Tauri shell | composition and window lifecycle | controller and presentation boundaries |

## 5. Semantic `TextSurface` port

The port exposes guarantees rather than procedures.

Conceptual operations:

```text
publish `Changed(ObservedChange)` only for editable, access-permitted, non-secure text changes
publish display-safe `Unavailable` without text or opaque identity when status must be communicated
request bounded current context
request optional range geometry normalized into Emenda presentation coordinates
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

V0.1 fixes `MAX_CONTEXT_SCALARS = 2000`. The deterministic selection order is:

```text
reliably bounded enclosing sentence that fits
→ reliably bounded enclosing local paragraph that fits
→ at-most-2000-scalar window containing the complete changed range
```

If the changed range itself exceeds the cap, the policy returns `ContextTooLarge` and inference does not run. Otherwise the fallback window contains the complete changed range, divides remaining capacity evenly before and after it, gives an odd spare scalar to the trailing side, clamps at document edges, and backfills unused capacity from the other side. No binding substitutes a platform-specific context heuristic.

Every `ObservedChange` reserves a monotonically increasing `RevisionId` immediately. After the 500 ms debounce, a context capture is accepted only if that ID is still current, then sealed with that same ID as an immutable `Revision`.

The core calls inference only when the captured context is nonempty, current, and semantically different from the last authoritative `TextContext`. Semantic comparison uses source, text, and focus/change range; a changed binding-private verification token alone is not a meaningful text change.

## 8. Geometry

`TextGeometry` is optional semantic data for presentation placement.

Its public shape is exactly:

```text
x
y
width
height
```

All values are finite logical pixels relative to the active Emenda presentation root. Width and height are non-negative; x and y may be negative in a valid multi-display desktop layout. The active binding converts from its native coordinates before returning. The root is implicit in the composed presentation, so native coordinate-space identifiers, monitor handles, transforms, scale factors, and platform coordinate types never cross the port.

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

The binding requests one coherent host edit when its environment supports that operation. Apply is reversible through native Undo only when the host and binding can guarantee a coherent undoable replacement; the shared product makes no reversibility claim otherwise.

## 10. Controller

The controller contains only product policy:

```text
receive SurfaceSignal
├─ Unavailable: reserve ID, invalidate older work, publish Error, stop
└─ Changed(ObservedChange)
   → synchronously reserve next RevisionId
   → invalidate all older work immediately
   → restart debounce
   → after 500 ms request TextContext with reserved ID
   → reject stale, empty, or unchanged context
   → seal immutable Revision { id, context }
   → call InferenceProvider
   → reject stale result
   → validate zero-or-one Correction[]
   → enter Clean for zero or Suggestion for one valid correction
   → process Apply or Dismiss
```

Freshness is checked after every asynchronous boundary and again on Apply. Context capture never allocates a second ID.

The controller can be understood and tested without knowing which binding is active.

## 11. Inference boundary

The controller depends on `InferenceProvider`.

The exact minimal application contract is:

```text
CheckRequest  = { revision_id, text }
CheckResult   = { revision_id, language_profile, corrections }
```

`language_profile` is one of `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, or `ru-RU`. `corrections` contains zero or one item. The provider copies `revision_id` from request to result; only `language_profile` and `corrections` are model-authored.

The exact model-output JSON Schema is defined in `SPEC.md`. It requires `language_profile` and `corrections`; requires `start`, `end`, `original`, `replacement`, `category`, and `confidence` on a correction; permits optional string `explanation`; sets `minItems: 0`, `maxItems: 1`; and sets `additionalProperties: false` at every object level.

Implementations:

```text
MockInferenceProvider
OpenRouterProvider
```

The provider boundary isolates external probabilistic behavior from deterministic product state.

## 12. Presentation boundary

The product state enum is exactly:

```text
State = Quiet | Checking | Suggestion | Clean | Error(ErrorKind)
```

`Suggestion` carries the current `SuggestionView`. UX phrases such as `Text looks good`, `Connection issue`, `Invalid response`, `Stale result`, `Protected surface`, and `Replacement issue` are copy derived from `Clean` or `Error(ErrorKind)`, not additional states.

Stale background completion produces no state transition. A writer-triggered Apply that races with a newer authoritative revision may produce `Error(StaleRevision)` and never replaces text.

A current schema-valid result with zero corrections enters `Clean`. A current result with exactly one correction enters `Suggestion` only after semantic validation succeeds.

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

The browser implementation is a separate strict-TypeScript product with a private browser-native binding.

It preserves equivalent semantics through version `1.0.1` language-neutral schemas and shared conformance fixtures:

```text
ObservedChange
TextContext
TextRange
TextGeometry
SourceReference
replace_if_current
```

It does not implement the Rust `TextSurface` trait, import the Rust desktop core, or exist as a Rust `target_os` branch. Its compatibility claim comes from passing the same serialized-contract fixtures as the desktop core.

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
