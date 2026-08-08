# Emenda

A minimal local desktop writing assistant that uses an AI API to correct and refine text while preserving the author's Duktus.

## What it does

Emenda follows one simple correction loop:

```text
select text → hotkey → AI correction → review → apply back to source app
```

1. Select text in another desktop application.
2. Trigger Emenda with the global hotkey.
3. Emenda sends an immutable text snapshot to the configured AI model.
4. Review concise, structured corrections.
5. Apply selected corrections directly back to the source application.

## Current V0.1 scope

V0.1 is focused on one reliable desktop correction loop:

- Tauri desktop application
- selected-text correction through a global hotkey
- OpenRouter inference
- `openrouter/free` as the default model
- current OpenRouter model selection
- structured, reviewable corrections
- revision-aware text snapshots
- language profiles for:
  - `de-CH` — Swiss Standard German
  - `en-GB` — British English and default English
  - `en-US` — American English
  - `fr-FR` — French
  - `ka-GE` — Georgian
  - `ru-RU` — Russian

The product principle is simple:

> **Emenda corrects the text while preserving the author's Duktus.**

## Architecture / stack

```text
React + strict TypeScript
         ↕
     Tauri 2
         ↕
        Rust
         ↕
     OpenRouter
```

Rust handles local and privileged operations such as text capture, state, secure credentials and text replacement. OpenRouter handles linguistic intelligence. React and strict TypeScript provide the product interface.

Emenda remains one local application with a small privileged core and one external inference API.

## Setup

The V0.1 scaffold uses the following development flow:

```bash
git clone https://github.com/duracell04/emenda.git
cd emenda
npm install
export OPENROUTER_API_KEY="your-key"
npm run tauri dev
```

The runtime application will store the user's OpenRouter credential through secure local credential storage.

## Development checks

A healthy repository should pass:

```bash
npm run typecheck
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

## Status / next milestone

The repository is initialized and ready for the V0.1 implementation. The immediate milestone is the complete selected-text correction loop in one simple native text editor and one additional desktop application: capture → OpenRouter → review → apply back to source.
