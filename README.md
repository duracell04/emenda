# Emenda

> **Preserve your Duktus**

Emenda is a quiet local writing assistant that observes editable text, delegates linguistic judgment to OpenRouter, validates every proposal locally, and lets the writer apply each change explicitly.

This package is the frozen clean-room constitution for rebuilding Emenda from an empty implementation context.

## Product model

```text
editable text changes
→ short debounce
→ smallest useful context
→ immutable revision
→ OpenRouter
→ structured corrections
→ deterministic validation
→ compact suggestion
→ Apply or Dismiss
→ safe current-source replacement
→ continue writing
```

Emenda keeps local intelligence deliberately small:

```text
Did useful text change?
Has typing settled?
What is the smallest useful context?
Is this revision still current?
Is the model response valid?
Can the exact change still be applied safely?
```

OpenRouter handles linguistic judgment.

## Hard architecture rule

```text
                         EMENDA

                  OS-AGNOSTIC PRODUCT
                            │
                    semantic TextSurface
                            │
             ┌──────────────┼──────────────┐
             ▼              ▼              ▼
       native binding  native binding  browser binding
```

Shared code contains zero knowledge of:

```text
Windows
macOS
Linux
HWND
UI Automation
AXUIElement
AT-SPI
clipboard shortcuts
keyboard simulation
DOM nodes
```

Those mechanisms belong to replaceable leaf bindings.

The current host operating system is a runtime verification environment, not an architectural input.

## Mock-first build model

The complete product is built and accepted first against:

```text
MockTextSurface
+
MockInferenceProvider
```

Only after the mock product loop and architecture gate pass does the project add a binding for the available runtime host.

In the owner's present environment, runtime evidence may be collected through a Windows binding. That environmental fact may appear only in binding-specific code, tests, and evidence.

## Technology

- Tauri 2
- safe Rust
- strict TypeScript
- HTML and CSS
- Serde
- Zod
- OpenRouter

The stack is selected to make the compiler, type system, runtime schemas, and capability boundaries part of the correctness system.

## V0.1 outcome

V0.1 is complete when:

```text
writer types ordinary editable text
→ Emenda observes the change automatically
→ one current request is produced after debounce
→ a valid correction is presented
→ Apply changes the exact intended source safely
→ Dismiss preserves the source
→ stale work cannot affect newer text
```

The shared product must already satisfy that loop deterministically through mocks before native runtime verification begins.

## Documentation map

1. [`PROMPT.md`](PROMPT.md): autonomous build objective
2. [`AGENTS.md`](AGENTS.md): agent execution governance
3. [`SPEC.md`](SPEC.md): product and engineering source of truth
4. [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md): dependency direction and semantic contracts
5. [`ROADMAP.md`](ROADMAP.md): product and platform milestone sequence
6. [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md): commit-by-commit build plan
7. [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md): evidence required for every gate
8. [`docs/ENGINEERING.md`](docs/ENGINEERING.md): AI-native engineering standard
9. [`UX.md`](UX.md): interaction rules and product north star
10. [`BRAND.md`](BRAND.md): visual identity and brand system
11. [`PACKAGE-MANIFEST.md`](PACKAGE-MANIFEST.md): freeze identity, contents, and checksums

Together these Markdown documents are sufficient to reconstruct the intended product without access to an earlier implementation.
