# Emenda V0.1 Specification

> **Frozen product specification, version 1.0.1**

## 1. Product goal

Emenda is a personal local writing assistant that improves text while preserving the author's meaning, voice, rhythm, register, terminology, and Duktus.

The writer's existing application remains the primary writing surface.

Emenda is a small deterministic control layer around OpenRouter:

```text
observe
→ reserve revision ID
→ debounce
→ capture context
→ seal immutable revision
→ infer
→ validate
→ suggest
→ decide
→ apply safely
```

OpenRouter performs linguistic judgment.

Emenda performs observation policy, revision control, context selection, validation, presentation, and safe application.

## 2. Hard operating-system invariant

> **Every shared domain type, state transition, interface, test, and presentation behavior is designed without reference to a particular operating system. Platform APIs exist exclusively in replaceable leaf bindings. The current host is a verification environment, not a design premise.**

The Rust desktop core must operate unchanged against:

```text
MockTextSurface
WindowsTextSurface
MacTextSurface
LinuxTextSurface
```

The Rust desktop application owns one semantic `TextSurface` port implemented by those desktop surfaces.

The browser is a separate strict-TypeScript implementation. It implements the same versioned, language-neutral schemas and is verified with the same versioned conformance fixtures, but it does not implement the Rust trait or share desktop runtime code. Cross-environment compatibility means schema and fixture conformance, not a common binary.

## 3. V0.1 user outcome

```text
writer types ordinary editable text
→ writer pauses briefly
→ Emenda receives an ObservedChange
→ Emenda immediately reserves a new RevisionId
→ Emenda requests the smallest useful TextContext
→ Emenda seals an immutable Revision from that ID and context
→ Emenda sends that context to OpenRouter
→ OpenRouter returns structured corrections
→ Emenda validates the response
→ Emenda presents one exact Suggestion
→ writer chooses Apply or Dismiss
→ Apply asks TextSurface to replace only if the source is still current
→ writing continues
```

Observation is ambient.

Application is explicit.

## 4. Local intelligence budget

V0.1 local intelligence answers six questions:

```text
Did current nonempty context meaningfully change?
Has typing settled?
What is the smallest useful context?
Is the result still current?
Is the correction valid?
Can the exact edit still be applied safely?
```

Linguistic judgment stays in OpenRouter.

V0.1 does not add local language models, embeddings, local grammar engines, speculative rewrite heuristics, or parallel correction workflows.

## 5. Canonical semantic types

The names below describe product meaning. Exact field decomposition may remain minimal while preserving these semantics.

### 5.1 `RevisionId`

A monotonically increasing session identifier reserved synchronously on every `ObservedChange`. Reserving a newer ID immediately invalidates every older debounce, context request, inference result, suggestion, and Apply action.

### 5.2 `Revision`

An immutable value sealed only after the 500 ms debounce has settled and current context capture succeeds:

```ts
type Revision = {
  id: RevisionId;
  context: TextContext;
};
```

The ID is the one reserved by the originating `ObservedChange`; context capture never allocates a replacement ID. A sealed revision is the only input to inference and correction validation.

### 5.3 `SourceReference`

An opaque token created and understood by the active `TextSurface` implementation.

Shared code may store, compare for equality where required, and return the token to the same port. Shared code never parses native identity.

### 5.4 `SourceDisplay`

Display-safe information such as:

```text
application label
optional context label
```

It contains no native handle, process identifier, executable path, DOM reference, or accessibility object.

### 5.5 `ObservedChange`

A semantic notification that eligible editable text may have changed.

It identifies the opaque source and carries only the minimum generic information required to request current context.

### 5.6 `SurfaceSignal`

The subscription signal delivered by `TextSurface`:

```text
Changed(ObservedChange)
or
Unavailable { optional SourceDisplay, SurfaceError }
```

`Unavailable` communicates an ineligible, protected, or temporarily unsupported surface without exposing text or `SourceReference`. It gives the controller one typed path to invalidate an older suggestion and publish a safe error state.

### 5.7 `ContextRequest`

A platform-neutral request describing the bounded context policy:

```text
maximum Unicode scalar length = MAX_CONTEXT_SCALARS
preferred sentence or local-paragraph boundary
change-centered selection
```

The core owns this policy. V0.1 defines:

```text
MAX_CONTEXT_SCALARS = 2000
```

### 5.8 `TextContext`

The current bounded text associated with an observed change.

Conceptually:

```text
source reference
source display
text
changed or focus range
opaque surface version or verification token when useful
```

The active binding may include opaque verification material without exposing its mechanism.

