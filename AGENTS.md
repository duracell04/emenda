# Agent Governance

This file defines the standing working rules for AI coding agents operating inside the Emenda repository.

`README.md` explains Emenda to humans.

`SPEC.md` is the engineering source of truth for what to build, why the architecture is shaped that way, and how implementation decisions should be resolved.

`AGENTS.md` defines how an AI coding agent should execute work inside the repository.

## Working Principle

Work in small, coherent implementation steps that preserve a readable repository and a high-resolution development history.

Use the project specification, existing architecture, type system, tests and repository state as the primary decision context.

When an implementation detail remains open, prefer the smallest solution that completes the current vertical slice cleanly and preserves the established architectural boundaries.

## Commit Discipline

Use small, atomic commits throughout development.

Each commit should represent one coherent implementation step that can be understood, reviewed and reverted independently.

Commit as soon as a logical step is complete and its relevant checks pass.

Prefer several meaningful commits over one large implementation dump.

## Commit Messages

Write detailed commit messages that explain:

- what changed
- why the change was made
- how it fits the Emenda architecture or current implementation step
- any important implementation decision or trade-off
- what was tested or verified

Use a concise descriptive subject followed by a detailed body when the change benefits from explanation.

Example:

```text
feat: add immutable revision tracking for correction requests

Introduce a monotonically increasing revision ID for every text snapshot.

This keeps OpenRouter responses bound to the text version that produced
them. A newer invocation becomes authoritative, so delayed responses from
earlier revisions can be recognised as stale before they reach suggestion
state.

Added unit coverage for current and stale revision handling.

This implements the snapshot discipline defined in SPEC.md.
```

## Verification Before Commit

Run the checks relevant to the implementation step before committing it.

For the complete repository, the expected health checks are:

```bash
npm run typecheck
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri build
```

During early scaffolding, run the subset that exists and applies to the current repository state.

Record meaningful verification in the commit body when useful.

## Architectural Discipline

Keep Emenda small enough to understand as a whole.

Maintain the established responsibility split:

- React + strict TypeScript for product UI and frontend application state
- Tauri as the application boundary
- Rust for privileged local operations, text transport, secure credentials, snapshots, validation and OpenRouter communication
- OpenRouter for linguistic intelligence

Keep abstractions proportional to demonstrated needs. Create small seams where they protect the current correction workflow or a clearly established future boundary.

Treat AI output as external untrusted data and validate it before it enters application state.

Keep text identity, revisions, security and source-text replacement deterministic in the local application.

## LLM Boundary and Failure Semantics

Emenda is deterministic software around a narrow probabilistic component. Preserve this boundary in implementation and testing decisions.

### Boundary Flow

```text
deterministic state
→ explicit LLM contract
→ probabilistic model
→ machine-readable response
→ deterministic validation
→ deterministic application
```

Emenda owns orchestration, state, validation, security, snapshots, error classification and text replacement. OpenRouter owns the linguistic transformation.

Treat model responses as raw external data until deterministic validation succeeds.

### Request Design

Design requests for the weakest model admitted by the selected execution path.

- State one precise linguistic task.
- Provide the exact input text and necessary context.
- Define the permitted transformation.
- Request only the minimum output Emenda needs.
- Prefer provider-enforced structured output with a minimal JSON Schema where supported.
- Require the fields Emenda consumes and set `additionalProperties: false` where compatible.
- Specify the exact response structure in the prompt when provider-level enforcement is unavailable.
- Validate every returned response independently before it enters application state.
- Keep application behavior, formatting, state transitions and error handling deterministic inside Emenda.

### Failure Classification

Classify each failure by causal layer before selecting a recovery strategy.

1. **Transport** — timeout, connection failure, rate limiting or transient provider failure.
2. **Protocol** — invalid JSON, schema violation or failure to satisfy Emenda's machine-readable response contract.
3. **Semantic** — schema-valid output that fails the requested linguistic transformation or quality threshold.
4. **Application** — valid output that cannot safely apply because of state, snapshot, replacement, UI or other deterministic application logic.

Address each failure at the layer that produced it.

### Retry and Fallback Policy

Base retries on typed retryability.

- Use bounded retry or backoff for failures classified as transient and retryable.
- Treat protocol failures as contract or capability diagnostics first; strengthen the request, schema or provider/model compatibility before introducing recovery behavior.
- Treat semantic failures as instruction or model-quality diagnostics.
- Keep production fail-closed by applying output only after deterministic validation and state checks succeed.
- Keep every model substitution or fallback explicit and observable so diagnostics preserve the identity of the model path that failed and the path that succeeded.

### `openrouter/free`

`openrouter/free` may route across underlying models with different capabilities.

- Design the model-facing contract for simplicity and broad compatibility.
- Validate every response independently.
- Test compatibility explicitly rather than inferring it from endpoint availability.
- Record repeated contract failures as evidence that a model or routing path is incompatible with Emenda's required protocol.

### Smoke Tests and Root-Cause Invariants

Use smoke tests to expose architectural behavior and preserve diagnostic signal.

When a smoke test fails, identify whether the violated invariant belongs to Emenda, the model contract, provider compatibility, the test environment or the harness itself.

Prefer the smallest change that restores the violated invariant.

Keep availability sampling across multiple free-router selections separate from strict protocol-compliance tests. A protocol-compliance test should establish whether the original interaction satisfied the contract rather than convert repeated failures into apparent success through silent model replacement.

Treat a passing test as useful evidence when the behavior it measures matches the architecture's intended guarantee.

## Scope Discipline

Complete the active vertical slice before expanding feature breadth.

For V0.1, prioritise the full selected-text correction loop:

```text
select text
→ hotkey
→ capture
→ snapshot
→ OpenRouter
→ validate
→ review
→ apply
→ source application
```

Use the current specification and decision matrix to resolve scope questions.

## Documentation Discipline

Keep repository documentation aligned with implementation.

- `README.md` remains a concise one-minute orientation page.
- `SPEC.md` carries architecture, rationale, contracts, decision matrix, implementation order and test strategy.
- `AGENTS.md` carries standing coding-agent governance.

Update the relevant document when an implemented architectural decision materially changes the source of truth.

## Repository History Principle

> **The Git history should tell the story of how and why Emenda was built.**

Every commit should make the repository easier to audit, understand, review and continue developing with both humans and AI coding agents.
