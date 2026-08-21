# Emenda V0.1 UX

> **Frozen interaction authority, version 2.1.0**

## 1. Interaction promise

> **Emenda improves the text while preserving the writer's Duktus and control.**

The writer stays on the page, keeps the original editor as the primary surface, sees at most one exact proposal, and decides whether text changes. Emenda instructs the model never to translate, exposes the complete change for review because structural validation cannot prove that instruction was followed, and never silently edits.

This file owns visible behavior and accessibility. Product behavior and the trust model are authoritative in [`SPEC.md`](SPEC.md), and visual identity is authoritative in [`BRAND.md`](BRAND.md). The writer-approval role implements `EM-PROV-003` and the Apply controls implement `EM-APPLY-001` through `EM-APPLY-003`.

## 2. Activation and revocation

Emenda observes a site only after the writer uses the toolbar action and grants permission for that exact top-level HTTP(S) origin. The action communicates the relevant next step without implying all-sites access:

```text
Enable on this site
Reactivate on this site
```

Each origin is enabled independently. The toolbar action remains an Enable or Reactivate command; it is not an options shortcut. Reusing it on an enabled origin idempotently refreshes authorization. After successful activation with incomplete configuration, Emenda opens Settings and observation stays paused. An invalid or unavailable origin produces a concise permission error and no partial activation.

Enable only origins the writer trusts. Emenda's one-use trusted input ticket rejects unattended synthetic changes, but page work nested in the same genuine editing event or queued ahead of ticket expiry cannot be distinguished completely and may cause one check.

Revoking an origin immediately disables new checks and Apply authorization there, then requests overlay and listener teardown. If a live document cannot be reached, it remains unauthorized and startup or document-lifecycle reconciliation retries cleanup; another enabled origin remains active.

## 3. Writing flow

Emenda observes only a visible, foreground, focused, writable, midpoint-exposed, sequentially keyboard-focusable light-DOM textarea with a collapsed caret. Ordinary input must arrive as the browser's paired trusted `beforeinput` and `input`; synthetic, value-only, unpaired, or expired programmatic input creates no check. After eligible committed input Emenda waits for a 600 ms pause. Moving the caret, covering the textarea midpoint in the DOM hit test, hiding the document, or blurring the window invalidates current work and presentation without retrying when focus returns.

Composition invalidates an old suggestion immediately but produces no check while the writer is composing. Composition end supplies the one committed change. A duplicate terminal input does not create another check.

These outcomes are deliberately silent:

- clean text;
- empty or nonlinguistic focus;
- unsupported language;
- a non-collapsed selection;
- a focus that exceeds its Unicode-scalar limit in [`SPEC.md`](SPEC.md);
- an unsupported or ambiguous surface encountered during ordinary typing;
- stale or cancelled background work.

`Idle`, `Debouncing`, and `Checking` create no persistent visual noise. There is no Clean badge, success toast, or persistent clean state.

## 4. Suggestion overlay

The overlay is fixed to a consistent viewport corner and deliberately unanchored from the caret. It is compact, visually stable, rendered in an Emenda-owned closed shadow root, and present only for a current suggestion or actionable content error. It does not track selection geometry. Page and model strings render literally as text and never become markup, links, or executable content.

One suggestion displays:

```text
category
complete original focus
complete corrected focus with the single changed hunk marked
concise explanation
Apply
Dismiss
```

Insertions and deletions use `[empty]` where needed. Changed whitespace and control, format, or combining scalars use deterministic ASCII names or `U+XXXX` markers, and text runs are bidi-isolated, so invisible or bidirectional changes remain inspectable. Category labels are:

```text
Spelling
Grammar
Punctuation
Style
```

Style suggestions remain local and restrained. Explanations describe the defect without judging the writer.

The exact before-and-after display is also the writer's semantic safeguard. Local validation proves that the proposal is one structural edit; it cannot prove that the model preserved language and meaning.

## 5. Apply and Dismiss

Apply acts only on the current suggestion through its current trusted native button. It enters `Applying`, obtains immediate worker authorization, restores the captured textarea and caret after the deliberate approval-UI handoff, verifies the snapshot, selects and re-verifies the exact correction range internally, changes that range once, and returns to `Idle`. The browser's next native Undo must restore the exact original text in one step.

Emenda consumes the exact self-authored input from Apply internally. A successful Apply does not debounce, request inference, or create another suggestion.

If the source, document, textarea value, captured selection, snapshot, mapping, worker authority, or original substring is no longer current, Apply makes no text mutation and shows a concise actionable error. A moved caret or selection invalidates the proposal. If mutation produces an unexpected external change, Emenda refreshes baseline and authority rather than treating it as acknowledgement; a new check starts only when that change independently has eligible paired provenance.

Suggestion Dismiss invalidates only the current suggestion and preserves page text exactly; error Dismiss clears only the current content error. Either variant best-effort restores the still-current unchanged textarea and captured caret, then returns to `Idle`. Stale suggestion and error controls are inert.