Once a `TextContext` is sealed into a `Revision`, it is immutable.

### 5.9 `TextRange`

A half-open Unicode scalar-value range inside `TextContext.text`.

### 5.10 `TextGeometry`

Optional platform-neutral screen geometry for a text range.

Conceptually:

```text
x
y
width
height
```

These four values are finite logical pixels relative to the active Emenda presentation root. `width` and `height` are non-negative; `x` and `y` may be negative for a valid multi-display desktop layout. A binding converts native coordinates before returning geometry. The root is implicit in the composed presentation, so no native coordinate system, monitor identifier, transform, scale factor, or coordinate-space token crosses the port.

Geometry enriches placement. It does not create an alternate product workflow.

### 5.11 `Correction`

One exact proposed change:

```ts
type Correction = {
  start: number;
  end: number;
  original: string;
  replacement: string;
  category: "spelling" | "grammar" | "punctuation" | "style";
  confidence: "high" | "medium" | "low";
  explanation?: string;
};
```

### 5.12 `Suggestion`

An internal current-revision proposal containing the validated correction and the product state required to apply it safely.

The presentation receives a display DTO with a `SuggestionId`, `SourceDisplay`, exact before/after text, category, explanation, and optional geometry.

Opaque source identity remains outside the frontend boundary.

## 6. `TextSurface` semantic port

The application-owned port expresses Emenda guarantees rather than platform mechanics.

A conceptual shape is:

```rust
trait TextSurface: Send + Sync {
    fn subscribe(
        &self,
        sink: SurfaceSink,
    ) -> Result<Subscription, SurfaceError>;

    async fn context(
        &self,
        change: &ObservedChange,
        request: &ContextRequest,
    ) -> Result<TextContext, SurfaceError>;

    async fn geometry(
        &self,
        source: &SourceReference,
        range: &TextRange,
    ) -> Result<Option<TextGeometry>, SurfaceError>;

    async fn replace_if_current(
        &self,
        source: &SourceReference,
        expected: &TextContext,
        range: &TextRange,
        replacement: &str,
    ) -> Result<(), SurfaceError>;
}
```

`SurfaceSink` receives `SurfaceSignal`. The exact Rust representation may use channels, streams, callbacks, or async traits according to the smallest idiomatic implementation.

Every public word describes Emenda semantics.

### 6.1 Binding ownership

A leaf binding owns:

```text
native event subscription
native source identity
text retrieval
geometry retrieval
focus when required
current-source and current-text verification
replacement
native permission and protection checks
```

A binding emits an `ObservedChange` only when all three eligibility conditions hold:

```text
surface is editable
+ access is permitted
+ surface is not secure or protected
```

Ineligible surfaces emit `SurfaceSignal::Unavailable` with display-safe context where available and do not expose text or opaque identity to the core.

### 6.2 Application ownership

The shared product owns:

```text
meaningful-change policy after context capture
context policy
debounce
revision authority
inference orchestration
correction validation
suggestion state
writer decisions
```

## 7. Mock-first product requirement

`MockTextSurface` is the first `TextSurface` implementation.

It deterministically supports:

```text
emit change
return context
return optional geometry
record replacement requests
simulate changed source
simulate protected source
simulate unsupported operation
```

`MockInferenceProvider` returns deterministic valid and invalid results.

The complete product loop must pass against these mocks before any platform-specific dependency, module, target branch, API, identifier, or test enters the implementation.

## 8. Observation and debounce

The active binding emits `SurfaceSignal::Changed(ObservedChange)` only for editable, access-permitted, non-secure surfaces. It may emit `SurfaceSignal::Unavailable` to communicate a protected, ineligible, or temporarily unsupported surface without reading its text.

On `Unavailable`, the controller reserves the next `RevisionId`, invalidates older work, publishes the corresponding `Error(ErrorKind)`, and performs no debounce, context request, or inference call.

The controller:

```text
receives change
→ synchronously reserves the next RevisionId
→ immediately invalidates all older work and visible suggestions
→ restarts one debounce timer
→ requests current bounded context after 500 ms settles
→ discards the capture if the reserved ID is no longer current
→ seals immutable Revision { id, context }
```

V0.1 debounce:

```text
500 ms
```

Store it as one explicit constant.

ID reservation and invalidation happen before debounce. Context capture and revision sealing happen after debounce. A newer change therefore becomes authoritative immediately even while older context or inference work is still in flight.

After a current context is captured, the core classifies it as meaningful only when all three conditions hold:

```text
context is nonempty
+ reserved RevisionId is still current
+ TextContext differs from the last authoritative TextContext
```

