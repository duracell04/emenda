# Emenda UX / UI Principles

> **Frozen UX system, version 1.0.1**

> **Emenda observes quietly, proposes precisely, and lets the writer decide.**

## 1. Current V0.1 interaction

V0.1 has one interaction model:

```text
write
→ pause
→ Emenda checks
→ exact suggestion appears
→ Apply or Dismiss
→ continue writing
```

Platform bindings provide the same experience through different private mechanisms.

V0.1 uses the ambient correction workflow exclusively. Additional interaction models belong to later evidence-driven milestones.

## 2. Product north star

Emenda becomes a continuous editorial layer across supported writing environments while the writer remains in the original application.

The mature product may add richer placement, review, application preferences, personal vocabulary, and additional environment bindings when evidence earns them. Every evolution preserves the same relationship:

```text
Emenda notices quietly
→ proposes the smallest useful change
→ explains it precisely
→ waits for writer intent
→ applies only to the current source
→ leaves authorship with the writer
```

The north star is broader reach and better judgment inside this interaction model, not a replacement editor, an autonomous rewriter, or a model-management dashboard.

## 3. Original context first

The writer remains in the original application.

Emenda appears only when it has a useful suggestion or a meaningful state to communicate.

## 4. Passive observation

Emenda observes eligible editable-text changes automatically.

One short debounce waits for typing to settle.

Initial value:

```text
500 ms
```

The value is one explicit product constant and changes only through measured evidence.

## 5. Explicit application

Every source edit follows an explicit writer action.

```text
suggestion
→ Apply
```

Dismiss preserves the source.

## 6. Smallest useful intervention

Prefer:

```text
exact local correction
→ sentence refinement when clearly useful
→ broader rewrite only under a later explicit objective
```

The author's text remains the source of truth.

## 7. One suggestion, one decision

Each visible suggestion represents one understandable change:

```text
Correct or Refine
original → replacement
short explanation
Apply   Dismiss
```

If the model omits an explanation, Emenda uses the deterministic category copy defined in `SPEC.md` so the interaction still contains a short reason.

## 8. Correct and Refine

```text
Correct
→ spelling
→ grammar
→ punctuation

Refine
→ restrained style
```

Correct receives stronger visual priority.

Refine remains softer and individually reviewable.

## 9. Compact presentation

V0.1 uses one small suggestion surface.

When reliable `TextGeometry` exists, the surface may use it for quiet placement.

When geometry is absent, the same suggestion interaction remains available in a stable compact position. This is presentation placement, not a second product workflow.

The interface remains subordinate to the writing application.

## 10. State model

The canonical product-state enum is:

```text
Quiet
Checking
Suggestion
Clean
Error(ErrorKind)
```

Writer-facing copy maps onto those states rather than creating a second state model:

| Product state | Writer-facing representation |
|---|---|
| `Quiet` | no interruption |
| `Checking` | Checking… |
| `Suggestion` | exact correction with Apply and Dismiss |
| `Clean` | Text looks good |
| `Error(InferenceTransport)` | Connection issue |
| `Error(InferenceProtocol)` | Invalid response |
| `Error(StaleRevision)` | Stale result |
| `Error(ProtectedSurface)` | Protected surface |
| `Error(Replacement)` | Replacement issue |

Background work that becomes stale publishes no state transition. `Error(StaleRevision)` is reserved for a writer-triggered Apply that loses a race with newer authoritative text; the current text remains untouched.

Every exceptional state communicates:

```text
what happened
→ what Emenda preserved
→ next useful action
```

## 11. Reversibility

Apply requests one coherent host edit. When the host exposes coherent native Undo for that edit, the binding preserves it. V0.1 does not claim an independent Emenda undo mechanism.

## 12. Keyboard and accessibility

Primary actions have keyboard paths:

```text
Apply
Dismiss
close suggestion
```

Use:

```text
visible focus
screen-reader labels
sufficient contrast
reduced-motion preference
```

Target WCAG 2.2 AA for Emenda-owned UI.

## 13. Language behavior

Automatic language selection is the default.

Supported profiles:

```text
de-CH
en-GB
en-US
fr-FR
ka-GE
ru-RU
```

Defaults:

```text
German  → de-CH
English → en-GB
```

Clearly American English maps to `en-US`.

Preserve names, quotations, terminology, and short embedded passages.

## 14. Model behavior

The normal writing experience stays model-agnostic.

Default:

```text
openrouter/free
```

Model configuration remains local infrastructure rather than a prominent writing control.

## 15. Privacy visibility

Protected or ineligible surfaces produce a clear quiet state.

Emenda sends only the bounded context required for the current request.

The UI never displays or receives native source identity.

## 16. Visual behavior

Use:

```text
Paper          → primary background
Ink Black      → primary structure
Graphite       → normal secondary text
Steel Gray     → non-text guides or qualifying large text
Oxblood        → rare correction/action cue
Inter          → functional UI
Special Elite  → restrained brand moments
```

Use generous whitespace, precise borders, compact controls, and quiet alignment. Normal text and controls meet WCAG 2.2 AA contrast; Steel Gray is not used for normal-size text on Paper.

## 17. Cross-platform mental model

Every binding preserves:

```text
observe
→ understand current context
→ suggest
→ apply safely
```

The writer learns Emenda once.

Binding mechanics remain invisible.

## 18. V0.1 UX boundary

V0.1 proves the ambient correction loop.

Richer interaction begins after measured evidence, for example:

```text
inline indication
richer anchoring
Review All
per-application behavior
personal vocabulary
```

These are future milestones rather than prebuilt alternate paths.

## 19. UX decision function

When an interaction choice remains open, optimize in this order:

```text
1. Keep the writer in the original context.
2. Show the exact proposed change.
3. Use the fewest useful actions.
4. Preserve host-native reversibility when available.
5. Preserve authorship and Duktus.
6. Keep state explicit and understandable.
7. Reveal complexity only when it materially helps writing.
8. Keep the interface visually quiet.
```

## 20. UX Definition of Done

```text
writer stays in the original application
→ Emenda detects a useful issue automatically
→ the exact proposed change is visible
→ one clear action applies or dismisses it
→ host-native Undo remains available when the host supports a coherent edit
→ writing continues immediately
```
