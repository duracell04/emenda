# Emenda V0.1 Specification

> **Frozen product specification, version 1.0.0**

## 1. Product goal

Emenda is a personal local writing assistant that improves text while preserving the author's meaning, voice, rhythm, register, terminology, and Duktus.

The writer's existing application remains the primary writing surface.

Emenda is a small deterministic control layer around OpenRouter:

```text
observe
→ debounce
→ select context
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

The shared product must operate unchanged against:

```text
MockTextSurface
WindowsTextSurface
MacTextSurface
LinuxTextSurface
BrowserTextSurface
```

The Rust desktop application owns one semantic `TextSurface` port. A browser implementation provides equivalent semantics in strict TypeScript.

## 3. V0.1 user outcome

```text
writer types ordinary editable text
→ writer pauses briefly
→ Emenda receives an ObservedChange
→ Emenda creates a new RevisionId
→ Emenda requests the smallest useful TextContext
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
Did eligible text change?
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

A monotonically increasing session identifier for authoritative product state.

### 5.2 `SourceReference`

An opaque token created and understood by the active `TextSurface` implementation.

Shared code may store, compare for equality where required, and return the token to the same port. Shared code never parses native identity.

### 5.3 `SourceDisplay`

Display-safe information such as:

```text
application label
optional context label
```

It contains no native handle, process identifier, executable path, DOM reference, or accessibility object.

### 5.4 `ObservedChange`

A semantic notification that eligible editable text may have changed.

It identifies the opaque source and carries only the minimum generic information required to request current context.

### 5.5 `ContextRequest`

A platform-neutral request describing the bounded context policy, for example:

```text
maximum Unicode scalar length
preferred sentence or local-paragraph boundary
change-centered selection
```

The core owns this policy.

### 5.6 `TextContext`

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

### 5.7 `TextRange`

A half-open Unicode scalar-value range inside `TextContext.text`.

### 5.8 `TextGeometry`

Optional platform-neutral screen geometry for a text range.

Conceptually:

```text
x
y
width
height
coordinate-space identifier when required
```

Geometry enriches placement. It does not create an alternate product workflow.

### 5.9 `Correction`

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

### 5.10 `Suggestion`

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
        sink: ChangeSink,
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

The exact Rust representation may use channels, streams, callbacks, or async traits according to the smallest idiomatic implementation.

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

### 6.2 Application ownership

The shared product owns:

```text
change eligibility policy
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

The active binding emits semantic `ObservedChange` values for eligible editable-text changes.

The controller:

```text
receives change
→ increments RevisionId
→ restarts one debounce timer
→ requests current bounded context after the timer settles
```

Recommended initial debounce:

```text
500 ms
```

Store it as one explicit constant.

A newer change becomes authoritative immediately.

## 9. Context selection

The shared core owns the smallest-useful-context policy.

Initial policy:

```text
current sentence where boundaries are reliable
otherwise local paragraph window
hard Unicode scalar maximum
centered around the changed range
```

The policy remains one small pure component.

The binding retrieves text according to the semantic request. Platform APIs do not decide linguistic context policy.

## 10. Revision model

Every inference request carries the `RevisionId` that produced it.

Core rule:

```text
result.revision_id == current_revision_id
```

An older result becomes `StaleRevision` and cannot enter visible suggestion state or trigger replacement.

## 11. Correction validation

A correction is applicable when:

```text
revision is current
range is within TextContext.text
original matches the scalar range
replacement is valid text
category is valid
confidence is valid
corrections do not overlap
```

A unique exact-original recovery may resolve a model range mismatch when the original occurs exactly once in the context.

Ambiguous identity produces a typed non-applicable result.

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

Conceptually:

```rust
trait InferenceProvider {
    async fn check(
        &self,
        request: CheckRequest,
    ) -> Result<CheckResult, InferenceError>;
}
```

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

## 16. Controller

The controller is pure application orchestration.

It owns:

```text
current revision
debounce state
context request
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

Primary states:

```text
Quiet
Checking
Suggestion
Clean
Error
```

A suggestion displays:

```text
Correct or Refine
original → replacement
short explanation
SourceDisplay label when useful
Apply
Dismiss
```

The presentation receives display-safe DTOs only.

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

Native Undo remains useful where the host supports coherent replacement.

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

- Observe eligible editable text only through the active binding.
- Exclude protected or secure surfaces through typed binding outcomes.
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
bounded context policy
revision authority
structured Correction[]
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
