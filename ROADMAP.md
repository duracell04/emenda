# Emenda Roadmap

> **Frozen V0.1 execution constitution, version 1.0.1. Supersedes version 1.0.0.**

## Role

This roadmap fixes product outcomes, dependency order, and evidence order.

docs/IMPLEMENTATION-PLAN.md defines the commit-sized execution work. docs/ACCEPTANCE.md defines the evidence that makes each checkpoint pass.

## North star

~~~text
Emenda works across supported writing environments
→ learns only through explicitly authorized product capabilities
→ notices the highest-value local issue quietly
→ proposes the smallest precise change in the original application
→ waits for one deliberate writer decision
→ preserves authorship, Duktus, privacy, and flow
~~~

V0.1 proves the first ambient correction loop inside this broader direction. `UX.md` is authoritative for the Product North Star, current interaction, future boundary, and UX decision function.

## One-run rule

V0.1 is one autonomous implementation run. Gates are evidence checkpoints inside that run, never stop points or requests for renewed permission.

At each checkpoint:

~~~text
run the required checks
→ repair any in-scope failure
→ rerun the checkpoint
→ append evidence to docs/EVIDENCE.md
→ continue immediately
~~~

Only a genuine external blocker or completed V0.1 conformance ends the run. Release work is a later objective.

## Canonical sequence

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

No OpenRouter implementation, credential use, network request, or live evidence occurs before the Mock Product Gate is recorded as passing.

## Milestone 0: Documentation baseline + Documentation Gate

Verify the already-frozen version 1.0.1 constitution at the starting commit, then initialize the supplied appendable implementation ledger without changing any constitutional file.

Establish:

~~~text
canonical Markdown package
resolved reading order and cross-references
baseline checksums
hard operating-system invariant
docs/EVIDENCE.md implementation ledger
~~~

The constitution remains frozen during implementation. The ledger records factual implementation evidence without rewriting policy.

Exit evidence:

~~~text
baseline files and checksums recorded
archive verified as Markdown-only
ledger identifies constitution version and baseline commit
~~~

## Milestone 1: Domain

Build the smallest safe-Rust, platform-neutral core and its schemas:

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
normalized TextGeometry
Correction
Suggestion
SuggestionView
CheckRequest
CheckResult
State
typed errors and product outcomes
~~~

Create shared serialization and conformance fixtures usable by Rust, strict TypeScript, and the future browser binding.

Exit evidence:

~~~text
pure domain and Unicode-scalar tests pass
geometry normalization tests pass
shared fixtures contain no host mechanism
no Tauri, native text dependency, or target-specific product type exists
~~~

## Milestone 2: TextSurface

Define the application-owned semantic port:

~~~text
subscribe
context
geometry
replace_if_current
~~~

Exit evidence:

~~~text
port contract compiles
typed outcomes are semantic
opaque identity remains binding-owned
unavailable signals expose no text or opaque identity
public contracts contain no environment mechanism
~~~

## Milestone 3: MockTextSurface

Implement deterministic surface behavior for changed and unavailable signals, bounded context, optional geometry, safe replacement, changed sources, protected sources, and unsupported operations.

Exit evidence:

~~~text
surface contract tests pass
replacement requests are inspectable
all success and failure outcomes are deterministic
~~~

## Milestone 4: InferenceProvider + Mock

Define the semantic inference port and MockInferenceProvider.

The result contract contains exactly zero or one correction in the required array field:

~~~text
corrections: [] | [Correction]
~~~

An empty array means no correction and later maps to `Clean`. A one-item array carries the only candidate for semantic validation. Every other cardinality is a protocol failure.

Exit evidence:

~~~text
mock success, no-correction, and typed failure tests pass
request and result types contain no provider protocol
no OpenRouter code, dependency, configuration, or live evidence exists
~~~

## Milestone 5: Controller, debounce, context, and revision

Implement one deterministic orchestration path.

An eligible change reserves a new RevisionId immediately and invalidates older work before debounce begins. After bounded current context is returned, the controller seals one immutable Revision for that reserved ID.

Use:

~~~text
500 ms default debounce
MAX_CONTEXT_SCALARS = 2000
deterministic sentence/local-paragraph selection
Unicode-scalar-safe fallback containing the complete changed range
typed ContextTooLarge when the changed range alone exceeds the cap
stale checks at every asynchronous boundary
~~~

Exit evidence:

