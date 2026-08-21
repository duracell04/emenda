# Emenda Frozen Clean-Room Constitution

> **Frozen package manifest, version 2.0.3**

> **Freeze ID: `emenda-clean-room-v2.0.3-2026-08-21`**

This documentation-only package is the complete constitution for a future, separately authorized Emenda V0.1 implementation objective.

Version 2.0.3 supersedes version 2.0.2, preserved at Git commit `6a4ddc65fa9067f94023f87aebe48840e1b88bc2`. Version 2.0.2 preserves version 2.0.1 at `d70b277998a23663ee6befc77dd6bb0da50ebcca`; version 2.0.1 preserves version 2.0.0 at `a1a13607867db8e6eb2ea904f6387ba130f22ce7`.

## Objective boundary

The v2.0.3 objective ends after these 13 Markdown files are rewritten, verified, hashed, committed as one direct child of v2.0.2, pushed, and the worktree is clean. This package contains no tracked implementation source and does not authorize implementation. Product work requires a separate future objective that identifies its implementation repository and baseline.

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

- `AGENTS.md`: SHA-256 `fc02560ef26797c81a0da20cd96d74a3ce535887350d7ea96b5ba08c2ec9a05a`
- `BRAND.md`: SHA-256 `208a87cee1cce352bd7bbd12a281abeb00db52dc8349e6345d8a9db04b9f3281`
- `PROMPT.md`: SHA-256 `ce0d9154c41a4b3008cb89d146d7c2f7fed9cf72b88583a62a123a7f4b075d58`
- `README.md`: SHA-256 `1e62af0424d8cdff5f3b4c7b3a7aa84e51055505aa83d56bc85fd2163178bd3d`
- `ROADMAP.md`: SHA-256 `c5d1313eece43d0e8402d3c40f1937edcc45df94f7c98673c4dc8d98fc569229`
- `SPEC.md`: SHA-256 `0f06e52651a59d7ffb6c97a80075abadf8d2ed6660a97a2353f7cc098d4efe3c`
- `UX.md`: SHA-256 `2e99618fd71dd491d7cdc2dcdde72096af103a1d212f4c5bdffaee81c3076ca8`
- `docs/ACCEPTANCE.md`: SHA-256 `0304ca3d4508ea11de9072f5a44c197bce72aa312ca914ab2d50cd5f7acd41fa`
- `docs/ARCHITECTURE.md`: SHA-256 `faf7f3d0025bc9c23bf52838c909822c6009fc29472b045cd48cdf4ead1bd4fd`
- `docs/ENGINEERING.md`: SHA-256 `fb619f62010ae67202be536dc741f98a6d8f9b5f76fe866b192936e510525c5d`
- `docs/IMPLEMENTATION-PLAN.md`: SHA-256 `14c946094694702c60f2faf2b5fb89c6754e9f168b47ce008fa9fb91adcad823`

## Freeze validation

A valid v2.0.3 freeze proves:

```text
the tracked tree contains exactly the declared 13 Markdown paths
→ every document identifies v2.0.3, the commit is a direct child of v2.0.2, and every ancestry reference is correct
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
