# Emenda V0.1

> **Frozen clean-room constitution, version 2.2.1**

This repository is the Markdown constitution for Emenda V0.1. It defines the product, architecture, implementation sequence, verification, interaction, brand, and agent operating contract. Product implementation belongs in a separately authorized implementation repository, and implementation mutation begins under the product objective defined below.

## Entry point

Every agent begins with this file and [AGENTS.md](AGENTS.md), completes the proportional preflight defined there, states the active gate, and then loads the authoritative documents relevant to the current decision. This progressive-disclosure path is the canonical reading method.

This file owns objective authorization and completion conditions. A human objective owner authorizes every product, architecture, safety, UX, acceptance, implementation-order, brand, or governance change. Agents surface evidence and draft proposals for adoption by that owner.

A documentation objective revises this repository through one new versioned atomic freeze. A product objective identifies a separate implementation repository, exact baseline, branch, frozen constitution commit and tree, and completion target before implementation mutation begins.

## Governing construction objective

> **Build the smallest sufficient implementation through affirmative, precise, auditable instructions; deterministic verification; low complexity and maintenance burden; and explicit completion criteria.**

The constitution is agent-agnostic and architecture-specific. Codex, Claude, Copilot, Gemini, and future coding agents receive the same authority hierarchy and observable contract. Agent-specific compatibility files remain thin integration layers that point here.

## Subject authority

- [PROMPT.md](PROMPT.md) owns objective authorization and completion conditions.
- [SPEC.md](SPEC.md) owns product behavior, safety, compatibility, failures, the trust model, the canonical provider prompt and schemas, and critical requirement IDs.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) owns component responsibility, runtime boundaries, and dependency direction.
- [docs/IMPLEMENTATION-PLAN.md](docs/IMPLEMENTATION-PLAN.md) owns build order and gate placement.
- [docs/ACCEPTANCE.md](docs/ACCEPTANCE.md) derives gate criteria and owns the canonical live-provider corpus.
- [docs/ENGINEERING.md](docs/ENGINEERING.md) owns implementation quality, toolchain, verification, and evidence vocabulary.
- [UX.md](UX.md) owns visible interaction and accessibility; [BRAND.md](BRAND.md) owns visual identity.
- [AGENTS.md](AGENTS.md) owns preflight, objective execution, agent coordination, audit handling, Git discipline, and post-completion stop discipline.

The subject homes above govern decisions. Treat supporting and external documents as classified context or evidence. [PACKAGE-MANIFEST.md](PACKAGE-MANIFEST.md) records the exact freeze, lineage, document classification, and integrity data. [docs/EVIDENCE.md](docs/EVIDENCE.md) records facts through its ledger procedure.

## Future V0.1 outcome

When a separate product objective authorizes it, Emenda V0.1 is one strict-TypeScript product core and one Chromium Manifest V3 extension. It proposes at most one bounded correction through one directly configured local oMLX model by default or one explicitly selected OpenRouter model, derives the edit deterministically, presents the complete identifiable proposal, and applies only the writer-approved correction to the specified supported textarea surface.

The detailed provider, permission, privacy, lifecycle, input-provenance, rendering, and Apply contracts remain exactly those in [SPEC.md](SPEC.md). The six gates and seven increments remain exactly those in [docs/IMPLEMENTATION-PLAN.md](docs/IMPLEMENTATION-PLAN.md).

## Authorized v2.2.1 clarification and continuation

Under the existing human-authorized autonomous implementation and debugging objective, this successor atomic freeze on `docs/v2.2.1-freeze` and blueprint `main` has sole parent [`11080a3c4be943f57ea35293b995d80453ffecad`](https://github.com/duracell04/emenda/commit/11080a3c4be943f57ea35293b995d80453ffecad), tree `6955e79551d9a4ea372729bf75e7c3317e3da894`. The published `docs/v2.2.0-freeze` remains at that exact commit and tree. Its preserved v2.1.1 baseline is `7a65e69e915a19b310db19fd425cbd2cbd67fb54` (tree `07bd030daaf1cc791f8e6a4d44afedcf97828851`), and all earlier ancestry remains intact.

The authorized deltas are a semantic-preserving general clarification of the shared canonical prompt and the measured startup policy for the existing explicit local Settings test. The prompt clarifies profile/language mappings, compatible-clean versus unsupported decisions, compatibility before a necessary single local edit, literal final equality, and truthful actual-diff category/explanation; concise plain English explanations are preferred, not required. Only that worker-owned synthetic test has a fixed 25,000 ms full-processing bound, with the existing immutable fixture and production validation/derivation. Writing, discovery, OpenRouter and official corpus cases remain at 15,000 ms. No corpus examples, result repair, writing-deadline extension, automatic/background warm-up, application retry, quality reduction, profile/surface expansion, new dependency or privacy change is authorized.

Continue the local/default oMLX and explicit OpenRouter implementation in `https://github.com/duracell04/emenda-built` on the existing `build/local-omlx` branch. Preserve its `build/v0.1` baseline `fa27dbfe5d18f8c8cec5d5c5a6c04cf7259f66aa` and prior constitution intakes. Import this successor’s exact commit/tree and update audit expectations before adopting its prompt and explicit-test policy. The objective retains autonomous measured candidate-model selection, independent named agent semantic review of one complete 15/15 canonical corpus, coherent commits, publication of the exact tested state, and verified personal installation into the user’s existing Brave profile on the local Mac. Existing failed attempts remain factual history; affected behavior receives fresh verification under this freeze. Runtime facts and qualified models remain measured evidence rather than compiled defaults or universal compatibility claims.

## Completion

A documentation objective is complete when the Documentation Gate passes for the exact candidate tree, the atomic freeze commit is pushed to its authorized remote refs, remote identity and ancestry match, and the tracked worktree is clean.

A future implementation objective is complete when all seven increments and six gates pass for the recorded implementation tree and commit, that commit is pushed and verified in the implementation repository, the later ledger-only factual evidence commit is pushed and verified in this constitution repository, both tracked worktrees are clean, and the continuing personal objective additionally has the tested local extension installed and verified after Brave restart on the recorded Mac. As before, passing an intermediate check alone does not complete the objective.

Completion is a contract state: required behavior, evidence, repository integrity, remote identity, and the objective-specific terminal condition all hold.
