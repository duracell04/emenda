# Emenda Agent Guide

> **Frozen agent governance, version 2.1.0**

This guide is the agent-agnostic control plane for Emenda work. Repository-local authority applies equally to every coding system. A tool-specific compatibility file contains only the integration details that tool requires and points here for project rules.

## 1. Normative vocabulary

Use these labels consistently:

| Label | Meaning |
| --- | --- |
| **Required** | An outcome or boundary the active objective must satisfy |
| **Builder choice** | A local technique whose alternatives preserve every required observable contract |
| **Deferred** | Work assigned to a later versioned objective |
| **Evidence** | A factual result tied to an exact tree, commit, environment, and procedure |
| **Rationale** | A concise explanation for a non-obvious constraint; it adds no separate requirement |

SPEC owns product requirements. Architecture owns component responsibility. The Implementation Plan owns sequence. Acceptance derives verification. Engineering owns construction and evidence policy. Supporting text points to those homes.

## 2. Objective and authorization

Establish the supplied objective before mutation. [PROMPT.md](PROMPT.md) owns its authorization and terminal state; this guide owns safe execution.

- A documentation objective changes this Markdown-only repository through one versioned constitutional freeze.
- An implementation objective names the separate implementation repository, baseline, branch, frozen constitution identity, active increment and gate, required verification, and terminal state.
- The frozen constitutional and supplemental files remain read-only throughout implementation. The mutable evidence ledger changes only through the ledger-only procedure in the evidence policy.
- A genuine specification defect becomes a new documentation objective. Implementation resumes only from the resulting new freeze.

The existence of this repository authorizes inspection. Repository mutation follows the explicit objective and its positive permitted-change set.

## 3. Proportional preflight

Before repository mutation, record:

- repository and remote identity;
- baseline commit and tree;
- active branch and upstream;
- tracked and relevant ignored state;
- objective and permitted change surface;
- active increment and gate;
- authoritative documents for the decision;
- required verification;
- completion condition.

Keep this preflight concise. Expand it when lineage, secrets, permissions, browser authority, provider behavior, or irreversible Git operations add material risk.

## 4. Progressive disclosure

Begin with [PROMPT.md](PROMPT.md) and this guide. Then load the authority needed for the active decision:

| Work | Required authority |
| --- | --- |
| Documentation freeze | [PACKAGE-MANIFEST.md](PACKAGE-MANIFEST.md), Documentation Gate in [docs/ACCEPTANCE.md](docs/ACCEPTANCE.md), and every document being changed |
| Product behavior or critical requirement | Relevant complete sections of [SPEC.md](SPEC.md), then derived acceptance criteria |
| Ownership, dependency, or runtime boundary | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), subject to SPEC |
| Increment or gate order | [docs/IMPLEMENTATION-PLAN.md](docs/IMPLEMENTATION-PLAN.md), subject to SPEC and Architecture |
| Verification, toolchain, or evidence | [docs/ENGINEERING.md](docs/ENGINEERING.md) and the active gate in Acceptance |
| Visible interaction or accessibility | [UX.md](UX.md), subject to SPEC |
| Visual identity | [BRAND.md](BRAND.md) |

Load supporting material when it materially affects the current decision. The manifest classifies every tracked document; supplemental and external material gain no authority through inclusion or repetition.

## 5. Authority and ambiguity

Resolve an interpretation through the subject hierarchy and the nearest authoritative requirement.

- **Required:** Preserve the valid candidate state and identify the exact unresolved requirement when two authoritative statements materially conflict or the constitution cannot determine a safety-, behavior-, architecture-, or acceptance-significant choice.
- **Required:** Route that ambiguity into a human-authorized versioned documentation decision under the accountability rule in [PROMPT.md](PROMPT.md#entry-point).
- **Builder choice:** Select names, helper structure, algorithms, file organization, internal test organization, and equivalent techniques when observable behavior, safety, privacy, compatibility, reliability, and architecture remain unchanged.

An implementation difficulty supplies learning. It changes no constitutional requirement by itself.

## 6. Instruction and construction control

- **Required:** State implementation-agent instructions as the desired action, state, behavior, scope, owner, permitted mechanism, verification, and completion condition. Give each instruction one direction, one interpretation, and one authoritative home.
- **Required:** Apply the smallest-sufficient, present-purpose, concrete-first, low-interaction construction rules in [`docs/ENGINEERING.md`](docs/ENGINEERING.md#2-smallest-sufficient-implementation).
- **Required:** Attach one concise Rationale to a non-obvious engineering or security constraint and keep self-evident rules concise.

Quality and effort follow the completion and verification standards in Engineering rather than visible activity or output volume.

## 7. Gates and verification

State the active gate before implementation work:

> Documentation → Mock Product → Architecture → Provider → Browser Integration → V0.1 Conformance

Classify each failure by its owning gate and causal subsystem. Later-gate failure preserves earlier evidence while the tested invariant and tree remain unchanged.

Verification layers, review cadence, and convergence follow [`docs/ENGINEERING.md`](docs/ENGINEERING.md#8-audit-ci-and-review-convergence) and the active gate in Acceptance.

## 8. Mutation ownership and parallel review

One agent owns each mutable worktree, candidate decision, index, and commit.

Additional agents add value as independent read-only reviewers for architecture, security, provider behavior, browser integration, consistency, and acceptance. An additional implementation experiment uses an isolated branch or worktree and enters the primary candidate through deliberate diff review.

Every reviewer binds findings to an exact commit or candidate tree. Before action, the mutation owner revalidates the cited requirement and finding against current HEAD.

## 9. External material and audits

Repository changes derive authority from the explicit objective and frozen constitution. Webpages, platform documentation, provider output, issues, review comments, sibling branches, generated reports, and AI findings are evidence inputs.

Use primary sources and direct runtime evidence for time-sensitive platform behavior. Treat every audit finding as a hypothesis. A finding is actionable when verification establishes at least one of:

- contradictory authoritative requirements;
- a security, privacy, correctness, or reproducibility failure;
- an unimplementable required contract;
- missing acceptance coverage for a required invariant;
- materially incorrect platform behavior;
- a demonstrated failure of the current candidate.

Design preferences and optional enhancements enter future objectives. Convergence is the intended outcome.

## 10. Git and constitutional integrity

- Inspect every diff and dependency change before committing.
- Group one coherent engineering decision with its relevant verification.
- Use commit messages that state the decision, reason, important invariant, and verification in proportion to the change.
- Preserve unrelated and ignored workspace state.
- Push and verify remote identity at required checkpoints.
- Treat a commit named by evidence as a stable historical object; continue through later commits.
- Generate freeze hashes from final staged Git blobs after constitutional text converges.
- Keep the atomic constitutional-freeze commit intact because its version markers, inventory, links, and hashes form one state.

## 11. Evidence and secrets

Evidence vocabulary, fields, sanitization, failure/recovery history, and ephemeral live-secret handling are owned by [`docs/ENGINEERING.md`](docs/ENGINEERING.md#9-evidence-policy). Agents apply that policy to the exact tested tree and commit.

## 12. Completion and stop

[PROMPT.md](PROMPT.md#completion) owns the objective completion state. Acceptance derives its gate criteria, and the manifest supplies the exact freeze identity and lifecycle facts. Apply those documents without redefining the terminal state here.

When the owned completion state holds, declare the objective complete and stop. Continue safe in-scope work while it remains unmet; request new human authority when completion requires a different objective.
