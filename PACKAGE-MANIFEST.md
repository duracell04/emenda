# Emenda Frozen Clean-Room Constitution

> **Frozen package manifest, version 2.0.2**

> **Freeze ID: `emenda-clean-room-v2.0.2-2026-08-16`**

This documentation-only package is the complete constitution for a future, separately authorized Emenda V0.1 implementation objective.

Version 2.0.2 supersedes version 2.0.1, preserved at Git commit `d70b277998a23663ee6befc77dd6bb0da50ebcca`. Version 2.0.1 preserves version 2.0.0 at `a1a13607867db8e6eb2ea904f6387ba130f22ce7`; version 2.0.0 in turn preserves the earlier v1.0.1 constitution at `d3192b74a7dd78c0029bfee44fd087876d8ce774`.

## Objective boundary

The v2.0.2 objective ends after these 13 Markdown files are rewritten, verified, hashed, committed as one direct child of v2.0.1, pushed, and the worktree is clean. This package contains no tracked implementation source and does not authorize implementation. Product work requires a separate future objective that identifies its implementation repository and baseline.

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

- `AGENTS.md`: SHA-256 `a4ee3bc0041ce66fb5e7ed0ea3f72841fd94027aa96c037a4f679c504aeb0a15`
- `BRAND.md`: SHA-256 `195d1630b6fd9568bb4b93bd1c3419cbf3a0050ba8eca353ee075340f718c149`
- `PROMPT.md`: SHA-256 `1fa622eabff6f59927fc0638c51cab921f43a1550c32571d258d33464bd968f4`
- `README.md`: SHA-256 `7e4bf282c9f6e66b08e686588ffc813524c33f96fa005194d855754ac5662694`
- `ROADMAP.md`: SHA-256 `cf68fbc9405723f85691f1d32885feb47d2bfc3c3060c9a7ad996a3869bb8423`
- `SPEC.md`: SHA-256 `a65f543e34b508067e34856dc64fb83633d44e346ddbb30ff2f2e86f85d2da8e`
- `UX.md`: SHA-256 `41e200f4327fe287365339f58b9ba4081ff3549bdec4910f798f24b9e03fe20d`
- `docs/ACCEPTANCE.md`: SHA-256 `5c6386c6d6ce94898e18f630296740a99f44bcb79a0a7c80f46ad7a10ae988c5`
- `docs/ARCHITECTURE.md`: SHA-256 `fb826154a135d5ee244016230f433b2b63b5f2c31ba2ccf5fd4ccfbf910a852a`
- `docs/ENGINEERING.md`: SHA-256 `7a08c6deeccc1d23aad83b9166e8a850e4f5d874bf73efcdd5a2bcb179f24004`
- `docs/IMPLEMENTATION-PLAN.md`: SHA-256 `fec6613f8c94fcfe40d473d951fa4e5fdec4b0b32d7fb7917566cf8da917707a`

## Freeze validation

A valid v2.0.2 freeze proves:

```text
the tracked tree contains exactly the declared 13 Markdown paths
→ every document identifies v2.0.2, the commit is a direct child of v2.0.1, and every ancestry reference is correct
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
