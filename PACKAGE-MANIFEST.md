# Emenda Frozen Clean-Room Constitution

> **Frozen package manifest, version 2.1.0**

> **Freeze ID: emenda-clean-room-v2.1.0-2026-08-21**

This documentation-only package is the complete governance-hardened constitution for a future, separately authorized Emenda V0.1 implementation objective.

## Canonical state and lineage

Version 2.1.0 is one atomic two-parent documentation freeze:

| Role | Commit | Tree | Meaning |
| --- | --- | --- | --- |
| First parent and behavioral baseline | 5295799c637f89a5db12b2971dee12ead7977270 | e1c9227682c0d925750689b6da62b645cca7b6d1 | Frozen v2.0.3 constitution |
| Second parent and external input | 05eadea4dc05e02b715618c458f7df4bbd9c0b10 | 8b032780267e26026d839ee887e58b668aabf95d | Divergent v2.0.2 child adding the logo proposal |
| Shared v2.0.2 ancestor | 6a4ddc65fa9067f94023f87aebe48840e1b88bc2 | c27229f58dec3a480751711eabea4f2109f9fc02 | Preserved historical freeze |

Version 2.0.1 remains preserved at d70b277998a23663ee6befc77dd6bb0da50ebcca, and version 2.0.0 remains preserved at a1a13607867db8e6eb2ea904f6387ba130f22ce7.

The first parent is the sole source of carried product behavior. The second parent is preserved for lineage and provides external proposal material only. This decision reunifies the previously divergent remote state without silently granting the proposal authority.

## Authorization boundary

[`PROMPT.md`](PROMPT.md) owns objective authorization. This Markdown-only freeze authorizes no product implementation by itself; implementation begins only under the separate human authorization defined there.

## Complete tracked-document classification

The freeze contains exactly these 14 tracked Markdown paths:

| Path | Classification | Subject or role | Mutability after freeze | Integrity table |
| --- | --- | --- | --- | --- |
| PROMPT.md | Constitutional | Entry point and authorization boundary | Immutable | Included |
| AGENTS.md | Constitutional | Agent-agnostic operating discipline | Immutable | Included |
| SPEC.md | Constitutional | Product, safety, trust model, failures, and critical requirements | Immutable | Included |
| docs/ARCHITECTURE.md | Constitutional | Ownership, runtime boundaries, and dependency direction | Immutable | Included |
| docs/IMPLEMENTATION-PLAN.md | Constitutional | Build order and gate placement | Immutable | Included |
| docs/ACCEPTANCE.md | Constitutional | Derived gate criteria and canonical live corpus | Immutable | Included |
| docs/ENGINEERING.md | Constitutional | Construction, toolchain, verification, evidence, and review convergence | Immutable | Included |
| UX.md | Constitutional | Visible interaction and accessibility | Immutable | Included |
| ROADMAP.md | Supplemental specification | Non-authoritative future-horizon orientation | Immutable | Included |
| BRAND.md | Constitutional | Active visual identity | Immutable | Included |
| PACKAGE-MANIFEST.md | Constitutional | Freeze identity, lineage, classification, and integrity | Immutable | Excluded: self-referential |
| README.md | Supplemental specification | Non-authoritative repository orientation | Immutable | Included |
| docs/EVIDENCE.md | Mutable evidence | Empty factual ledger template | Append-only under a future objective | Excluded: mutable |
| docs/LOGO.md | External material | Non-authoritative logo-successor proposal and provenance | Immutable in this freeze | Included |

There are 10 constitutional files, two supplemental specifications, one mutable evidence file, and one external-material file. The integrity table covers the 12 immutable non-self files.

## Authority

| Authority | Subject |
| --- | --- |
| PROMPT.md | Objective authorization and completion conditions |
| SPEC.md | Product behavior, safety, compatibility, failures, trust model, provider prompt/schema/constants, and critical requirement IDs |
| docs/ARCHITECTURE.md | Ownership, runtime boundaries, and dependency direction |
| docs/IMPLEMENTATION-PLAN.md | Future implementation order and gate placement |
| docs/ACCEPTANCE.md | Derived gate criteria and canonical live-provider corpus |
| docs/ENGINEERING.md | Toolchain, construction quality, verification, evidence vocabulary, and review convergence |
| UX.md | Visible interaction and accessibility |
| BRAND.md | Visual identity |
| AGENTS.md | Preflight, objective execution, agent coordination, audit handling, Git discipline, and post-completion stop discipline |
| PACKAGE-MANIFEST.md | Freeze identity, lineage, classification, integrity, and lifecycle |