Only meaningful current context proceeds to inference. Duplicate, empty, or stale captures do not call the provider.

For this comparison, `TextContext` identity is its semantic source, text, and focus/change range. A changed binding-private verification token alone does not make unchanged text meaningful.

## 9. Context selection

The shared core owns the smallest-useful-context policy.

V0.1 uses Unicode scalar positions and the constant:

```text
MAX_CONTEXT_SCALARS = 2000
```

Given the eligible source text and changed range, selection is deterministic:

1. If the changed range itself exceeds 2000 Unicode scalars, return `ContextTooLarge` and do not call inference.
2. Use the sentence enclosing the changed range when both sentence boundaries are reliable and the sentence is at most 2000 Unicode scalars.
3. Otherwise use the local paragraph enclosing the changed range when both paragraph boundaries are reliable and the paragraph is at most 2000 Unicode scalars.
4. Otherwise create a window containing the complete changed range. Divide the remaining scalar capacity evenly before and after the range, give an odd spare scalar to the trailing side, clamp at the document edges, and backfill unused capacity from the other side.

No selected context exceeds `MAX_CONTEXT_SCALARS`.

The policy remains one small pure component.

The binding retrieves text according to the semantic request. Platform APIs do not decide linguistic context policy.

## 10. Revision model

Every `ObservedChange` synchronously reserves the next monotonically increasing `RevisionId`. This reservation is the authoritative freshness boundary; it invalidates older timers, captures, requests, results, suggestions, and pending Apply actions before the debounce runs.

After debounce, context capture uses the already-reserved ID. If that ID is still current, the controller seals:

```text
Revision {
  id: reserved RevisionId,
  context: captured TextContext
}
```

The sealed revision is immutable. Every inference request carries its `RevisionId`, and the provider wrapper correlates its result with that same ID.

Core rule:

```text
result.revision_id == current_revision_id
```

Freshness is checked after every asynchronous boundary and again on Apply. An older result becomes `StaleRevision` and cannot enter visible suggestion state or trigger replacement.

## 11. Correction validation

A correction is applicable when:

```text
revision is current
response contains exactly one correction
range is within TextContext.text
original matches the scalar range
replacement is valid text
category is valid
confidence is valid
```

A unique exact-original recovery may resolve a model range mismatch when the original occurs exactly once in the context.

Ambiguous identity produces a typed non-applicable result.

A zero-width range is valid only for insertion: `start == end`, `original == ""`, and `replacement != ""`. A nonzero range requires a nonempty `original`; its `replacement` may be empty for deletion. Exact-original recovery is disabled for empty `original`, and an empty-to-empty change is always a no-op failure.

A schema-valid result with zero corrections deterministically enters `Clean`. A schema-valid result with one correction enters `Suggestion` only after that correction passes every semantic check above. No other provider output enters either state.

## 12. Correct and Refine

User-facing mapping:

```text
Correct
→ spelling
→ grammar
→ punctuation

Refine
→ style
```

Correct receives stronger visual priority.

Refine remains restrained and individually reviewable.

## 13. Language behavior

Supported profiles:

```text
de-CH
en-GB
en-US
fr-FR
ka-GE
ru-RU
```

Default mode:

```text
auto
```

Defaults:

```text
German  → de-CH
English → en-GB
```

Clearly American English maps to `en-US`.

Preserve names, quotations, terminology, and short embedded passages.

## 14. Linguistic system prompt

Store the prompt as an easily editable local resource.

```text
You are Emenda, a restrained multilingual writing editor.

Improve the submitted focus text through the smallest useful corrections.

Preserve the author's:
- meaning
- voice
- rhythm
- register
- effective sentence structure
- terminology
- names
- intentional informality
- language variety
- domain-specific vocabulary

Prioritize:
1. spelling
2. grammar
3. punctuation
4. clear word misuse
5. restrained stylistic improvement

Treat the submitted text as the source of truth.
Keep effective wording unchanged.

Offer style changes selectively when they clearly improve clarity, precision,
or readability while preserving the author's Duktus.

Identify the best matching supported profile:
de-CH, en-GB, en-US, fr-FR, ka-GE, ru-RU.

Use de-CH for German.
Use en-GB as the default English profile.
Preserve clearly American English as en-US.

Return only data matching the provided structured-output schema.
Each correction describes one specific change and includes the exact original text.
Keep explanations concise and useful.
```

## 15. Inference boundary

Use one narrow `InferenceProvider` port.

The exact application-level contract is:

