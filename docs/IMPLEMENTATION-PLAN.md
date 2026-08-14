# Emenda V0.1 Implementation Plan

> **Frozen V0.1 execution constitution, version 1.0.1. Supersedes version 1.0.0.**

## 1. Role and objective

This plan turns the outcomes in ROADMAP.md into one commit-sized execution sequence. docs/ACCEPTANCE.md is the pass/fail contract.

Build Emenda from the documentation baseline to current-host V0.1 conformance without pausing between gates.

## 2. Execution rules

1. Execute the increments below in order.
2. Complete one independently verifiable decision at a time and commit it with its relevant checks.
3. Append the commit, commands, results, and limitations to docs/EVIDENCE.md after each increment and gate.
4. Treat every gate as an in-run checkpoint: repair, rerun, record, and continue.
5. Do not add OpenRouter code, dependencies, credentials, requests, or live evidence until Increment 7 has passed.
6. Do not add a native text binding until Increment 10 has passed.
7. Stop only for a genuine external blocker or completed Increment 12. Release remains a separate objective.

## 3. Canonical sequence

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

## Increment 0: Documentation baseline + Documentation Gate

Verify the already-frozen canonical Markdown package, reading order, links, hard operating-system invariant, checksums, and Markdown-only clean-room baseline at the starting commit. Do not modify or recommit a constitutional file.

Initialize the supplied docs/EVIDENCE.md as the factual implementation ledger. It is not constitutional policy and remains appendable during the run. Its Baseline section records:

~~~text
constitution version
baseline commit
baseline checksums
host and toolchain facts
evidence-entry format
~~~

Each later entry records:

~~~text
increment or gate
implementation commit
commands run
pass, fail, or blocked result
runtime environment when relevant
known limitations without inflated claims
~~~

Verification:

~~~text
all canonical files exist
cross-references resolve
baseline checksums reproduce
archive contains Markdown only
ledger points to version 1.0.1 and the baseline commit
~~~

Record the Documentation Gate in the ledger and commit the ledger skeleton as the first implementation increment.

## Increment 1: Domain

Create the smallest safe-Rust library scaffold required for pure product code and tests.

Use:

~~~text
#![forbid(unsafe_code)]
standard test runner
no Tauri dependency
no native text dependency
no target-specific module
~~~

Implement:

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
CheckRequest
CheckResult
SuggestionId
SuggestionView
State
typed domain errors and product outcomes
~~~

RevisionId is a monotonically increasing session value. Revision is a sealed value constructed only from a reserved RevisionId plus returned current TextContext; its fields cannot be mutated after construction.

Normalize TextGeometry before it crosses a semantic boundary:

~~~text
finite x and y
finite non-negative width and height
left-to-right and top-to-bottom normalized rectangle
logical pixels, never native device units
coordinates relative to the active Emenda presentation root
no coordinate-space field or native transform crosses the boundary
negative x or y allowed for valid multi-display layouts
unsupported or unreliable geometry represented as None
~~~

Create one platform-neutral JSON fixture corpus for serialized contracts, states, errors, zero-or-one inference decisions, and geometry. Rust tests consume it now. Reserve strict-TypeScript/browser conformance tests against the same fixtures for Increment 9; do not create a browser implementation.

Verification:

~~~text
Rust compiles
pure domain tests pass
Unicode scalar ranges pass
Revision has no mutation path
geometry normalization and rejection cases pass
fixtures contain no native identity or host-specific field
display-safe serialization excludes SourceReference
~~~

Commit.

## Increment 2: TextSurface

Implement the application-owned semantic port:

~~~text
subscribe
context
geometry
replace_if_current
~~~

Define typed SurfaceError outcomes. The subscription publishes `Changed(ObservedChange)` or display-safe `Unavailable { optional SourceDisplay, SurfaceError }`; unavailable signals contain no text or SourceReference. Keep SourceReference opaque and round-trip it only to the same surface.

Verification:

