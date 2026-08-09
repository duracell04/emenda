# Emenda

Emenda is a restrained cross-platform writing assistant that corrects and refines text while preserving the author's Duktus. The current V0.1 native adapter is verified on Windows.

> **Emenda corrects the text while preserving the author's Duktus.**

## V0.1 workflow

1. Launch Emenda and open **Settings**.
2. Enter an OpenRouter API key, keep `openrouter/free` or choose a current model, and select automatic or fixed language handling.
3. Select text in another application and press `Ctrl+Alt+E`.
4. Review the validated corrections. Card actions stage accepted changes; Emenda performs one final replacement so the source application's native undo remains useful.

Apply or Dismiss may hide Emenda while its hotkey stays active. Launching Emenda again restores the existing window; closing it with X exits the application.

V0.1 supports Swiss Standard German (`de-CH`), British and American English, French, Georgian, and Russian. Automatic mode defaults German to `de-CH` and English to `en-GB` while preserving clearly American usage.

## Security and correctness

- Runtime API keys are stored through the operating system credential manager. They are never written to `settings.json` or returned to React.
- `OPENROUTER_API_KEY` is an optional development fallback when no saved credential exists.
- Selected text is sent only after an explicit button or hotkey invocation.
- OpenRouter output is treated as untrusted data and must pass strict schema, range, overlap, and source-text validation.
- Every request is bound to an immutable revision; late responses cannot replace newer text.
- Before replacement, Emenda refocuses the exact source window and re-copies the selection. A changed window, caret, or passage fails closed.
- Clipboard contents are preserved and restored around capture and one-paste replacement.

## Architecture

```text
Shared Emenda core
├── React 19 + strict TypeScript + Zod
├── Rust snapshots, validation, credentials and inference
├── OpenRouter
└── Text-surface adapters
    ├── Windows
    ├── macOS
    ├── Linux
    └── Browser / ChromeOS
```

The privileged desktop core is safe Rust (`#![forbid(unsafe_code)]`). Platform-specific text access stays behind adapter boundaries, while correction workflow, inference contracts, snapshots, language profiles, settings semantics, and UX behaviour remain shared.

Windows is the current development and runtime-verification environment. A platform is considered supported only after its adapter implements the common contract, the shared test suite passes, and platform-specific integration tests pass on that operating system.

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

Production installers are generated under `src-tauri/target/release/bundle/` by `npm run tauri build`.

For machine-local Authenticode test signing, follow the
[Windows local test-signing guide](docs/windows-test-signing.md). Self-signed
artifacts prove local integrity but do not establish public publisher trust.

## Verification

The repository health gate is:

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

The desktop test temporarily opens and focuses test-owned editor windows. On 2026-08-09, the complete real text-surface and live OpenRouter correction flow (`I liek this sentence.` → `I like this sentence.`) passed in:

| Application | Version | Result |
| --- | --- | --- |
| Windows Notepad | 11.2606.15.0 x64 | Passed |
| Visual Studio Code | 1.132.0, isolated profile | Passed |

The optimized Tauri build also produced the Windows executable, MSI, and NSIS installer. Dynamic free-model routing can occasionally return incompatible content; Emenda rejects it as a typed structured-output error without touching source text.

## Current platform status

- **Windows:** first implemented and runtime-verified native adapter.
- **macOS:** first-class target; adapter support is established only after native implementation and macOS integration tests pass.
- **Linux:** first-class target; adapter support is established only after native implementation and Linux integration tests pass.
- **Browser / ChromeOS:** first-class Emenda surface and primary ChromeOS path through the browser extension.

The current V0.1 native workflow is selected-text correction. Broader passive detection, inline suggestions, personal vocabulary, per-app behaviour, and richer Grammarly-like interaction build on the same shared contracts.

See [SPEC.md](SPEC.md) for the engineering source of truth, [AGENTS.md](AGENTS.md) for repository governance, [UX.md](UX.md) for interaction principles, and [BRAND.md](BRAND.md) for the visual identity system.
