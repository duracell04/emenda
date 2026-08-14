# Emenda V0.1 UX

> **Frozen interaction specification, version 2.0.0**

## 1. UX north star

> **Emenda improves the text while preserving the writer's Duktus and control.**

The writer remains on the page, keeps focus in the original editor, sees one exact proposal, and decides whether the page changes.

## 2. Activation

Emenda begins on an origin only after the writer activates the toolbar action and grants permission for that exact top-level HTTP(S) origin.

The action communicates one of three simple conditions:

```text
Enable on this site
Enabled on this site
Open settings
```

Revoking an origin removes Emenda from that origin. Another origin remains inactive until separately enabled.

## 3. Eligible writing moment

Emenda observes only a visible, focused, writable supported surface. It waits for committed input and a 600 ms pause. Composition activity clears old authority immediately while allowing the writer to finish the composition before a check begins.

Empty, whitespace-only, nonlinguistic, unsupported-language, and clean outcomes remain silent.

## 4. Product states

```text
Idle
Debouncing
Checking
Suggestion
Applying
Error
```

`Idle`, `Debouncing`, and `Checking` do not create persistent visual noise. There is no Clean badge, success toast, or persistent clean state.

A writer-triggered failure may appear when action is useful, for example missing configuration, permission loss, provider failure, timeout, or a changed surface that refuses Apply. Stale background failures remain silent.

## 5. Overlay

The overlay is:

- fixed to the viewport;
- deliberately unanchored from selection geometry;
- compact and visually stable;
- rendered inside a shadow root;
- shown only for a current suggestion or writer-triggered failure;
- focus-neutral on appearance;
- dismissible without editing text.

It uses a consistent viewport corner that avoids covering the browser's primary editing focus where practical. It does not track the caret or animate across the page.

## 6. Suggestion content

One suggestion shows:

```text
category
exact original → exact replacement
concise explanation
Apply
Dismiss
```

The before and after strings remain visually distinct for insertion and deletion. Category labels are plain and restrained:

```text
Spelling
Grammar
Punctuation
Style
```

The explanation states the local reason without implying that Emenda rewrote or improved the writer as a person.

## 7. Actions

Apply:

- submits only the current `SuggestionId`;
- enters `Applying`;
- changes the verified current surface once;
- returns to `Idle` after success;
- produces one browser Undo step;
- shows a quiet writer-triggered refusal if the surface changed.

Dismiss:

- invalidates the current suggestion;
- preserves page text exactly;
- returns to `Idle`.

Keyboard:

- `Escape` dismisses the current suggestion.
- `Alt+Enter` applies the current suggestion.

Keyboard handling is active only when Emenda has a current suggestion and must not interfere with IME composition or ordinary host shortcuts.

## 8. Focus

The overlay never autofocuses and never steals focus when it appears, updates, or disappears. Page typing continues uninterrupted.

If the writer explicitly tabs into the overlay, focus order is logical and all controls are operable. After Apply or Dismiss, focus returns to the verified writing surface when the browser operation preserves a coherent focus path.

Visible focus uses more than color alone.

## 9. Accessibility

Emenda-owned UI targets WCAG 2.2 AA:

- semantic controls;
- accessible names;
- understandable status and error text;
- keyboard operation;
- visible focus;
- sufficient text, control, and meaningful non-text contrast;
- color paired with text or shape;
- reduced-motion support;
- stable placement and restrained animation;
- no focus change caused solely by suggestion arrival.

Presentation and accessibility evidence is collected in the Browser Integration Gate.

## 10. Language and authorship

Profiles are `auto`, `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`. Unsupported language fails closed.

The correction preserves:

```text
meaning
voice
register
terminology
names
quotations
rhythm
Duktus
```

Emenda never translates. Style corrections are restrained and local.

## 11. Settings

The options page provides:

- a write-only OpenRouter API-key field;
- a required concrete structured-output model field;
- language-profile selection;
- enabled-origin review and revocation;
- concise privacy disclosure.

After saving, the key is never displayed back in full. Content scripts see only whether a key exists. The disclosure states that the key is stored in the browser profile and is not protected by an operating-system secret vault.

There is no prominent model picker in the writing overlay.

## 12. Errors

Writer-visible errors are concise, typed, and actionable:

- configuration required;
- site permission unavailable;
- current surface unsupported;
- correction expired because the text changed;
- provider unavailable or timed out;
- response could not be validated.

Errors contain no raw request text, model body, source identity, or credential detail. A later committed input clears obsolete presentation and begins a new revision.

## 13. Supported and excluded surfaces

Positive V0.1 claims cover enabled top-level HTTP(S) pages with visible, focused, writable light-DOM textareas and conventional contenteditable surfaces whose mapping is lossless.

Inputs, iframes, shadow DOM, rich, virtualized, canvas, and Google Docs-style editors, restricted pages, file URLs, PDFs, readonly or disabled surfaces, and incognito are presented as unsupported rather than unreliable.

## 14. Visual behavior

Use:

```text
Paper       → overlay and options background
Ink Black   → structure, text, primary action
Graphite    → secondary text
Steel Gray  → qualifying large text and non-text guides
Oxblood     → rare correction or failure accent
Inter       → functional interface
Special Elite → restrained brand moments
```

Keep spacing generous, controls compact, borders precise, and movement quiet.

## 15. UX decision function

```text
1. Preserve writer control and page focus.
2. Show the exact proposed change.
3. Keep authority and staleness understandable.
4. Use the fewest useful actions.
5. Preserve one-step browser Undo.
6. Preserve meaning and Duktus.
7. Make unsupported conditions explicit.
8. Meet accessibility requirements.
9. Keep the interface visually quiet.
```

## 16. V0.1 UX Definition of Done

```text
writer enables an origin
→ writes in a supported surface
→ pauses
→ receives at most one exact suggestion without losing focus
→ applies or dismisses
→ Apply is safe and reversible with one Undo
→ writing continues
```

## 17. Deferred UX

Caret anchoring, inline underlines, geometry APIs, multiple suggestions, review-all flows, inputs, complex editors, native surfaces, packaging, signing, and store-publication UX are later evidence-led objectives.
