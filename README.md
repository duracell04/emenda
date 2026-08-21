# Emenda

> **Frozen clean-room constitution, version 2.0.3**

> **Preserve your Duktus**

Emenda V0.1 is specified as a quiet personal writing assistant for Chromium. It observes committed changes on explicitly enabled sites, waits for typing to settle, sends one split bounded context through OpenRouter using the writer's configured model ID, validates the model's complete corrected focus, derives at most one Unicode-scalar edit locally, and presents that exact correction for the writer to apply or dismiss.

This branch currently contains the 13-file Markdown constitution only. Version 2.0.3 supersedes version 2.0.2, preserved at commit `6a4ddc65fa9067f94023f87aebe48840e1b88bc2`; version 2.0.1 remains preserved at `d70b277998a23663ee6befc77dd6bb0da50ebcca`. Product implementation requires a separate future objective in the separate implementation repository.

## Intended V0.1 experience

```text
enable an exact site origin
→ write in a supported surface
→ pause for 600 ms
→ receive zero or one current suggestion
→ Apply or Dismiss
→ continue writing with page focus and one-step Undo preserved
```

V0.1 targets Chrome 140 or newer and supports only active, visible, writable, midpoint-exposed, sequentially keyboard-focusable light-DOM `<textarea>` elements with a collapsed caret in a visible, window-focused top-level page. Ordinary checks require the browser's paired trusted `beforeinput` and `input`; unpaired or synthetic changes create no request. Inputs, contenteditable hosts, iframes, shadow-DOM editors, rich or virtualized editors, Google Docs-style surfaces, restricted pages, file URLs, PDFs, hidden, DOM-hit-test-covered, or offscreen surfaces, readonly or disabled surfaces, and incognito are unsupported.

## Settings and site access

- The writer supplies an OpenRouter API key, one base model-shaped ID without a variant suffix, and a language profile. There is no compiled router-model default or application-level model substitution; a result is accepted only when its model ID exactly matches the request, while syntax alone cannot identify an internally routing catalog service. Live qualification uses and records a documented direct model.
- Provider calls use the writer's OpenRouter quota and may incur charges under that account.
- `auto` is the default profile; fixed profiles are `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`.
- A fixed profile is authoritative. `auto` permits supported-profile detection.
- The toolbar requests optional permission for the exact active HTTP(S) origin and explicit port.
- Enable only trusted origins: one-use paired-input provenance blocks unattended synthetic changes but cannot fully distinguish page work piggybacked on the same genuine editing event, and DOM hit-testing cannot detect every page-controlled visual cover over inline approval controls.
- Revoking an origin immediately removes its authority, then best-effort tears live scripts down and reconciles registration and permission cleanup.
- The worker owns trusted settings. Options communicates with the worker; content scripts never receive the API key or model.

## Privacy

The only page text Emenda-authored provider traffic sends is the bounded context defined in [`SPEC.md`](SPEC.md), not the page URL, a separate or unbounded full-document field, source identity, DOM structure, or Chrome sender metadata. On a short document, that bounded context can equal all of its text. Within-request fallback may expose it to more than one eligible provider endpoint for the same model, and each attempted provider's policy applies. Provider routing requests data-collection denial, but that is not a guarantee of zero retention. The exact options-page disclosure is owned by [`UX.md`](UX.md).

Emenda specifies no telemetry, analytics, correction history, persistent text cache, application-level retry, streaming, or private-text logging. Raw private text never enters logs. A check has a 15-second deadline. OpenRouter may try eligible provider endpoints for the same configured model inside that single request; Emenda itself does not retry or substitute another model.

## Constitutional authority

- [`SPEC.md`](SPEC.md) defines product behavior, safety, compatibility, and failures.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) defines ownership and dependency direction.
- [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md) defines the Documentation baseline and seven future implementation increments.

Supporting documents:

- [`PROMPT.md`](PROMPT.md) — objective and entry point
- [`AGENTS.md`](AGENTS.md) — repository operating constraints
- [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md) — gate verification
- [`docs/ENGINEERING.md`](docs/ENGINEERING.md) — toolchain and evidence policy
- [`UX.md`](UX.md) — visible behavior and accessibility
- [`ROADMAP.md`](ROADMAP.md) — V0.1 and deferred work
- [`BRAND.md`](BRAND.md) — visual identity
- [`docs/EVIDENCE.md`](docs/EVIDENCE.md) — mutable empty evidence template
- [`PACKAGE-MANIFEST.md`](PACKAGE-MANIFEST.md) — freeze identity and checksums

## Scope boundary

Native runtimes, native credential stores, broader editor support, packaging, signing, store publication, release automation, and commercial infrastructure are deferred. Claims remain limited to environments directly tested in a future implementation objective.
