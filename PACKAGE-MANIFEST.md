# Emenda Frozen Clean-Room Build Context

> **Frozen package manifest, version 2.0.0**

> **Freeze ID: `emenda-clean-room-v2.0.0-2026-08-14`**

This documentation-only package is the canonical constitution for Emenda's browser-only TypeScript V0.1.

Version 2.0.0 supersedes version 1.0.1. The historical v1.0.1 constitution remains preserved at Git commit `d3192b7`; its manifest hashes are historical and are not repaired in place. Archived implementation code remains outside `main`.

## Package objective

```text
strict-TypeScript browser-independent product semantics
→ complete deterministic product through mocks
→ architecture isolation
→ Chromium MV3 leaf integration
→ OpenRouter provider conformance
→ textarea and contenteditable runtime conformance
→ unpacked V0.1 completion
```

## Contents

- `PROMPT.md`
- `AGENTS.md`
- `SPEC.md`
- `docs/ARCHITECTURE.md`
- `ROADMAP.md`
- `docs/IMPLEMENTATION-PLAN.md`
- `docs/ACCEPTANCE.md`
- `docs/ENGINEERING.md`
- `docs/EVIDENCE.md` — mutable implementation ledger, not constitutional authority
- `UX.md`
- `BRAND.md`
- `README.md`
- `PACKAGE-MANIFEST.md`

All 13 files are Markdown. This freeze contains no application source, dependency manifest, executable configuration, binary asset, credential, generated implementation artifact, or implementation evidence.

The 11 immutable constitutional documents are every listed file except this manifest and `docs/EVIDENCE.md`.

## Reading order

1. `PROMPT.md`
2. `AGENTS.md`
3. `SPEC.md`
4. `docs/ARCHITECTURE.md`
5. `ROADMAP.md`
6. `docs/IMPLEMENTATION-PLAN.md`
7. `docs/ACCEPTANCE.md`
8. `docs/ENGINEERING.md`
9. `UX.md`
10. `BRAND.md`
11. `README.md`

`docs/EVIDENCE.md` is initialized only after this reading order and the Documentation Gate are complete.

## Source-of-truth hierarchy

```text
PROMPT.md                    autonomous objective and hard limits
AGENTS.md                    persistent agent governance
SPEC.md                      product and semantic contracts
docs/ARCHITECTURE.md         dependency direction and runtime ownership
ROADMAP.md                   milestone order
docs/IMPLEMENTATION-PLAN.md  exact implementation increments
docs/ACCEPTANCE.md           required gate evidence
docs/ENGINEERING.md          engineering quality system
UX.md                        writer interaction and accessibility
BRAND.md                     visual identity and extension assets
README.md                    concise orientation
docs/EVIDENCE.md             mutable factual ledger only
```

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

Presentation and accessibility evidence belongs to Browser Integration.

## Active runtime and deferral

The only active V0.1 runtime is an unpacked Chromium Manifest V3 extension backed by an OS-agnostic strict-TypeScript core.

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, packaging, signing, Chrome Web Store publication, release automation, native placeholders, and cross-OS runtime claims are deferred to separately versioned objectives.

## Freeze lifecycle

During implementation, the builder initializes and appends factual status to `docs/EVIDENCE.md`. The ledger may change without changing the constitution.

A material change to product behavior, architecture, UX, brand, acceptance, implementation order, or agent governance requires a newly versioned package and newly staged checksums.

## Integrity checksums

The SHA-256 values below cover the exact Git-blob bytes staged for the 11 immutable documents. They exclude this manifest and the mutable evidence ledger.

- `AGENTS.md`: SHA-256 `6c438956abee423718e7ab349e18f0a8739629e5fb301a8f984ebe4b4a98062c`
- `BRAND.md`: SHA-256 `546bd7848e16c8a7f58e45f867d71987a6dceeb081647a2ac71a448e5389525a`
- `PROMPT.md`: SHA-256 `17ac5454be674dc6379123adf929f20d8056653b2ddedc436266a300b632c991`
- `README.md`: SHA-256 `850c5cfd439b0d1937821bdcb2a1ba8ed78f525701f8e174093c81bd16f0329d`
- `ROADMAP.md`: SHA-256 `9bbaf656c75718b24c342216094f53bda9fcb914ba89fe8c3b47f597a16b2a2b`
- `SPEC.md`: SHA-256 `1e7426d2714a30d246895d3869ea1c3589c0e78bbb3f1f80d7ce7a2f26d04bc3`
- `UX.md`: SHA-256 `2785e05fcc2d8fb2f13b66897fea76ca357193d5168a2f82977a3c43440faa76`
- `docs/ACCEPTANCE.md`: SHA-256 `7f7599551abe219cf03568aaa6cd29ef4481d2aa0f9100eff85f1c2ef61f4079`
- `docs/ARCHITECTURE.md`: SHA-256 `d10e548c2a5e7e9f1ba668d05f697fe2792845507d9b01b9532a7857aa2d0693`
- `docs/ENGINEERING.md`: SHA-256 `e9961b9b5b837efdf838c61d03bdf53c85d820b32a0f01ad43c6d3c96dad16f3`
- `docs/IMPLEMENTATION-PLAN.md`: SHA-256 `3a3843e4612d952d3a475dc484e707b59d64d4d661ba04448f5708127402c200`

## Freeze validation

A valid v2.0.0 documentation freeze satisfies:

```text
exactly 13 Markdown files are present
→ every file identifies version 2.0.0
→ local links resolve
→ canonical sequences are identical
→ native references are deferred only
→ 11 staged Git-blob hashes match this manifest
→ this manifest and the evidence ledger are excluded from checksums
→ the evidence ledger contains no implementation claim
→ git diff --check passes
```
