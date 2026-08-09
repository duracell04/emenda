# Emenda

Emenda is a restrained cross-platform writing assistant that corrects and refines text while preserving the author's Duktus.

> **Emenda corrects the text while preserving the author's Duktus.**

## Current V0.1

V0.1 is Emenda's **personal and developer-validation milestone**.

The current verified workflow is:

1. Launch Emenda and open **Settings**.
2. Enter an OpenRouter API key, keep `openrouter/free` or choose a current model, and select automatic or fixed language handling.
3. Select text in another Windows application and press `Ctrl+Alt+E`.
4. Review validated corrections.
5. Apply the accepted changes back to the original source through one adapter-owned replacement operation.

V0.1 supports Swiss Standard German (`de-CH`), British and American English, French, Georgian, and Russian. Automatic mode defaults German to `de-CH` and English to `en-GB` while preserving clearly American usage.

The next product direction builds from this proven selected-text loop toward passive observation, inline signals, richer per-application behaviour, and Grammarly-like ambient assistance.

## Architecture

```text
Shared Emenda product semantics
├── correction schema
├── inference contract
├── snapshots and revisions
├── language profiles
├── settings concepts
├── typed error meanings
└── UX decision rules

Desktop
└── Rust TextSurfaceAdapter
    ├── Windows implementation
    ├── macOS implementation, when development begins
    └── Linux implementation, when development begins

Browser / ChromeOS
└── TypeScript semantic equivalent
```

The privileged desktop core is safe Rust (`#![forbid(unsafe_code)]`). React and strict TypeScript provide the desktop product UI. The browser extension uses strict TypeScript and implements equivalent text-surface semantics using browser-native concepts.

Native source identity, focus, source revalidation, replacement mechanics, protection checks, and clipboard handling stay inside the active adapter. Shared workflow code handles revisions, correction validation, user decisions, and typed outcomes without interpreting platform-specific identifiers.

OpenRouter provides linguistic intelligence through the `InferenceProvider` boundary.

## Correctness model

- Runtime API keys are stored through the operating-system credential manager on desktop.
- `OPENROUTER_API_KEY` is an optional development fallback when no saved credential exists.
- Selected text is sent only after the current V0.1 button or hotkey invocation.
- OpenRouter output is treated as untrusted data and must pass strict schema, range, overlap, and source-text validation.
- Every request is bound to an immutable revision; late responses cannot replace newer text.
- Source replacement is adapter-owned and fail-closed: the active adapter replaces text only while it can verify that the captured source and expected text remain authoritative.
- Dynamic free-model routing can occasionally return incompatible content; Emenda rejects it as a typed structured-output error without touching source text.

## Platform status

Use these terms consistently:

```text
Architectural target
= intended platform represented by shared product contracts and design decisions

Compiles
= repository builds successfully on that host

Supported platform
= adapter implemented
+ shared platform-agnostic tests pass
+ platform-specific integration tests pass on that OS

Distribution-ready
= supported platform
+ packaging and platform trust requirements satisfied
```

Current status:

- **Windows:** supported and runtime-verified native adapter.
- **macOS:** first-class architectural target; native adapter implementation and macOS integration verification remain future work.
- **Linux:** first-class architectural target; native adapter implementation and Linux integration verification remain future work.
- **Browser / ChromeOS:** first-class architectural target and primary ChromeOS path; extension implementation and browser integration verification remain future work.

Windows reaching support first represents the first verified adapter rather than a Windows-specific product architecture.

## Development

Current Windows development prerequisites:

- Windows 10 or 11 with WebView2
- Node.js and npm
- Rust 1.88 or newer
- MSVC Build Tools with a Windows SDK

```powershell
npm install
npm run tauri dev
```

You can configure the key in Settings. For process-scoped development instead:

```powershell
$env:OPENROUTER_API_KEY = "your-key"
npm run tauri dev
```

MSVC, the Windows SDK, Xcode command-line tools, and Linux system packages are build infrastructure rather than additional Emenda application languages.

## Verification

The local repository health gate is:

```powershell
npm run typecheck
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

Opt-in live Windows tests require a process-scoped key and network access:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml live_openrouter_flow -- --ignored
cargo test --manifest-path src-tauri/Cargo.toml --test windows_desktop_smoke -- --ignored --test-threads=1
```

On 2026-08-09, the complete real text-surface and live OpenRouter correction flow (`I liek this sentence.` → `I like this sentence.`) passed in:

| Application | Version | Result |
| --- | --- | --- |
| Windows Notepad | 11.2606.15.0 x64 | Passed |
| Visual Studio Code | 1.132.0, isolated profile | Passed |

Cross-platform CI should compile and run the strongest applicable shared checks on Windows, macOS, and Linux. A successful build on a host establishes compilation evidence; runtime platform support additionally requires the native adapter and its platform-specific integration tests.

## Release status

The current milestone is personal/developer validation, not public beta distribution.

Public beta readiness is a separate release gate covering:

- signing and platform trust
- installer/package quality
- update delivery
- runtime compatibility breadth
- platform-specific support verification

Packaging remains a deployment concern outside Emenda's shared correction, inference, state, and text-surface architecture.

## Documentation

- [SPEC.md](SPEC.md) — engineering source of truth, architecture, contracts, rationale and implementation order
- [AGENTS.md](AGENTS.md) — coding-agent governance and commit discipline
- [UX.md](UX.md) — interaction principles and UX decision rules
- [BRAND.md](BRAND.md) — visual identity and brand system
