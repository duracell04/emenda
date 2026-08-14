# Emenda V0.1 Acceptance

> **Frozen V0.1 execution constitution, version 1.0.1. Supersedes version 1.0.0.**

## 1. Role and checkpoint model

This file is the pass/fail contract for ROADMAP.md and docs/IMPLEMENTATION-PLAN.md.

V0.1 follows one evidence order:

~~~text
0. Documentation baseline + Documentation Gate
1. Domain
2. TextSurface
3. MockTextSurface
4. InferenceProvider + Mock
5. Controller, debounce, context, and revision
6. Validator + presentation state
7. Complete mock product + Mock Product Gate
8. OpenRouterProvider + Provider Gate
9. Tauri UI + Presentation Gate
10. Architecture Gate
11. Current-host leaf + Current-Host Binding Gate
12. Two-app runtime + V0.1 Conformance Gate
13. Release later
~~~

Gates are checkpoints inside one autonomous V0.1 run, never stop points. On failure, repair the violated in-scope invariant, rerun the checkpoint, append truthful evidence, and continue. Only a genuine external blocker or completed V0.1 Conformance ends the run.

No OpenRouter implementation or live evidence is admissible before the Mock Product Gate. No native binding is admissible before the Architecture Gate.

## 2. Evidence rules

docs/EVIDENCE.md is the implementation ledger. Every increment and gate entry identifies:

~~~text
constitution version 1.0.1
baseline and implementation commit
commands and exact result
deterministic, integration, live, or runtime evidence level
host, application, model, and dependency facts when relevant
limitations and failures without inflated claims
~~~

Later evidence may extend earlier evidence but cannot silently replace or reinterpret it.

## 3. Documentation baseline + Documentation Gate

Expected:

~~~text
canonical Markdown files present
reading order and cross-references resolve
hard operating-system invariant is consistent
roadmap, plan, and acceptance sequence agree exactly
clean-room archive contains Markdown only
constitution files are frozen at version 1.0.1
baseline checksums and commit are recorded
docs/EVIDENCE.md exists as a separate appendable factual ledger
~~~

Normal implementation evidence changes the ledger, not the frozen constitution. A policy change requires an explicit later constitution version.

## 4. Domain

Expected platform-neutral types include:

~~~text
RevisionId
immutable Revision
SourceReference
SourceDisplay
ObservedChange
SurfaceSignal
ContextRequest
TextContext
TextRange
TextGeometry
Correction
Suggestion
SuggestionView
CheckRequest
CheckResult
State
typed outcomes and errors
~~~

RevisionId is monotonically increasing. Revision has no mutation path after it is sealed from an ID and returned context.

TextRange uses half-open Unicode scalar positions.

TextGeometry is normalized before crossing the shared boundary:

~~~text
x and y are finite logical-pixel coordinates
width and height are finite and non-negative
rectangle orientation is left-to-right and top-to-bottom
coordinates are relative to the active Emenda presentation root
no coordinate-space field, native transform, or scale factor crosses the boundary
negative origins remain valid for multi-display layouts
native units remain binding-private
unreliable geometry becomes None
~~~

One shared fixture corpus covers serialized domain values, zero-or-one inference decisions, states, errors, and geometry. Rust consumes it; strict TypeScript and the future browser contract consume the same fixtures without native fields or changed meanings.

## 5. TextSurface

The semantic port provides:

~~~text
subscribe
context
geometry
replace_if_current
~~~

Expected:

~~~text
SourceReference remains opaque
subscription emits Changed only for eligible text and display-safe Unavailable without text or opaque identity
context and replacement use scalar ranges
geometry returns normalized TextGeometry or None
typed outcomes describe product meaning
no host API, identifier, path, handle, DOM object, or input mechanism appears publicly
~~~

## 6. MockTextSurface

Expected deterministic coverage:

~~~text
emit change
emit display-safe Unavailable without text or SourceReference
return bounded context
return normalized geometry or None
record exact replacement
simulate changed source
simulate protected source
simulate unsupported operation
~~~

Changed, protected, and unsupported paths perform no source edit.

## 7. InferenceProvider + Mock

The semantic result contains exactly zero or one correction in its required array field:

~~~text
corrections: [] | [Correction]
~~~

Its strict serialized shape matches `SPEC.md`: required `language_profile`, required `corrections`, zero-or-one array cardinality, strict correction fields, and no additional owned properties. Multiple corrections, missing fields, and extra fields are rejected.

MockInferenceProvider deterministically covers one correction, no correction, typed failures, and delayed stale completion.

Expected at this checkpoint:

~~~text
no OpenRouter code or dependency
no credential or network configuration
no live-provider evidence
~~~

## 8. Controller, debounce, context, and revision

Revision lifecycle:

~~~text
eligible ObservedChange
→ reserve and publish a new authoritative RevisionId immediately
→ invalidate older work
→ debounce
→ request current bounded context for that ID
→ reject stale context
→ seal one immutable Revision
~~~

The RevisionId is reserved before debounce. Revision is created only after context returns and cannot change afterward.

Context policy defines:

~~~rust
const MAX_CONTEXT_SCALARS: usize = 2000;
~~~

Expected:

1. If the changed range alone exceeds the cap, return `ContextTooLarge` and do not call inference.
2. Use the reliably bounded enclosing sentence when it fits the cap.
3. Otherwise use the reliably bounded enclosing local paragraph when it fits the cap.
4. Otherwise use a deterministic Unicode-scalar-safe window containing the complete changed range. Divide spare capacity evenly around the range, give an odd spare scalar to the trailing side, clamp at ends, and backfill from the other side.
5. Check currency after every asynchronous boundary and immediately before replacement.

Rapid changes produce one current request. Old context cannot seal a current Revision. Stale work cannot publish or edit.

## 9. Validator + presentation state

Correction validation requires:

~~~text
strict zero-or-one corrections-array schema
current revision
in-range scalar positions
exact original identity or unique exact-original recovery
valid insertion and deletion range rules
valid replacement, category, and confidence
non-ambiguous and non-no-op result
~~~

State mapping is exhaustive:

| Event or outcome | Published state |
|---|---|
| No active work | Quiet |
| New change or pending debounce | Quiet; invalidate an older suggestion |
| Current context or inference begins | Checking |
| Exactly one correction validates | Suggestion |
| Valid empty corrections array | Clean |
| Apply succeeds or writer Dismisses | Quiet |
| Stale completion | No transition; preserve newer state |
| Writer-triggered Apply loses a revision race | Error(StaleRevision); preserve current text |
| Protected, unsupported, transport, protocol, semantic, validation, context, or replacement failure | Error with typed display-safe kind |

SuggestionView exposes SuggestionId, SourceDisplay, exact before/after values, category, optional explanation, and optional normalized geometry. It never exposes SourceReference or native identity.

When explanation is absent, presentation uses the exact category fallback copy from `SPEC.md` and performs no additional inference request.

## 10. Complete mock product + Mock Product Gate

Required loop:

~~~text
MockTextSurface change
→ immediate RevisionId reservation
→ debounce and bounded context
→ immutable Revision
→ MockInferenceProvider
→ validator
→ presentation state
→ Apply or Dismiss
→ MockTextSurface.replace_if_current
~~~

Required evidence:

~~~text
one valid correction produces Suggestion and one exact Apply
Dismiss produces no replacement
an empty corrections array produces Clean and no replacement
rapid changes coalesce
stale context and inference publish nothing
changed and protected sources remain untouched
invalid, multiple, ambiguous, and no-op output fails closed
context fallback and oversize rejection are exact
missing geometry preserves the same interaction
typed failures map explicitly
~~~

The gate passes only with the complete product semantics running without OpenRouter, Tauri, a native binding, credentials, or network access. Record it and continue.

## 11. OpenRouterProvider + Provider Gate

Only after the Mock Product Gate, implement the adapter.

Request evidence:

~~~text
sealed current Revision
at most MAX_CONTEXT_SCALARS
configured model
supported-language-profile instructions
the exact zero-or-one structured-output schema from SPEC.md
no source-native identity
credentials absent from state and logs
~~~

Response evidence:

