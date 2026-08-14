# Emenda Frozen Clean-Room Build Context

> **Freeze ID: `emenda-clean-room-v1.0.1-2026-08-10`**

This documentation-only package is the canonical constitution for rebuilding Emenda from scratch with an AI coding agent.

A frozen package is changed by creating a new version. Version 1.0.1 records the consistency correction to the supplied 1.0.0 baseline; these files are not silently reinterpreted in place.

## Package objective

```text
platform-neutral product semantics
→ complete deterministic product through mocks
→ OpenRouter provider conformance
→ presentation conformance
→ architecture conformance gate
→ replaceable current-host binding
→ V0.1 runtime conformance
```

## Hard invariant

> **The current host operating system is a runtime verification environment, not an architectural input. Shared types, state, interfaces, tests, and presentation behavior contain no operating-system mechanism.**

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

Every file in the clean-room package is Markdown. The package contains no application source code, dependency manifest, executable configuration, binary asset, credential, or generated implementation artifact. All files except the explicitly mutable evidence ledger form the frozen constitution.

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

`docs/EVIDENCE.md` is initialized only after this reading order and baseline verification are complete.

## Source-of-truth hierarchy

```text
PROMPT.md
→ autonomous objective and hard invariants

AGENTS.md
→ persistent coding-agent governance

SPEC.md
→ product behavior and semantic contracts

docs/ARCHITECTURE.md
→ dependency direction and ownership

ROADMAP.md
→ product milestone sequence

docs/IMPLEMENTATION-PLAN.md
→ exact increment-by-increment execution order

docs/ACCEPTANCE.md
→ evidence required to pass each gate

docs/ENGINEERING.md
→ AI-native engineering quality system

UX.md
→ writer interaction rules

BRAND.md
→ visual identity and voice

README.md
→ concise orientation

docs/EVIDENCE.md
→ mutable factual implementation ledger; never constitutional authority
```

## Canonical product model

```text
ObservedChange
→ reserve RevisionId
→ debounce
→ ContextRequest
→ TextContext
→ seal immutable Revision
→ InferenceProvider
→ corrections: [] | [Correction]
→ deterministic validation
├─ [] → Clean
└─ [Correction] → Suggestion
   → Apply or Dismiss
   → TextSurface.replace_if_current(...) or no edit
```

## Canonical implementation order

```text
documentation baseline + Documentation Gate
→ semantic domain
→ TextSurface port
→ MockTextSurface
→ InferenceProvider + MockInferenceProvider
→ controller, debounce, context, and revision
→ validator + presentation state
→ complete mock product + Mock Product Gate
→ OpenRouterProvider + Provider Gate
→ Tauri UI + Presentation Gate
→ Architecture Gate
→ current-host leaf + Current-Host Binding Gate
→ two-application runtime + V0.1 Conformance Gate
```

## Freeze lifecycle

These checksums establish the immutable constitution seed. During implementation, the builder initializes the supplied `docs/EVIDENCE.md` with the starting commit and appends factual status and gate evidence; that mutable ledger is not constitutional authority and is excluded from the freeze checksums.

Application source and mechanically justified project files enter only after the Documentation Gate passes. Their later presence does not alter the frozen seed. A change to product behavior, architecture, UX, brand, acceptance, execution, or agent governance requires a newly versioned package and updated checksums.

## Integrity checksums

The checksums below cover every immutable constitutional document except this manifest. They intentionally exclude the mutable `docs/EVIDENCE.md` ledger.

- `AGENTS.md`: SHA-256 `2f61a1ff531a01a95c95bf754a4d638bd129e21ae0d1e7efd1907a0548a68e99`
- `BRAND.md`: SHA-256 `ea4472665ec8844a536b34e183d988850158e0652c260df1505d8a94a84a1abe`
- `PROMPT.md`: SHA-256 `e81d8cb1766bccf80f572c3ee85a962c69d9491404f14d3b29ba185dd0c0fdc3`
- `README.md`: SHA-256 `0087e1a99bac38203b77fc27b923bf2250277d26b57fb8c5769bc580cead9881`
- `ROADMAP.md`: SHA-256 `e484208512fc869ed434339ef7867a49da2ad6db4fd68b0920e0be51323eefa4`
- `SPEC.md`: SHA-256 `ddff82563f60e3a6033b279df1466f5180f8af403ffc0316ed41107629647012`
- `UX.md`: SHA-256 `b57756d66793b5806e18fe950a9c760c5dca36b0f58bccc0b5fc6528af0af558`
- `docs/ACCEPTANCE.md`: SHA-256 `8a395955b7516dfc29a45daf9d483ab87a7e53c663a44fbdb81364853c87716c`
- `docs/ARCHITECTURE.md`: SHA-256 `59a99761b69ef1f745890511a71e6fb0633f149c9d36ba44b5a1ab4b17fe9389`
- `docs/ENGINEERING.md`: SHA-256 `b1464ee07a4c7692f3d1fb26d7c93a452c0bb9ab20033cdeb40298ded1d80b60`
- `docs/IMPLEMENTATION-PLAN.md`: SHA-256 `3ae768d5e37a27b9f1f07fc0ed40d38eb2d892820c97c613edabf41babf55c1e`

## Freeze validation

A valid documentation-only distribution of this package satisfies:

```text
all listed Markdown files are present
all recorded checksums match
no non-Markdown file is present
all cross-references resolve inside the package
the evidence ledger identifies itself as mutable and non-constitutional
```
