# Emenda UX / UI Principles

> **Emenda observes quietly, proposes precisely, and lets the writer decide.**

## 1. Product UX Goal

### Decision
Emenda stays inside the writer’s existing workflow and keeps the original application as the primary surface.

### Why
Writing remains the user’s main task. A separate editor adds context switching and weakens the feeling that Emenda is a lightweight editorial layer.

### Deterministic behaviour

```text
write
→ detect
→ indicate
→ suggest
→ decide
→ apply
→ continue writing
```

Every UX feature should strengthen this loop.

---

## 2. Correct Before Refine

### Decision
Emenda separates **Correct** from **Refine**.

```text
Correct
→ spelling
→ grammar
→ punctuation
→ capitalization
→ clear word misuse

Refine
→ clarity
→ precision
→ concision
→ restrained stylistic improvement
```

Correct receives the primary visual and interaction priority. Refine remains softer and secondary.

### Why
Correctness is usually objective and low-risk. Style is contextual and more closely tied to the author’s Duktus. Separating them lets users trust automatic detection without feeling that Emenda is rewriting their voice.

### Deterministic behaviour
A suggestion belongs to exactly one category: `Correct` or `Refine`.

---

## 3. Passive Detection, Explicit Application

### Decision
Emenda analyses eligible text automatically after a short typing pause. Every edit is applied through an explicit user action.

### Why
Passive detection removes friction. Explicit application preserves authorship and gives the writer final editorial control.

### Deterministic behaviour

```text
typing stops briefly
→ snapshot current text
→ analyse
→ show suggestion
→ user applies or dismisses
```

A newer text snapshot supersedes an older pending result.

---

## 4. Three Levels of Visibility

### Decision
Emenda uses progressive disclosure.

```text
Level 1  Quiet status
Level 2  Inline signal
Level 3  Suggestion surface
```

### Why
Most writing needs little attention. The interface should reveal detail only when a useful correction exists or the user asks for it.

### Deterministic behaviour

**Level 1 — Quiet status**

```text
Ready
Checking…
2 suggestions
Text looks good
Paused
```

**Level 2 — Inline signal**

Use a restrained underline or marker when exact text geometry is reliable.

**Level 3 — Suggestion surface**

```text
original → replacement
short reason

Apply   Dismiss
```

Deeper explanation appears on request.

---

## 5. Smallest Reliable Surface

### Decision
Emenda selects the richest reliable interaction supported by the current text surface.

### Why
Desktop applications expose text differently. A consistent workflow matters more than identical rendering across every host application.

### Deterministic capability ladder

```text
1. Inline anchored correction
2. Floating anchored widget
3. Compact review panel
4. Selected-text correction
5. Copy corrected text
```

Use the first reliable level available.

This keeps the user-facing model constant across Windows, macOS, Linux, browsers, and ChromeOS while allowing platform adapters to differ internally.

---

## 6. One Suggestion, One Decision

### Decision
Each suggestion represents one understandable change.

### Why
Small, attributable changes are easier to trust, review, apply, undo, test, and learn from than opaque whole-passage rewrites.

### Deterministic card content

```text
category
original → replacement
short reason

Apply   Dismiss
```

Optional secondary actions appear only when relevant:

```text
Explain
Add to vocabulary
```

---

## 7. Preserve the Author’s Duktus

### Decision
Emenda proposes the smallest useful intervention.

### Why
The author’s existing text is the source of truth. Localised corrections improve quality while keeping wording, rhythm, register, terminology, and personality recognisably the writer’s own.

### Deterministic behaviour

Prefer:

```text
exact local correction
```

before:

```text
sentence rewrite
```

and prefer:

```text
sentence rewrite
```

before:

```text
passage rewrite
```

Use the smallest scope that solves the identified issue.

---

## 8. Review All for Dense Text

### Decision
A compact **Review All** surface appears when several suggestions exist.

### Why
One-by-one review works well for normal writing, while longer passages benefit from a faster overview.

### Deterministic behaviour

```text
multiple suggestions
→ Review All
→ Correct / Refine grouping
→ inspect individually
→ Apply selected
```

High-confidence Correct suggestions may support a batch action. Refine remains separately reviewable.

---

## 9. Emenda Indicator

### Decision
A small Emenda indicator acts as the ambient control surface.

### Why
The user needs one predictable place to understand Emenda’s state without opening a full application window.

### Deterministic behaviour

The indicator may expose:

```text
current state
suggestion count
Review All
Pause for this app
Settings
```

It can anchor near the active writing surface, move to an edge, collapse, or remain hidden while inline suggestions stay active.

---

## 10. Per-Application Control

### Decision
Emenda stores simple local preferences for each application.

### Why
Writing context changes by application. Email, messaging, word processing, and code editing can benefit from different levels of assistance.

### Deterministic app preference

```text
AppPreference
- app identity
- active / paused
- language override
- Correct only / Correct + Refine
```

Default:

```text
Active
Automatic language
Correct
```

The current app can be changed quickly through the indicator or tray/menu-bar control.

---

## 11. Language Behaviour

### Decision
Language detection stays automatic by default.

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

### Why
Language selection is infrastructure rather than the writing task. Automatic routing keeps the interface quiet while preserving the user’s preferred language varieties.

### Deterministic behaviour
Use the dominant language of the current text span. Preserve names, quotations, terminology, and short embedded passages in another language.

A manual override remains available in Settings and per-app preferences.

---

## 12. Model Choice

### Decision
The normal writing surface stays model-agnostic.

Default:

```text
openrouter/free
```

A searchable model selector lives in Settings.

### Why
The user cares about correction quality, speed, and cost more than provider mechanics. OpenRouter keeps model choice flexible without turning model management into the product.

### Deterministic behaviour