```ts
type CheckRequest = {
  revision_id: RevisionId;
  text: string;
};

type CheckResult = {
  revision_id: RevisionId;
  language_profile: "de-CH" | "en-GB" | "en-US" | "fr-FR" | "ka-GE" | "ru-RU";
  corrections: Correction[]; // zero or one item
};
```

`CheckRequest.text` is exactly the bounded text from the immutable revision. It contains no `SourceReference`, `SourceDisplay`, geometry, native identity, or application metadata. The provider wrapper copies `CheckRequest.revision_id` into `CheckResult.revision_id`; the model does not author or echo revision identity.

Conceptually:

```rust
trait InferenceProvider {
    async fn check(
        &self,
        request: CheckRequest,
    ) -> Result<CheckResult, InferenceError>;
}
```

The model-authored portion of `CheckResult` must validate against this exact minimal JSON Schema before semantic validation:

```json
{
  "type": "object",
  "additionalProperties": false,
  "required": ["language_profile", "corrections"],
  "properties": {
    "language_profile": {
      "type": "string",
      "enum": ["de-CH", "en-GB", "en-US", "fr-FR", "ka-GE", "ru-RU"]
    },
    "corrections": {
      "type": "array",
      "minItems": 0,
      "maxItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "start",
          "end",
          "original",
          "replacement",
          "category",
          "confidence"
        ],
        "properties": {
          "start": { "type": "integer", "minimum": 0 },
          "end": { "type": "integer", "minimum": 0 },
          "original": { "type": "string" },
          "replacement": { "type": "string" },
          "category": {
            "type": "string",
            "enum": ["spelling", "grammar", "punctuation", "style"]
          },
          "confidence": {
            "type": "string",
            "enum": ["high", "medium", "low"]
          },
          "explanation": { "type": "string" }
        }
      }
    }
  }
}
```

`additionalProperties: false` applies at both object levels. `explanation` is optional; every other correction field is required. Provider-enforced structured output is used when supported, but Emenda always performs the same local schema validation.

Implementations:

```text
MockInferenceProvider
OpenRouterProvider
```

Default OpenRouter model:

```text
openrouter/free
```

Use structured output when the selected route supports the required schema.

Every response passes deterministic local parsing and semantic validation.

V0.1 performs one provider request per sealed revision. It uses no automatic retry, fallback model, or silent substitution. A transport, protocol, or semantic failure remains typed and observable; a later product version may add a measured retry policy without changing revision authority or validation.

State mapping is deterministic:

```text
zero corrections            → Clean
one semantically valid item → Suggestion
schema or semantic failure  → Error(ErrorKind)
```

## 16. Controller

The controller is pure application orchestration.

It owns:

```text
immediate RevisionId reservation
debounce state
context request
immutable Revision sealing
meaningful-change decision
inference request
stale-result rejection
validated suggestion session
Apply / Dismiss transitions
```

It depends only on:

```text
TextSurface
InferenceProvider
validator
presentation-state publisher
```

It contains no platform names, native identifiers, accessibility APIs, clipboard logic, keyboard logic, focus strategy, or target-specific timing.

## 17. Presentation

V0.1 uses a tiny Tauri-owned suggestion surface built with:

```text
strict TypeScript
HTML
CSS
```

The product state enum is exactly:

```text
State =
  Quiet
  | Checking
  | Suggestion
  | Clean
  | Error(ErrorKind)
```

The `Suggestion` state carries the current `SuggestionView`; `SuggestionView` is data, not another state discriminant.

Phrases such as `Text looks good`, `Connection issue`, `Invalid response`, `Stale result`, `Protected surface`, and `Replacement issue` are presentation copy derived from `Clean` or `Error(ErrorKind)`. They are not additional product states.

A stale background completion publishes no transition. `Error(StaleRevision)` is reserved for a writer-triggered Apply that loses a race with a newer authoritative revision; no replacement occurs.

A suggestion displays:

```text
Correct or Refine
original → replacement
short explanation
SourceDisplay label when useful
Apply
Dismiss
```

When `Correction.explanation` is absent, the presentation derives concise deterministic copy from the category: `Spelling correction.`, `Grammar correction.`, `Punctuation correction.`, or `Style refinement.` The UI never asks the model for a second explanation.

The presentation receives display-safe DTOs only.

A current schema-valid result with no correction enters `Clean`. Exactly one current, schema-valid, semantically valid correction enters `Suggestion`. Validation never promotes any other shape into either state.

Apply sends `SuggestionId`. The Rust controller resolves the internal source and context state.

The presentation may use `TextGeometry` for placement when available. The product interaction remains the same when geometry is absent.