Apply and Dismiss are native buttons operated by pointer or by their ordinary Enter/Space activation after the writer tabs into Emenda UI. V0.1 defines no page-level custom shortcut. Only trusted activation of a current, visible, DOM-hit-test-unobscured, focused Emenda control is accepted; synthetic, stale, hidden, moved, DOM-hit-test-covered, disconnected, or page-focused events do nothing. Page-controlled compositor-only or `pointer-events: none` visual covers are outside that proof boundary, so inline approval is supported only on trusted enabled origins.

## 6. Focus and accessibility

The fixed overlay host follows the current textarea in sequential DOM order, never autofocuses, and never steals focus when it appears, changes, or disappears. Typing continues in the page.

If the writer deliberately tabs into Emenda UI, focus may move among its current controls without invalidating the approval handoff; every control is keyboard operable. Focus leaving both the captured textarea and current Emenda UI invalidates the proposal. After Apply or either Dismiss variant, the unchanged verified textarea regains focus and the appropriate selection when safe.

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

Each new current suggestion or content error produces exactly one polite assistive-technology notification. Hidden debounce and checking activity produces none. Browser Integration owns the accessibility evidence.

## 7. Settings

The options page provides:

- a write-only OpenRouter API-key field;
- a required base model-shaped OpenRouter ID field, with no variant suffix or compiled default;
- profile selection, defaulting to `auto`;
- enabled-origin review and revocation;
- the privacy disclosure below.

The profile choices are `auto`, `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`. A fixed profile is authoritative; `auto` asks the model to identify a supported profile. Text that cannot safely be handled under the selected mode produces no suggestion.

Options communicates only with the service worker. After saving, the API key is never displayed back in full. The model and key never appear in the page or writing overlay. Emenda remains paused with Configuration required until both fields are valid.

Changing the API key, model, or profile cancels any active check and removes any visible suggestion. Emenda resumes on the next committed input; it does not retry the interrupted text. Origin enablement and revocation remain separate controls.

There is no model picker in the writing overlay.

## 8. Errors

Writer-visible errors are concise, typed, and actionable:

| Condition | Visible action |
| --- | --- |
| API key or model missing | Configuration required, with **Open Settings** |
| Site permission unavailable | Explain that Emenda could not be enabled for this site |
| Revocation cleanup incomplete | Explain that the site is disabled and cleanup will be retried |
| Current provider failure or 15-second timeout | Explain that the check failed and writing can continue |
| Invalid profile result or otherwise unsafe provider result | Explain that no safe suggestion could be produced |
| Apply refusal after the writer acts | Explain that authority or the surface changed; claim nothing was applied only when the verified text did not change |

Unsupported capture during ordinary typing and stale background failures remain silent. New eligible committed input clears an obsolete error and begins a new revision.

Any provider fallback occurs inside the same OpenRouter request. Emenda does not retry a failed or timed-out check at the application level.

Errors never display an API key, authorization detail, raw request context, raw model body, source identity, DOM data, or page URL.

## 9. Privacy disclosure

The options page displays this text verbatim:

> The only page text Emenda sends to OpenRouter is the current bounded context for the configured model ID. On a short document, that context can equal all of its text; Emenda sends no separate or unbounded full-document field. Enable only trusted origins: page work nested in a genuine editing event or queued ahead of its ticket expiry may cause one check, and page-controlled visual covers that DOM hit-testing cannot detect may obscure an inline approval control. A catalog ID may itself represent a routing service; Emenda does not infer that from its syntax. OpenRouter may try more than one eligible provider endpoint for the same model inside one request; each attempted endpoint may receive the bounded text. Emenda disables web search, response healing, context compression, and Fusion in every request. OpenRouter account or workspace policies that prevent those overrides are unsupported, can still apply, and may add processing or cost outside Emenda’s request contract. Provider calls use the writer’s OpenRouter quota and may incur charges. Emenda does not send the page URL, editor identity, DOM structure, or Chrome sender metadata to OpenRouter. Emenda requests provider routing that denies data collection, but this is not a guarantee of zero retention; processing remains subject to OpenRouter’s, any enforced account policy, and every attempted provider’s policies. The API key is stored in the browser profile, not in an operating-system secret vault.

Emenda shows no claim that browser-profile storage is an operating-system secret vault. The UI makes no telemetry, analytics, or text-history claim because V0.1 implements none.

## 10. Supported and unsupported surfaces

Positive V0.1 claims cover explicitly enabled, visible, window-focused top-level HTTP(S) pages with one active, visible, writable, midpoint-exposed, sequentially keyboard-focusable light-DOM textarea and a collapsed caret. Its exact value, captured selection, and scalar/UTF-16 replacement range must map losslessly. Exposure is the conservative DOM midpoint hit test in [`SPEC.md`](SPEC.md), not a claim to detect compositor-only visual covers.

Inputs, contenteditable hosts, iframes, shadow-DOM editors, rich, virtualized, canvas, and Google Docs-style editors, restricted or extension pages, file URLs, PDFs, hidden or offscreen surfaces, readonly, disabled, inert, or non-sequential surfaces, and incognito are unsupported. Ordinary typing in them remains silent rather than unreliable.

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

Caret anchoring, inline underlines, multiple suggestions, review-all flows, inputs, contenteditable, complex editors, native surfaces, packaging, signing, and store-publication UX are Deferred. Implementing this frozen UX requires a separate product objective.