~~~text
rapid changes coalesce
revision authority changes immediately
sealed revisions never mutate
context never exceeds the cap
stale work cannot publish or replace
~~~

## Milestone 6: Validator + presentation state

Implement deterministic correction validation and the display-safe product state machine.

Canonical states:

~~~text
Quiet
Checking
Suggestion
Clean
Error(ErrorKind)
~~~

Exactly one valid correction maps to `Suggestion`; a valid empty correction array maps to `Clean`; stale work publishes no transition; typed failures map explicitly to `Error(ErrorKind)`.

Exit evidence:

~~~text
range, identity, ambiguity, and no-op tests pass
state mapping is exhaustive
frontend DTOs contain SourceDisplay but never SourceReference
~~~

## Milestone 7: Complete mock product + Mock Product Gate

Connect:

~~~text
MockTextSurface
→ controller
→ MockInferenceProvider
→ validator
→ presentation state
→ Apply / Dismiss
→ MockTextSurface.replace_if_current
~~~

Exit evidence:

~~~text
complete product behavior passes without Tauri, OpenRouter, or a native binding
Apply performs one exact current replacement
Dismiss performs none
Clean, stale, protected, changed-source, and invalid-output paths fail closed
Mock Product Gate is recorded in docs/EVIDENCE.md
~~~

This is the dependency boundary after which external provider work may begin.

## Milestone 8: OpenRouterProvider + Provider Gate

Implement the provider adapter using the exact zero-or-one structured-output schema in `SPEC.md`, bounded response handling, typed failure classification, and environment configuration.

Only now run strict live evidence.

Exit evidence:

~~~text
request construction and parsing tests pass
no native identity leaves the provider boundary
invalid output leaves source text untouched
one successful strict live correction is recorded
Provider Gate is recorded in docs/EVIDENCE.md
~~~

## Milestone 9: Tauri UI + Presentation Gate

Add a thin Tauri composition shell and compact strict-TypeScript presentation for the canonical states and actions.

Exit evidence:

~~~text
Rust, Tauri, and strict TypeScript compile
shared Rust/TypeScript conformance fixtures pass
Zod boundary and state rendering tests pass
keyboard, focus, labels, reduced motion, and brand contrast pass
normalized geometry is used only for optional placement
Presentation Gate is recorded in docs/EVIDENCE.md
~~~

## Milestone 10: Architecture Gate

Audit the complete mock product, provider adapter, presentation boundary, dependencies, fixtures, and composition root before native work.

Exit evidence:

~~~text
shared code contains zero operating-system mechanics
controller depends only on semantic ports
frontend receives display-safe values only
Tauri remains composition only
browser conformance fixtures preserve the shared contract
no native text dependency or target-specific shared abstraction exists
Architecture Gate is recorded in docs/EVIDENCE.md
~~~

Fix audit failures and continue; the gate is not a stopping point.

## Milestone 11: Current-host leaf + Current-Host Binding Gate

Implement exactly one leaf binding for the available verification host. The owner's present host permits a WindowsTextSurface, but Windows remains private to that leaf, its tests, and its evidence.

The binding owns observation, private identity, context retrieval, native-to-scalar translation, geometry normalization, source revalidation, coherent replacement, and typed protection/support outcomes.

Exit evidence:

~~~text
binding contract and native fixture tests pass
changed and protected sources remain untouched
geometry crosses the port only in normalized form
one coherent edit is produced
Undo is verified when the exercised host exposes it, otherwise the limitation is recorded
Current-Host Binding Gate is recorded in docs/EVIDENCE.md
~~~

## Milestone 12: Two-app runtime + V0.1 Conformance Gate

Run the complete ambient flow in:

~~~text
one simple editable application
+
one additional ordinary editable application
~~~

Verify observation, one debounced current request, live provider result, exact suggestion, Apply, Dismiss, stale safety, changed-source safety, and conditional host Undo.

Then run the complete architecture, dependency, UX, brand, documentation, and repository health review.

Exit evidence:

~~~text
both application runs are recorded
all prior gate evidence is linked
V0.1 Conformance passes
verified implementation commit is identified
worktree is clean
~~~

## Milestone 13: Release later

Packaging, signing, installers, publisher trust, update delivery, and public distribution begin only under a separately authorized release objective.

Release evidence neither blocks nor redefines V0.1 product conformance.

Future product breadth is likewise chosen from measured evidence and preserves the shared semantics frozen by V0.1.
