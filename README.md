# Emenda

Emenda is a restrained Windows desktop writing assistant. Select text in another application, press a global shortcut, review structured OpenRouter corrections, and apply only the changes you want without losing your Duktus.

> **Emenda corrects the text while preserving the author's Duktus.**

## V0.1 workflow

1. Launch Emenda and open **Settings**.
2. Enter an OpenRouter API key, keep `openrouter/free` or choose a current model, and select automatic or fixed language handling.
3. Select text in another application and press `Ctrl+Alt+E`.
4. Review the validated corrections. Card actions stage accepted changes; Emenda performs one final replacement so the source application's native undo remains useful.

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
React 19 + strict TypeScript + Zod
                  ↕ Tauri 2 commands/events
Rust snapshots, validation, credentials, text transport
                  ↕ HTTPS
               OpenRouter
```

The privileged core is safe Rust (`#![forbid(unsafe_code)]`). The V0.1 native text adapter targets Windows; the provider and text-surface traits keep later platforms and inference backends replaceable.

## Development

Prerequisites:

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

## Verification

The repository health gate is:

```powershell
npm run typecheck
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

Opt-in live tests require a process-scoped key and network access:

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

## V0.1 limits

- Native selected-text transport is Windows-only.
- Elevated, protected, or inaccessible surfaces are rejected rather than overwritten.
- The original selection and source window must remain unchanged while suggestions are reviewed.
- Broader application compatibility, history, dictionaries, tray behavior, and local inference remain later work.

See [SPEC.md](SPEC.md) for the engineering source of truth and [AGENTS.md](AGENTS.md) for repository governance.
