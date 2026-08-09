# Emenda V0.1 Acceptance

> **Frozen acceptance standard, version 1.0.0**

## 1. Gate model

```text
Documentation Gate
→ Mock Product Gate
→ Provider Gate
→ Presentation Gate
→ Architecture Gate
→ Current-Host Binding Gate
→ V0.1 Conformance Gate

Release Gate
→ later explicit objective
```

A later gate cannot invalidate independently verified evidence from an earlier gate.

## 2. Documentation Gate

Expected:

```text
all canonical Markdown files present
reading order is consistent
cross-references resolve
hard OS-independence invariant appears in Prompt, Spec, and Architecture
roadmap and implementation plan agree
package contains no source code
checksums verify
```

## 3. Mock Product Gate

This gate must pass before native binding work begins.

### A. Domain purity

Expected shared types:

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
```

Expected:

```text
no operating-system type, API, identifier, path, handle, accessibility object, clipboard format, or keyboard mechanism appears in shared public contracts
```

### B. Correction parsing

Input:

```text
I liek this sentence.
```

Expected:

```text
original: liek
replacement: like
category: spelling
valid Unicode scalar range
```

### C. Unicode correctness

Use at least one Georgian or Russian example.

Expected:

```text
scalar positions resolve to the intended substring
replacement applies to the intended range
```

### D. Debounce

Sequence:

```text
several ObservedChange values arrive inside 500 ms
```

Expected:

```text
timer restarts
one current context request is issued after settling
```

### E. Revision freshness

Sequence:

```text
revision 41 starts
revision 42 starts
revision 41 returns
revision 42 returns
```

Expected:

```text
revision 41 is stale
revision 42 remains authoritative
```

### F. Context policy

Expected:

```text
bounded context respects maximum scalar length
context centers around changed range
sentence or local-paragraph boundary is deterministic
```

### G. Complete mock loop

```text
MockTextSurface emits change
→ controller debounces
→ context is requested
→ MockInferenceProvider returns correction
→ validator accepts correction
→ SuggestionView is produced
→ Apply
→ MockTextSurface records one exact replacement
```

### H. Dismiss

Expected:

```text
no replacement request
suggestion clears
source remains authoritative
```

### I. Changed source

Sequence:

```text
suggestion appears
→ mock source changes
→ Apply
```

Expected:

```text
replace_if_current returns typed failure
no source edit occurs
```

### J. Invalid provider output

Expected:

```text
invalid envelope, schema, range, overlap, or original identity produces a typed outcome
source remains untouched
```

### K. Protected source

Expected:

```text
protected source becomes a typed presentation state
no context is sent or edit applied beyond the binding's allowed boundary
```

## 4. Provider Gate

### A. Request construction

Expected:

```text
bounded TextContext only
configured model
supported language profile instructions
minimal structured-output schema
no source-native identity
```

### B. Response handling

Expected:

```text
bounded response read
strict parse
semantic validation
typed transport/protocol/semantic outcomes
```

### C. Strict live evidence

With credentials and network available:

```text
one request
→ valid correction
or
→ correctly classified external failure
```

At least one successful live correction is required for the V0.1 conformance gate.

## 5. Presentation Gate

Expected:

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
Apply
Dismiss
```

Verify:

```text
strict TypeScript
Zod boundary
keyboard access
visible focus
screen-reader labels
sufficient contrast
reduced-motion handling
SourceDisplay only
SuggestionId-based actions
```

The UI contains no opaque source token or native identity.

## 6. Architecture Gate

This gate occurs before native binding work.

Expected:

```text
complete mock product loop passes
controller depends only on semantic ports
context policy is platform-neutral
provider boundary is isolated
presentation receives display-safe DTOs
Tauri shell is composition only
no native text dependency exists yet
no target-specific module shaped a shared contract
no alternate correction workflow or capability framework exists
```

A repository search should find platform names only in documentation that explains the invariant, future roadmap text, or later binding-specific files.

Shared source and shared tests contain no fake native handles, paths, process identifiers, or platform API names.

## 7. Current-Host Binding Gate

The runtime host determines the binding used for evidence.

The owner's present verification environment is Windows. This fact belongs only to this gate and binding-specific implementation.

### A. Ambient observation

In one simple editable application:

```text
type:
I liek this sentence.

pause
```

Expected:

```text
binding emits ObservedChange
one current request follows debounce
```

### B. Context and suggestion

Expected:

```text
current bounded context reaches provider
liek → like is validated
compact suggestion appears
```

### C. Safe Apply

Expected:

```text
Apply verifies current source and expected context
exact intended text becomes:
I like this sentence.
```

One coherent host edit preserves native Undo where supported.

### D. Dismiss

Expected:

```text
source remains unchanged
```

### E. Changed source

Expected:

```text
source changes before Apply
→ binding refuses replacement
→ current source remains authoritative
```

### F. Second application

Repeat the full loop in one additional ordinary editable application.

### G. Contract preservation

Expected:

```text
native binding required no change to controller, validator, provider port, presentation state, or shared domain contracts
```

## 8. UX Gate

Expected:

```text
writer remains in original application
Emenda reacts after a short pause
exact change is visible
Apply and Dismiss are clear
state is understandable
writing resumes immediately
```

## 9. Brand Gate

Expected:

```text
Paper background
Ink Black primary structure
Graphite / Steel secondary information
Oxblood as rare correction/action accent
Inter for functional UI
Special Elite for restrained brand moments
```

The interface remains quiet, compact, precise, and editorial.

## 10. Dependency Gate

For every direct dependency record:

```text
current capability
owning module
architectural layer
reason it reduces total complexity or risk
```

Expected:

```text
every dependency has a current product justification
native dependencies are leaf-binding scoped
```

## 11. V0.1 Conformance Gate

V0.1 passes when:

```text
Documentation Gate
+ Mock Product Gate
+ Provider Gate with one successful live correction
+ Presentation Gate
+ Architecture Gate
+ Current-Host Binding Gate in two applications
+ UX Gate
+ Brand Gate
+ Dependency Gate
```

## 12. Evidence language

Use:

```text
implemented
compiled
deterministically tested
integration tested
runtime verified
supported
```

Record uncertainty directly.

A passing compile is not runtime support. A passing mock is product architecture evidence. A passing host test is binding evidence.

## 13. Release Gate

Distribution begins under a separate objective.

Possible later evidence:

```text
packaging
signing
installer behavior
publisher trust
update delivery
public distribution
```

Release failures do not redefine product correctness or host support.