## 18. Safe application

Apply invokes one semantic operation:

```text
replace_if_current(
    source,
    expected_context,
    correction_range,
    replacement
)
```

The active binding verifies the source and expected text immediately before one coherent edit.

A verification failure leaves the current source unchanged and returns a typed result.

Apply is reversible through native Undo only when the host and active binding support one coherent undoable replacement. Where that guarantee is unavailable, Emenda makes no reversibility claim; source-current verification and explicit writer approval remain mandatory.

## 19. Configuration

Personal V0.1 uses environment-based local configuration:

```text
OPENROUTER_API_KEY
OPENROUTER_MODEL
```

Default:

```text
OPENROUTER_MODEL=openrouter/free
```

Track `.env.example` when implementation begins.

The documentation package itself remains Markdown-only.

## 20. Error model

Use typed outcomes:

```text
Configuration
Observation
Context
ContextTooLarge
InferenceTransport
InferenceProtocol
InferenceSemantic
Validation
StaleRevision
ProtectedSurface
Replacement
Unsupported
```

Every exceptional state communicates:

```text
what happened
→ what Emenda preserved
→ next useful action
```

## 21. Privacy and security

- The active binding treats a surface as eligible only when it is editable, access is permitted, and it is not secure or protected.
- Ineligible surfaces expose no text to the core and return typed binding outcomes where a state must be communicated.
- Send only the bounded context required for the current correction request.
- Keep API credentials outside presentation state and logs.
- Treat OpenRouter data as untrusted until validation succeeds.
- Keep opaque source identity outside the TypeScript UI boundary.
- Apply no source edit without explicit writer action.

## 22. Technology stack

```text
Tauri 2
safe Rust
strict TypeScript
HTML
CSS
Serde
Zod
JSON Schema
OpenRouter
```

Rust implements the desktop core and desktop `TextSurface` bindings. Strict TypeScript separately implements browser semantics against the same versioned language-neutral schemas and conformance fixtures; neither runtime imports the other's platform implementation.

A UI framework becomes justified when measured presentation complexity makes the total repository simpler with it.

## 23. Repository shape

Keep one modular monolith.

A likely implementation shape is:

```text
emenda/
├── README.md
├── PROMPT.md
├── SPEC.md
├── ROADMAP.md
├── AGENTS.md
├── UX.md
├── BRAND.md
├── docs/
│   ├── ARCHITECTURE.md
│   ├── ENGINEERING.md
│   ├── IMPLEMENTATION-PLAN.md
│   └── ACCEPTANCE.md
├── src-tauri/
│   └── src/
│       ├── controller.rs
│       ├── correction.rs
│       ├── context.rs
│       ├── inference/
│       ├── text_surface/
│       │   ├── mod.rs
│       │   ├── mock.rs
│       │   └── <current-host-binding>.rs
│       └── presentation.rs
└── ui/
    ├── index.html
    ├── app.ts
    └── app.css
```

The exact decomposition emerges from responsibility. Empty future-platform modules are unnecessary.

## 24. V0.1 scope

V0.1 includes:

```text
platform-neutral semantic domain
TextSurface port
MockTextSurface
InferenceProvider
MockInferenceProvider
OpenRouterProvider
meaningful change orchestration
500 ms debounce
MAX_CONTEXT_SCALARS = 2000 deterministic context policy
immediate RevisionId reservation and immutable Revision sealing
structured zero-or-one Correction[]
deterministic validation
tiny suggestion presentation
Apply
Dismiss
safe replacement semantics
one current-host native binding
runtime verification in two editable applications on that host
```

The owner's present runtime verification environment is Windows. This fact affects only the leaf binding and binding-specific evidence.

## 25. Deferred objectives

Later separately authorized milestones may include:

```text
additional native bindings
browser extension
inline markers
richer geometry and anchoring
per-application behavior
personal vocabulary
local inference
distribution packaging
signing
installers
public release automation
```

Each begins from measured need and preserves the frozen shared contracts unless a new constitutional version explicitly changes them.

## 26. Definition of Done

V0.1 is complete when:

```text
Mock Product Gate passes completely
→ live OpenRouter compatibility is evidenced
→ compact presentation passes its state and accessibility checks
→ architecture gate confirms zero OS mechanics in shared code
→ one current-host binding observes real text changes
→ the full loop succeeds in two ordinary editable applications
→ Apply changes only the intended current source
→ Dismiss preserves the source
→ stale work cannot affect newer text
→ dependency and documentation conformance pass
```

The final repository is compact, strongly typed, easy to understand, and causally aligned with this specification.
