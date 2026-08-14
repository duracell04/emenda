# Emenda

> **Frozen clean-room constitution, version 2.0.0**

> **Preserve your Duktus**

Emenda is a quiet personal writing assistant. V0.1 is one Chromium Manifest V3 extension backed by an OS-agnostic strict-TypeScript core. It observes eligible browser text, asks a user-configured OpenRouter model for zero or one structured correction, validates the response locally, and lets the writer apply the exact change explicitly.

## V0.1 product model

```text
eligible committed input
→ reserve current revision immediately
→ 600 ms trailing-edge debounce
→ capture a DOM-free snapshot
→ select sentence focus and bounded context
→ fixed OpenRouter structured-output request
→ strict local validation
├─ zero corrections → Idle
└─ one correction → fixed overlay
   → Apply or Dismiss
   → verified one-step undo-aware edit or no mutation
```

Emenda supports explicitly enabled top-level HTTP(S) pages with a visible, focused, writable light-DOM `<textarea>` or conventional `contenteditable` surface that maps losslessly. Unsupported or ambiguous surfaces fail closed.

## Architecture

```text
core/                     strict TypeScript; no browser or runtime types
extension/content/        controller, BrowserTextSurface, overlay
extension/worker/         permissions, trusted settings, cancellation, fetch
extension/options/        API-key and model configuration
tests/                    deterministic and persistent-Chromium evidence
scripts/build-extension.mjs
```

The product uses one npm package. Zod is the only direct runtime dependency. Plain TypeScript and DOM APIs implement the overlay and options page.

## Privacy and permissions

- Toolbar activation requests optional permission for the exact current origin.
- One dynamic content-script registration covers enabled origins.
- There is no static all-sites content script and no `<all_urls>` grant.
- The API key and model stay in `chrome.storage.local`, restricted to trusted extension contexts.
- Content scripts receive only `hasApiKey`; raw DOM data and source identity never enter worker messages.
- Browser-profile storage is disclosed as local browser storage, not an operating-system secret vault.
- There is no telemetry, analytics, or persistent text cache.

## Canonical implementation sequence

```text
Documentation baseline + Documentation Gate
→ strict-TypeScript domain and schemas
→ TextSurface + MockTextSurface
→ InferenceProvider + MockInferenceProvider
→ controller, scheduler, context, and revision
→ validator + presentation state
→ complete mock product + Mock Product Gate
→ Architecture Gate
→ BrowserTextSurface
→ MV3 worker, options, and overlay
→ OpenRouterProvider + Provider Gate
→ textarea runtime
→ conventional contenteditable runtime
→ Browser Integration + V0.1 Conformance Gate
→ stop
```

## Active gates

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

## Documentation map

1. [`PROMPT.md`](PROMPT.md): autonomous V0.1 objective
2. [`AGENTS.md`](AGENTS.md): agent governance
3. [`SPEC.md`](SPEC.md): product and semantic contracts
4. [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md): dependency direction and extension composition
5. [`ROADMAP.md`](ROADMAP.md): milestone sequence and later horizons
6. [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md): ordered build increments
7. [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md): gate evidence and conformance criteria
8. [`docs/ENGINEERING.md`](docs/ENGINEERING.md): engineering and verification standards
9. [`UX.md`](UX.md): writer interaction and accessibility rules
10. [`BRAND.md`](BRAND.md): visual identity and extension assets
11. [`docs/EVIDENCE.md`](docs/EVIDENCE.md): mutable implementation ledger template
12. [`PACKAGE-MANIFEST.md`](PACKAGE-MANIFEST.md): freeze identity, supersession, inventory, and checksums

Together these 13 Markdown files form freeze `emenda-clean-room-v2.0.0-2026-08-14`. Version 2.0.0 supersedes version 1.0.1 while preserving the earlier constitution at Git commit `d3192b7`.

## Explicitly deferred

Native hosts, Tauri, Rust, operating-system accessibility APIs, packaging, signing, Chrome Web Store publication, release automation, and native placeholders belong to later separately versioned objectives. Cross-OS runtime support is claimed only when independently evidenced.