~~~text
port compiles
public API contains semantic vocabulary only
context and replacement contracts use Unicode scalar ranges
geometry returns normalized TextGeometry or None
controller-facing values expose no environment mechanism
~~~

Commit.

## Increment 3: MockTextSurface

Implement deterministic behavior:

~~~text
emit ObservedChange
emit display-safe Unavailable without text or SourceReference
return TextContext
return normalized geometry or None
record replacement requests
simulate changed source
simulate protected source
simulate unsupported operation
~~~

Use a deterministic clock or controllable scheduler seam where timing evidence requires it.

Verification:

~~~text
surface contract tests pass
replacement recording is exact
changed and protected sources remain untouched
all outcomes are reproducible
~~~

Commit.

## Increment 4: InferenceProvider + Mock

Implement platform-neutral request/result types, InferenceProvider, and MockInferenceProvider.

The semantic correction cardinality is exactly:

~~~text
corrections: [] | [Correction]
~~~

`CheckRequest` and `CheckResult` match `SPEC.md`. The serialized result requires `language_profile` and a `corrections` array with zero or one strict `Correction` object. Additional properties, missing fields, and every other cardinality are protocol violations.

MockInferenceProvider supports:

~~~text
one correction
no correction
typed transport failure
typed protocol failure
typed semantic failure
delayed completion for stale-result tests
~~~

Verification:

~~~text
mock outcomes are deterministic
zero-or-one array cardinality is enforced
empty-corrections fixture is valid
multiple, missing, and extra top-level values fail
request and result types contain no provider protocol detail
repository contains no OpenRouter dependency, configuration, code, or live evidence
~~~

Commit.

## Increment 5: Controller, debounce, context, and revision

Implement one controller and one ambient workflow.

Revision lifecycle:

~~~text
eligible ObservedChange arrives
→ reserve a new RevisionId synchronously and make it authoritative
→ invalidate any older request, result, or suggestion
→ restart the 500 ms debounce
→ request bounded current context for the reserved ID
→ reject the return if that ID is no longer current
→ seal one immutable Revision from the ID and returned TextContext
→ pass that exact Revision through inference, validation, and Apply
~~~

A RevisionId is therefore reserved immediately; a Revision exists only after context is returned and is immutable once sealed.

Define:

~~~rust
const MAX_CONTEXT_SCALARS: usize = 2000;
~~~

Context policy:

1. If the changed range itself exceeds 2000 scalars, return `ContextTooLarge` and do not call inference.
2. Use the reliably bounded enclosing sentence when it fits the cap.
3. Otherwise use the reliably bounded enclosing local paragraph when it fits the cap.
4. Otherwise create a Unicode-scalar-safe window containing the complete changed range. Divide the remaining capacity evenly before and after the range, give an odd spare scalar to the trailing side, clamp at document ends, then backfill from the other side.

Check revision currency after debounce, context, inference, geometry, validation, and immediately before replacement.

Verification:

~~~text
rapid changes produce one current context request
newer change becomes authoritative before debounce
context returning for an old ID cannot seal a current Revision
sealed Revision values never mutate
all context results contain at most 2000 Unicode scalars
fallback examples at start, middle, and end are exact
oversized changed range never reaches inference
stale results cannot publish or replace
~~~

Commit.

## Increment 6: Validator + presentation state

Implement strict parsing and semantic validation for zero or one correction:

~~~text
required language_profile and corrections fields
zero or one strict Correction object in the corrections array
additional properties rejected
Unicode scalar range bounds
exact original identity
unique exact-original recovery
insertion and deletion range rules
replacement validity
category and confidence validity
no-op rejection
ambiguous identity rejection
~~~

Implement display-safe states and this exhaustive mapping:

| Current event or outcome | Published state |
|---|---|
| No active work | Quiet |
| New change reserved or debounce pending | Quiet; any older suggestion is invalidated |
| Current context or inference work begins | Checking |
| Exactly one correction validates | Suggestion |
| Valid empty corrections array | Clean |
| Apply succeeds | Quiet |
| Dismiss | Quiet |
| Stale completion | No transition; preserve the newer authoritative state |
| Writer-triggered Apply loses a revision race | Error(StaleRevision); preserve current text |
| Protected, unsupported, transport, protocol, semantic, validation, context, or replacement failure | Error with a typed display-safe kind |

