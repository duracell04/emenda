# Emenda Frozen Clean-Room Constitution

> **Frozen package manifest, version 2.0.1**

> **Freeze ID: `emenda-clean-room-v2.0.1-2026-08-14`**

This documentation-only package is the complete constitution for a future, separately authorized Emenda V0.1 implementation objective.

Version 2.0.1 supersedes version 2.0.0, preserved at Git commit `a1a13607867db8e6eb2ea904f6387ba130f22ce7`. Version 2.0.0 in turn preserves the earlier v1.0.1 constitution at `d3192b74a7dd78c0029bfee44fd087876d8ce774`.

## Objective boundary

The v2.0.1 objective ends after these 13 Markdown files are rewritten, verified, hashed, committed, pushed, and the worktree is clean. This package contains no tracked implementation source and does not authorize implementation. Product work requires a separate future objective.

## Contents

- `PROMPT.md`
- `AGENTS.md`
- `SPEC.md`
- `docs/ARCHITECTURE.md`
- `docs/IMPLEMENTATION-PLAN.md`
- `docs/ACCEPTANCE.md`
- `docs/ENGINEERING.md`
- `UX.md`
- `ROADMAP.md`
- `BRAND.md`
- `README.md`
- `docs/EVIDENCE.md` — mutable factual ledger template
- `PACKAGE-MANIFEST.md` — freeze identity and integrity data

The 11 immutable constitutional documents are all listed files except this manifest and `docs/EVIDENCE.md`.

## Subject authority

```text
SPEC.md                      product behavior, safety, compatibility, failures
docs/ARCHITECTURE.md         ownership, boundaries, dependency direction
docs/IMPLEMENTATION-PLAN.md  future implementation order and gate placement
```

`PROMPT.md` and `AGENTS.md` define the objective and operating constraints. Acceptance and Engineering verify the authorities. UX, Roadmap, Brand, and README specialize or summarize their assigned subject without overriding them. Evidence records facts only.

## Future scope

The future V0.1 product is one strict-TypeScript core and one Chromium Manifest V3 extension requiring Chrome 140 or newer. It has seven numbered implementation increments and six separate gates. The Documentation Gate is the prerequisite, not an implementation increment.

Native runtimes, operating-system accessibility APIs, native credential stores, packaging, signing, store publication, release automation, commercial infrastructure, and placeholders for deferred runtimes remain outside V0.1.

## Integrity checksums

The following SHA-256 values cover the exact raw Git-blob bytes staged for the 11 immutable documents. They are individual checksums; no aggregate digest replaces them. This manifest and the mutable evidence ledger are excluded.

- `AGENTS.md`: SHA-256 `8bfefd810f30dbdec04fcc1cd21c54c4108a4dada2e723633ea7109b320b66fb`
- `BRAND.md`: SHA-256 `59281de06330e552c7d608b56d2e742f18c9a1aa5090450a5c7800d452903c6c`
- `PROMPT.md`: SHA-256 `b348f6896bba97c7cea52a25d6c91f961052aac573c07e4ff2c4cbbd00d8f1ef`
- `README.md`: SHA-256 `564f369bf7a6ad12435a5b768fae7095b8e23a9acfd16bd6dd5d7a102f94ff59`
- `ROADMAP.md`: SHA-256 `97df1f20c2f6c8007c913359e23e8f19e8e9de8f5aa95300e053a6495f14bc1c`
- `SPEC.md`: SHA-256 `dd1a3d4c5870194cbac78a7c0cb91239e3f00a8b7e842b664f03c79a34aa4acd`
- `UX.md`: SHA-256 `ee389558db7c70fd25a474ebc42fad5e29d7c476c532e27353f9d9ebd30672ad`
- `docs/ACCEPTANCE.md`: SHA-256 `a9dbab367dde51c4714ba51dd6e80a0cc33aa066c45a37506b47c02c4fddc42a`
- `docs/ARCHITECTURE.md`: SHA-256 `a05e16ebf0d9d24a2146193401c33312c404b6e7f2d9d662548e72f176aa4d14`
- `docs/ENGINEERING.md`: SHA-256 `91f9aff96db569f7042659f56b665ff003740b783c2f207ab3519fb6b9d032c8`
- `docs/IMPLEMENTATION-PLAN.md`: SHA-256 `e2abb8e2145178b7d1d0654f69a8457c5e994b63568777b8f2789969e21f686d`

## Freeze validation

A valid v2.0.1 freeze proves:

```text
the tracked tree contains exactly the declared 13 Markdown paths
→ every document identifies v2.0.1 and every ancestry reference is correct
→ local Markdown links resolve
→ every canonical-sequence occurrence is byte-identical
→ the six gate names and ownership are consistent
→ the 11 staged raw-blob SHA-256 values match this manifest
→ this manifest and docs/EVIDENCE.md are excluded from those hashes
→ the evidence ledger contains no implementation evidence
→ the diff is documentation-only and git diff --check passes
→ the documentation commit is pushed and remote identity is verified
→ the worktree is clean
```

Working-tree line-ending conversions are not the checksum boundary. Hash the staged Git blobs.

## Freeze lifecycle

The immutable documents remain frozen during a future implementation objective. The evidence ledger may append factual entries about already-existing tested implementation commits. Any material product, safety, architecture, UX, acceptance, implementation-order, brand, or governance change requires a newly versioned constitution and new staged checksums.

The constitution resolves all product, safety, architecture, and acceptance decisions. The implementation agent retains ordinary discretion over local naming and code organization within the locked boundaries.
