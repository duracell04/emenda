# Emenda V0.1 UX

> **Frozen interaction authority, version 2.0.1**

## 1. Interaction promise

> **Emenda improves the text while preserving the writer's Duktus and control.**

The writer stays on the page, keeps the original editor as the primary surface, sees at most one exact proposal, and decides whether text changes. Emenda never translates and never silently edits.

This file owns visible behavior and accessibility. Product behavior is authoritative in [`SPEC.md`](SPEC.md), and visual identity is authoritative in [`BRAND.md`](BRAND.md).

## 2. Activation and revocation

Emenda observes a site only after the writer uses the toolbar action and grants permission for that exact top-level HTTP(S) origin. The action communicates the relevant next step without implying all-sites access:

```text
Enable on this site
Enabled on this site
Open Settings
```

Each origin is enabled independently. An invalid or unavailable origin produces a concise permission error and no partial activation.

Revoking an origin immediately disables new checks there and removes any current Emenda overlay. Already injected content becomes inert; another enabled origin remains active.

## 3. Writing flow

Emenda observes only a visible, focused, writable supported surface with a collapsed caret. After committed input it waits for a 600 ms pause before checking.

Composition invalidates an old suggestion immediately but produces no check while the writer is composing. Composition end supplies the one committed change. A duplicate terminal input does not create another check.

These outcomes are deliberately silent:

- clean text;
- empty or nonlinguistic focus;
- unsupported language;
- a non-collapsed selection;
- context whose focus exceeds the 1,200-scalar bound;
- an unsupported or ambiguous surface encountered during ordinary typing;
- stale or cancelled background work.

`Idle`, `Debouncing`, and `Checking` create no persistent visual noise. There is no Clean badge, success toast, or persistent clean state.

## 4. Suggestion overlay

The overlay is fixed to a consistent viewport corner and deliberately unanchored from the caret. It is compact, visually stable, rendered in an Emenda-owned shadow root, and present only for a current suggestion or actionable error. It does not track selection geometry.

One suggestion displays:

```text
category
exact original → exact replacement
concise explanation
Apply
Dismiss
```

Insertions and deletions keep before and after states distinguishable. Category labels are:

```text
Spelling
Grammar
Punctuation
Style
```

Style suggestions remain local and restrained. Explanations describe the defect without judging the writer.

## 5. Apply and Dismiss

Apply acts only on the current suggestion. It enters `Applying`, changes the verified current surface once, and returns to `Idle`. The browser's next native Undo must restore the exact original text in one step.

Emenda consumes the exact self-authored input from Apply internally. A successful Apply does not debounce, request inference, or create another suggestion.

If the source, focus, text, snapshot, mapping, or original substring is no longer current, Apply makes no mutation and shows a concise actionable error. If the mutation produces an unexpected external change, that change follows the ordinary committed-input flow instead of being treated as Emenda's acknowledgement.

Dismiss invalidates the current suggestion, preserves page text exactly, and returns to `Idle`.

Keyboard behavior exists only while a current suggestion is available:

- `Escape` dismisses.
- `Alt+Enter` applies.

These shortcuts must not interfere with IME composition or ordinary host shortcuts.

## 6. Focus and accessibility

The overlay never autofocuses and never steals focus when it appears, changes, or disappears. Typing continues in the page.

If the writer deliberately tabs into Emenda UI, focus order is logical and every control is keyboard operable. After Apply or Dismiss, the verified writing surface remains or regains focus when a coherent browser focus path exists.

Emenda-owned overlay and options UI target WCAG 2.2 AA:

- native semantic controls and accurate accessible names;
- understandable suggestion, status, and error text;
- keyboard operation without a pointer;
- clearly visible focus using more than color alone;
- sufficient text, control, and meaningful non-text contrast;
- meaning conveyed by text or shape as well as color;
- reduced-motion support;
- stable placement and restrained motion;
- no focus change caused solely by suggestion arrival.

Suggestions and errors must be available to assistive technology without repeatedly announcing hidden debounce or checking activity. Browser Integration owns the accessibility evidence.

## 7. Settings

The options page provides:

- a write-only OpenRouter API-key field;
- a required concrete structured-output model field;
- profile selection, defaulting to `auto`;
- enabled-origin review and revocation;
- the privacy disclosure below.

The profile choices are `auto`, `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`.

Options communicates only with the service worker. After saving, the API key is never displayed back in full. The model and key never appear in the page or writing overlay.

Changing the API key, model, or profile cancels any active check and removes any visible suggestion. Emenda resumes on the next committed input; it does not retry the interrupted text. Origin enablement and revocation remain separate controls.

There is no model picker in the writing overlay.

## 8. Errors

Writer-visible errors are concise, typed, and actionable:

| Condition | Visible action |
| --- | --- |
| API key or model missing | Configuration required, with **Open Settings** |
| Site permission unavailable | Explain that Emenda could not be enabled for this site |
| Current provider failure or timeout | Explain that the check failed and writing can continue |
| Invalid response or fixed-profile `LanguageMismatch` | Explain that no safe suggestion could be produced |
| Apply refusal after the writer acts | Explain that the text or surface changed and nothing was applied |

Unsupported capture during ordinary typing and stale background failures remain silent. New committed input clears an obsolete error and begins a new revision.

Errors never display an API key, authorization detail, raw request context, raw model body, source identity, DOM data, or page URL.

## 9. Privacy disclosure

The options page displays this text verbatim:

> Emenda sends only the current bounded text context, up to 1,200 Unicode scalars, to OpenRouter and the provider serving the configured model. It does not send the page URL, full document, source identity or DOM structure. Processing remains subject to OpenRouter’s and the model provider’s policies. The API key is stored in the browser profile, not in an operating-system secret vault.

Emenda shows no claim that browser-profile storage is an operating-system secret vault. The UI makes no telemetry, analytics, or text-history claim because V0.1 implements none.

## 10. Supported and unsupported surfaces

Positive V0.1 claims cover explicitly enabled top-level HTTP(S) pages with a visible, focused, writable light-DOM textarea or the bounded contenteditable grammar in [`SPEC.md`](SPEC.md). Exact logical text and the replacement span must map losslessly.

Inputs, iframes, shadow-DOM editors, rich, virtualized, canvas, and Google Docs-style editors, restricted or extension pages, file URLs, PDFs, readonly or disabled surfaces, ambiguous contenteditable structures, and incognito are unsupported. Ordinary typing in them remains silent rather than unreliable.

## 11. Visual behavior

Apply [`BRAND.md`](BRAND.md) to a quiet functional interface: compact controls, precise borders, generous internal spacing, readable before/after text, restrained accent color, stable placement, and no decorative motion that competes with writing.

The overlay does not add caret anchoring, inline underlines, multiple-suggestion review, or persistent status chrome.

## 12. UX completion and boundary

The future implemented experience is complete when a writer can:

```text
enable one origin
→ write in a supported surface
→ pause
→ receive at most one exact suggestion without losing focus
→ Apply or Dismiss
→ Undo an Apply in one native step
→ continue writing without a self-triggered check
```

Caret anchoring, inline underlines, multiple suggestions, review-all flows, inputs, complex editors, native surfaces, packaging, signing, and store-publication UX are deferred.

The current v2.0.1 objective freezes documentation only. Implementing this UX requires a separate future objective.