SuggestionView contains SuggestionId, SourceDisplay, exact before/after values, category, optional concise explanation, and optional normalized geometry. It never contains SourceReference.

When explanation is absent, derive the exact category fallback copy defined in `SPEC.md`; do not issue another model request.

Verification:

~~~text
liek → like
Georgian or Russian scalar-range example
ambiguous original
out-of-range correction
no-op replacement
valid insertion and deletion
empty corrections array → Clean
stale → no transition
every typed outcome has one explicit state mapping
DTO serialization contains display-safe values only
missing explanation uses deterministic category copy
~~~

Commit.

## Increment 7: Complete mock product + Mock Product Gate

Connect:

~~~text
MockTextSurface change
→ immediate RevisionId reservation
→ debounce
→ bounded TextContext
→ immutable Revision
→ MockInferenceProvider
→ validator
→ presentation state
→ Apply or Dismiss
→ MockTextSurface.replace_if_current
~~~

Run deterministic scenarios for:

~~~text
one valid correction and Apply
Dismiss
no correction → Clean
rapid changes
stale context and inference
changed source before Apply
protected source
invalid and multiple provider output
missing geometry
context fallback and oversize rejection
typed failures
~~~

Verification:

~~~text
Apply records one exact current replacement
Dismiss records none
Clean records none
invalid, stale, changed, protected, and oversized paths leave source untouched
complete product semantics pass without OpenRouter, Tauri, or a native binding
no external network or credential evidence exists
~~~

Record the Mock Product Gate in docs/EVIDENCE.md and continue immediately.

Commit.

## Increment 8: OpenRouterProvider + Provider Gate

Only after the recorded Mock Product Gate, add OpenRouterProvider, its minimal HTTP dependency, environment configuration, and provider tests.

Use the exact model-output JSON Schema in `SPEC.md`: required `language_profile`, required `corrections`, zero-or-one array cardinality, required correction fields, and `additionalProperties: false` at every object level. Do not accept prose fallback, silent model substitution, or multiple corrections.

Implement:

~~~text
bounded request context
configured model and supported-language-profile instructions
bounded response body
strict parse
typed transport, protocol, and semantic outcomes
observable model identity
credential and secret redaction
~~~

Run unit/contract tests first. Then run strict live evidence using a sealed Revision and at most MAX_CONTEXT_SCALARS.

Provider Gate does not pass until one successful live correction is parsed and validated. Record external failures truthfully. V0.1 performs one provider request per sealed revision and uses no automatic retry, fallback model, or silent substitution.

Verification:

~~~text
request construction is exact
response cardinality is zero or one
live liek → like or equivalent correction succeeds
no native identity or text beyond bounded context is sent
invalid output remains typed and cannot touch source text
~~~

Record the Provider Gate in docs/EVIDENCE.md and continue immediately.

Commit.

## Increment 9: Tauri UI + Presentation Gate

Introduce Tauri as a thin composition root only now.

Implement:

~~~text
strict TypeScript
Zod boundary
Quiet
Checking
Suggestion
Clean
Error(ErrorKind)
Apply
Dismiss
keyboard paths
optional placement from normalized geometry
brand tokens
~~~

Run strict-TypeScript/Zod conformance against the same shared JSON fixtures used by Rust, including the future browser contract fixtures. Differences in field names, scalar units, geometry normalization, state meanings, or error meanings fail the gate.

Apply the brand contrast rule:

~~~text
WCAG 2.2 AA takes precedence over palette preference
normal text contrast is at least 4.5:1
large text, focus indicators, and essential UI graphics are at least 3:1
Steel Gray is not used for essential or normal-size text unless the measured pair passes
Oxblood is never the sole carrier of meaning
~~~

