# Emenda

> **Frozen clean-room constitution, version 2.0.1**

> **Preserve your Duktus**

Emenda V0.1 is specified as a quiet personal writing assistant for Chromium. It observes committed changes on explicitly enabled sites, waits for typing to settle, sends one bounded context to a user-configured OpenRouter model, validates the structured result locally, and presents at most one exact correction for the writer to apply or dismiss.

This branch currently contains the 13-file Markdown constitution only. Version 2.0.1 supersedes version 2.0.0, preserved at commit `a1a13607867db8e6eb2ea904f6387ba130f22ce7`. Product implementation requires a separate future objective.

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

- The writer supplies an OpenRouter API key, one concrete structured-output model, and a language profile.
- `auto` is the default profile; fixed profiles are `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, and `ru-RU`.
- The toolbar requests optional permission for the exact active HTTP(S) origin.
- Revoking an origin makes live content scripts inert and removes its registration and permission.
- The worker owns trusted settings. Options communicates with the worker; content scripts never receive the API key or model.

## Privacy

> Emenda sends only the current bounded text context, up to 1,200 Unicode scalars, to OpenRouter and the provider serving the configured model. It does not send the page URL, full document, source identity or DOM structure. Processing remains subject to OpenRouter’s and the model provider’s policies. The API key is stored in the browser profile, not in an operating-system secret vault.

Emenda specifies no telemetry, analytics, correction history, persistent text cache, request logging, retry, streaming, provider fallback, or model substitution.

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