Supporting, evidence, and external documents introduce no authority over those homes. Each normative rule has one subject owner. The five-term vocabulary in AGENTS.md applies across the package.

## Behavior-preservation statement

Version 2.1.0 adds AI-construction governance, a consolidated trust model, selective traceability, reproducibility rules, and explicit lineage. It carries forward the v2.0.3 constants, state/profile unions, provider input/result schemas, request body, canonical prompt, live corpus, Unicode/context derivation, paired-input and IME provenance, origin and permission lifecycle, supported textarea/exposure predicate, Apply authority and mutation, text-only rendering, failure mapping, and verbatim privacy disclosure without observable product change.

## Integrity checksums

These SHA-256 values cover the exact raw Git-blob bytes staged for the 12 immutable non-self files. Individual checksums remain independent. This manifest and the mutable evidence ledger are excluded.

- AGENTS.md: SHA-256 bafabc1cdb16364d982531807ef158972a1da88bd935ca4f057e79057e92ca2d
- BRAND.md: SHA-256 6f09006145f6c1f687d227438d30bdc4662085595bf9fb98d3fe72e250489612
- PROMPT.md: SHA-256 00ddc493619775a68af704dccf4826d2656a7e1ab0f73e5ac9bdb52fe1b0f0a3
- README.md: SHA-256 42bcc052fdc7720bd7b6539aa1dc49fd6f454d6b4f21c97cf5794475f2af784d
- ROADMAP.md: SHA-256 de17776256c15eac8d33b0de644dca49014d25f06c9c8e45f1b3e6a646c6c5b3
- SPEC.md: SHA-256 c1b667d4426f04bd5b0eb6749e826b96721c778a6a668e0c680b6c8803cbd5d0
- UX.md: SHA-256 5d4e1ae6cefe739059f4995dd715e57ca8546e92e113ec8981512e47ae6d10fa
- docs/ACCEPTANCE.md: SHA-256 ffa8ca494b3da5aa6a95f7d0fc20e4bdb635e48d440d6ab6e1d89b229f8308d8
- docs/ARCHITECTURE.md: SHA-256 ec7382acef39f6d08331f2f2099e0fb4866f93fb67f28fb85bea27d232959716
- docs/ENGINEERING.md: SHA-256 80d6b50ac992c8082b8fc5ffc3bcde366831c53596cbd35488bf65ec6d90352c
- docs/IMPLEMENTATION-PLAN.md: SHA-256 283c955bdd7e833509c3b874688cbcf7c0f5bde230c64ca17ccea0ffdb348779
- docs/LOGO.md: SHA-256 08684eb5dd3a327a4002bc444c1f143dcff7002f4a8f07118c68db23c960ef4f

Working-tree line-ending conversion is not the checksum boundary. Hash the final staged Git blobs.

## Freeze validation

A valid v2.1.0 freeze proves:

1. The tracked tree contains exactly the declared 14 Markdown paths.
2. The constitutional, supplemental, and evidence documents identify v2.1.0; docs/LOGO.md identifies itself as non-authoritative external material.
3. Subject authority is singular and supporting material introduces no contradiction.
4. Local Markdown links resolve, canonical-sequence blocks are byte-identical, and the six gate names and order remain consistent.
5. All 18 active critical requirement IDs are unique in SPEC and map to Acceptance.
6. Review against the v2.0.3 first parent finds no observable product-behavior change.
7. The 12 final staged raw-blob SHA-256 values match this manifest; the manifest and evidence ledger are excluded.
8. The evidence ledger contains no implementation claim.
9. The candidate is Markdown-only, git diff --check passes, and the final staged tree receives one complete consistency, security, architecture, and implementability audit.
10. The atomic commit has the declared first and second parents.
11. origin/docs/v2.1.0-freeze and origin/main resolve to that same commit after push.
12. The tracked worktree is clean and pre-existing ignored state is preserved.

## Freeze lifecycle

During a future implementation objective, the implementation repository's constitutional snapshot remains read-only. The canonical `docs/EVIDENCE.md` in this constitution repository may append facts through a ledger-only commit about already-existing tested implementation commits. A requirement ID remains stable while its semantics remain stable and is never reused after retirement.

A material product, safety, architecture, UX, acceptance, implementation-order, brand, or governance change requires a new versioned documentation freeze and new staged checksums. External material becomes authority only through that explicit decision.