Normal writing:

```text
use configured default model
```

Important text:

```text
optionally choose a stronger model
→ run check
→ continue with configured default afterward
```

The exact model remains visible to advanced users.

---

## 13. Personal Vocabulary

### Decision
Valid unfamiliar terms can be added directly from a suggestion.

### Why
Repeated names, organisations, Swiss vocabulary, legal terms, and technical expressions should become frictionless over time.

### Deterministic behaviour

```text
Dismiss
Add to vocabulary
```

Confirmation:

```text
Added “[term]” to vocabulary
```

Settings provides a simple searchable vocabulary list with removal and optional import/export.

---

## 14. Settings

### Decision
Settings contains only controls that materially affect writing.

### Why
A compact settings surface keeps infrastructure subordinate to the product experience.

### Deterministic information architecture

```text
Writing
- automatic checking
- Correct / Correct + Refine
- hotkey

Language
- automatic / manual profile
- per-app override

AI
- OpenRouter API key
- connection test
- model

Apps
- active / paused
- per-app preferences

Vocabulary
- added terms
```

Diagnostics and version information remain secondary.

---

## 15. First-Run Experience

### Decision
Onboarding teaches Emenda through one successful correction.

### Why
The interaction model becomes understandable faster through direct use than through explanatory screens.

### Deterministic flow

```text
Launch
→ enter OpenRouter API key
→ test connection
→ model = openrouter/free
→ language = Automatic
→ show hotkey
→ optional sample correction
→ Ready
```

Sample:

```text
I liek this sentence.
→
liek → like
```

After setup, Emenda enters its normal quiet state.

---

## 16. System States

### Decision
Every meaningful state has one explicit user-facing representation.

### Why
A deterministic state model prevents failures from looking like successful “no correction” results and makes recovery understandable.

### Canonical states

```text
Ready
Checking…
Suggestions found
Text looks good
Paused
Protected field
Connection error
Authentication error
Rate limited
Model unavailable
Invalid response
Stale result
Replacement issue
```

### Recovery rule

Every exceptional state communicates:

```text
what happened
→ what Emenda preserved
→ next useful action
```

Example:

```text
The text changed while Emenda was checking it.

Your current text remains unchanged.

Check current text
```

---

## 17. Protected Surfaces

### Decision
Emenda recognises sensitive input surfaces and shows a clear protected state.

### Why
A writing assistant should make its operating boundary visible and predictable.

### Deterministic behaviour

Examples:

```text
password field
payment field
authentication prompt
secure credential input
```

State:

```text
Protected field
Emenda inactive here
```

---

## 18. Speed and Request Behaviour

### Decision
Local UI remains responsive while inference runs asynchronously.

### Why
Perceived speed depends on immediate interface feedback even when model latency varies.

### Deterministic behaviour

```text
short debounce
→ smallest sufficient text span
→ asynchronous inference
→ cancel stale request
→ validate current snapshot
→ show result
```

Use caching for identical recent checks where it reduces unnecessary API calls.

---

## 19. Reversibility

### Decision
Applied corrections follow the host application’s normal undo expectations whenever the host supports them.

### Why
Easy reversibility increases trust and makes accepting a suggestion feel low-risk.

### Deterministic behaviour

```text
Apply
→ one coherent host edit
→ native Undo restores previous text
```

When direct replacement is unavailable, present corrected text for explicit copy/use.

---

## 20. Keyboard and Accessibility

### Decision
Every primary Emenda action has a keyboard-accessible path and Emenda-owned UI targets WCAG 2.2 AA.

### Why
Writing is keyboard-heavy, and accessibility benefits also improve speed, predictability, and clarity for all users.

### Deterministic behaviour

Support keyboard access to:

```text
open focused suggestion
next / previous suggestion
Apply
Dismiss
Review All
close and return to writing
Settings
```

Use:

```text
visible focus
screen-reader labels
sufficient contrast
reduced-motion preference
```

---

## 21. Visual Behaviour

### Decision
The UX inherits the Emenda brand as a functional system.

### Why
Visual restraint supports the product’s low-interruption behaviour and makes the interface feel like one coherent editorial instrument.

### Deterministic visual hierarchy

```text
Paper          → primary background
Ink Black      → primary structure
Graphite/Steel → secondary information
Oxblood        → rare meaningful action/correction cue
Inter          → functional UI
Special Elite  → restrained brand moments
```

Use generous whitespace, precise borders, compact controls, and quiet alignment.

---

## 22. Cross-Platform Rule

### Decision
The same user-facing mental model applies everywhere.

```text
detect
→ indicate
→ suggest
→ apply
```

### Why
Platform-specific text access is an implementation detail. The writer should learn Emenda once.

### Deterministic behaviour

```text
Windows  → Windows adapter
macOS    → macOS adapter
Linux    → Linux adapter
Browser  → browser adapter
ChromeOS → browser extension
```

Each adapter exposes the strongest reliable interaction from the capability ladder defined above.

---

## 23. UX Decision Function

When an implementation detail remains open, choose the option that best satisfies this order:

```text
1. Keep the writer in the original context.
2. Show the exact proposed change.
3. Use the fewest useful actions.
4. Preserve reversibility.
5. Preserve authorship and Duktus.
6. Keep state explicit and understandable.
7. Reveal complexity only when useful.
8. Keep the interface visually quiet.
```

### Why
This gives future developers and AI coding agents a stable decision function for cases the specification does not enumerate.

---

## 24. UX Definition of Done

A core Emenda interaction succeeds when:

```text
writer stays in the original application
→ Emenda detects a useful issue
→ the exact proposed change is visible
→ one clear action applies or dismisses it
→ the result remains reversible
→ writing continues immediately
```

This is the UX outcome every implementation step should protect.