Verification:

~~~text
Rust and Tauri compile
strict TypeScript passes
Rust and TypeScript fixture conformance passes
Zod rejects native identity and malformed states
all explicit state mappings render
keyboard, visible focus, screen-reader labels, and reduced motion pass
actual token pairs pass contrast measurements
geometry absence uses the same stable interaction
~~~

Record the Presentation Gate in docs/EVIDENCE.md and continue immediately.

Commit.

## Increment 10: Architecture Gate

Audit the repository before native binding work.

Verify:

~~~text
complete mock product still passes
controller depends only on TextSurface and InferenceProvider semantics
context and revision policy are platform-neutral
provider protocol is isolated in OpenRouterProvider
frontend receives display-safe state only
Tauri shell contains composition and window lifecycle only
Rust and browser/TypeScript conformance fixtures agree
no native text dependency exists
no platform-named source module shaped a shared contract
one ambient workflow exists; no generalized capability framework exists
every direct dependency has a current owner and justification
~~~

Record searches, dependency inspection, and full mock regression results in docs/EVIDENCE.md.

Repair any failure, rerun the Architecture Gate, record it, and continue immediately.

Commit only factual ledger updates or mechanically useful architecture checks produced by the gate.

## Increment 11: Current-host leaf + Current-Host Binding Gate

Select the binding that matches the available runtime host. In the owner's present environment, implement WindowsTextSurface only as a leaf after Increment 10.

Responsibilities:

~~~text
subscribe to eligible editable-text events
retain native identity privately
retrieve requested bounded context
translate native offsets to Unicode scalar ranges
normalize native geometry to the shared logical-pixel contract
verify current source and expected context
perform one coherent exact replacement
return typed protection, support, and replacement outcomes
~~~

Do not change the controller, provider port, validator, presentation state, or shared domain contract to accommodate the host.

Current-Host Binding Gate verification:

~~~text
deterministic binding contract tests
native offset/scalar translation fixtures
geometry normalization across scale and negative display origins
protected-source outcome
changed-source outcome
controlled host fixture observation and replacement
one coherent edit command
~~~

If the controlled host exposes native Undo, verify one Undo restores the original text. If it does not expose or support Undo, record that fact; lack of host Undo is not converted into an Emenda failure.

Record the Current-Host Binding Gate in docs/EVIDENCE.md and continue immediately.

Commit.

## Increment 12: Two-app runtime + V0.1 Conformance Gate

Run the complete current-host product in:

~~~text
one simple editable application
+
one additional ordinary editable application
~~~

For each application record:

~~~text
application and version
host build and display scale
ambient ObservedChange
immediate revision reservation
one debounced bounded request
successful live provider correction
exact Suggestion rendering
Apply result
Dismiss result
changed-source refusal
stale-result safety
geometry present or absent behavior
Undo result when the host supports Undo
~~~

Then run the complete docs/ACCEPTANCE.md contract, repository health checks, architecture review, dependency review, UX review, brand review, documentation status review, and clean-worktree check.

V0.1 Conformance requires all earlier checkpoints plus both runtime applications. Link every result from the final ledger entry; do not replace earlier evidence with a summary claim.

Repair in-scope failures through small commits and rerun the affected checkpoint and final conformance.

Record the verified implementation commit and final V0.1 Conformance result in docs/EVIDENCE.md.

Commit and push the conformance ledger update, verify the pushed state, and finish with a clean worktree.

## Increment 13: Release later

Do not implement packaging, signing, installers, publisher trust, update delivery, or public distribution during the V0.1 run.

Those concerns begin under a separately authorized Release objective after V0.1 Conformance. Release failures do not redefine product, architecture, binding, or runtime evidence.

## Completion report

Report:

~~~text
constitution version and baseline commit
verified implementation commit
implementation commits by increment
docs/EVIDENCE.md ledger location
checks run and exact results
provider live evidence
runtime applications verified
architecture, dependency, UX, and brand results
conditional Undo results
remaining release and future milestones
~~~
