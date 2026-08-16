# Emenda

> **Frozen clean-room constitution, version 2.0.2**

> **Preserve your Duktus**

Emenda V0.1 is specified as a quiet personal writing assistant for Chromium. It observes committed changes on explicitly enabled sites, waits for typing to settle, sends one split bounded context through the default `openrouter/free` route or an advanced concrete-model override, validates the model's complete corrected focus, derives at most one Unicode-scalar edit locally, and presents that exact correction for the writer to apply or dismiss.

This branch currently contains the 13-file Markdown constitution only. Version 2.0.2 supersedes version 2.0.1, preserved at commit `d70b277998a23663ee6befc77dd6bb0da50ebcca`; version 2.0.0 remains preserved at `a1a13607867db8e6eb2ea904f6387ba130f22ce7`. Product implementation requires a separate future objective in the separate implementation repository.

## Intended V0.1 experience

```text
enable an exact site origin
→ write in a supported surface
→ pause for 600 ms
→ receive zero or one current suggestion
→ Apply or Dismiss
→ continue writing with page focus and one-step Undo preserved
```

V0.1 targets Chrome 140 or newer and supports visible, focused, writable, light-DOM `<textarea>` elements and the bounded `contenteditable` grammar defined in [`SPEC.md`](SPEC.md). Inputs, iframes, shadow-DOM editors, rich or virtualized editors, Google Docs-style surfaces, restricted pages, file URLs, PDFs, readonly surfaces, and incognito are unsupported.

## Settings and site access

- The writer supplies an OpenRouter API key and a language profile. The model route defaults to `openrouter/free`; advanced users may override it with a concrete model ID.
- `auto` is the default profile; fixed profiles are `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`.
- A fixed profile is authoritative. `auto` permits supported-profile detection.
- The toolbar requests optional permission for the exact active HTTP(S) origin.
- Revoking an origin makes live content scripts inert and removes its registration and permission.
- The worker owns trusted settings. Options communicates with the worker; content scripts never receive the API key or model.

## Privacy

Emenda sends only the bounded context defined in [`SPEC.md`](SPEC.md), not the page URL, full document, source identity, or DOM structure. Provider routing requests data-collection denial, but that is not a guarantee of zero retention. The exact options-page disclosure is owned by [`UX.md`](UX.md).

Emenda specifies no telemetry, analytics, correction history, persistent text cache, application-level retry, streaming, or persistent private-text logging. A check has a 15-second deadline. OpenRouter may select or fall back among eligible providers inside that single request, and the default free route may select different eligible models between requests; Emenda itself does not retry.

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