~~~text
bounded body
required language_profile and corrections fields
zero or one strict Correction object in the corrections array
additional properties rejected
no prose fallback, automatic retry, fallback model, or silent substitution
typed transport, protocol, and semantic failures
observable model identity
~~~

Provider Gate requires at least one successful strict live correction that parses and validates. External failures are recorded truthfully. V0.1 performs one provider request per sealed revision and uses no automatic retry, fallback model, or silent substitution; a classified failure alone does not pass this gate.

Invalid output never reaches suggestion or source replacement.

## 12. Tauri UI + Presentation Gate

The thin Tauri composition root renders:

~~~text
Quiet
Checking
Suggestion
Clean
Error(ErrorKind)
~~~

Verify:

~~~text
strict TypeScript and Zod boundary
shared Rust/TypeScript/browser conformance fixtures
all explicit state mappings
SourceDisplay only
SuggestionId-based Apply and Dismiss
keyboard access and visible focus
screen-reader labels
reduced-motion handling
normalized geometry or stable no-geometry placement
~~~

Brand contrast rule:

~~~text
WCAG 2.2 AA overrides palette preference
normal text is at least 4.5:1
large text, focus indicators, and essential graphics are at least 3:1
Steel Gray is not essential or normal-size text unless its measured pair passes
Oxblood is never the only carrier of meaning
actual rendered token pairs are measured
~~~

The UI remains compact, quiet, and subordinate to the original application. Record the gate and continue.

## 13. Architecture Gate

Before native work, verify:

~~~text
complete mock product still passes
controller depends only on semantic ports
context, revision, validation, and presentation are platform-neutral
OpenRouter protocol is isolated in OpenRouterProvider
frontend receives display-safe DTOs only
Tauri owns composition and window lifecycle only
Rust and strict-TypeScript/browser fixtures agree
no native text dependency or target-specific shared abstraction exists
no fake native handles, paths, process IDs, or platform APIs exist in shared tests
one ambient workflow exists without a generalized capability framework
every direct dependency has a current owner and justification
~~~

Repair failures, rerun, record the Architecture Gate, and continue.

## 14. Current-host leaf + Current-Host Binding Gate

The current verification host selects one private leaf binding. On the owner's present host this may be WindowsTextSurface; that name and its mechanisms remain in binding code, binding tests, and evidence only.

Expected:

~~~text
ambient eligible-change observation
private native identity
bounded context retrieval
native-offset to Unicode-scalar translation
normalized logical-pixel geometry or None
current-source and expected-context verification
one coherent exact replacement
typed protected, changed, unsupported, and replacement outcomes
no shared contract changes
~~~

Current-Host Binding Gate uses deterministic contract tests plus a controlled native fixture. Changed and protected sources remain untouched.

Undo is conditional on the exercised host: when native Undo is exposed, one Undo must restore the original text after Apply; when it is not exposed or supported, record that limitation rather than claiming or failing Emenda-owned Undo behavior.

Record the gate and continue.

## 15. Two-app runtime + V0.1 Conformance Gate

Run the complete ambient product in:

~~~text
one simple editable application
+
one additional ordinary editable application
~~~

For both applications record:

~~~text
application/version and host facts
ObservedChange
immediate RevisionId reservation
one debounced bounded request
successful live provider correction
exact Suggestion
Apply and Dismiss
stale-result safety
changed-source safety
geometry-present or geometry-absent behavior
Undo result when supported by that host
~~~

V0.1 Conformance requires:

~~~text
frozen constitution and complete docs/EVIDENCE.md ledger
Domain
TextSurface
MockTextSurface
InferenceProvider + Mock
Controller, debounce, context, and revision
Validator + presentation state
Mock Product Gate
Provider Gate with successful live correction
Presentation Gate
Architecture Gate
Current-Host Binding Gate
two-application runtime evidence
UX and brand review
dependency review
repository health checks
verified implementation commit and clean worktree
~~~

Use precise evidence language: implemented, compiled, deterministically tested, integration tested, live tested, runtime verified, and supported. Compilation is not runtime support; mock evidence is not host evidence.

## 16. Release later

Packaging, signing, installers, publisher trust, update delivery, and public distribution require a later explicitly authorized Release objective.

Release evidence does not block or redefine V0.1 product conformance.
