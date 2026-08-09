# Emenda AI-Native Engineering Standard

> **Frozen engineering standard, version 1.0.0**

## 1. Purpose

Emenda is engineered for maximum useful coding-agent autonomy inside a small, mechanically constrained solution space.

## 2. Outcome loyalty

Optimize for the writer's observable outcome:

```text
observe useful text
→ ask OpenRouter at the right moment
→ validate
→ show the exact change
→ apply safely
```

Infrastructure earns its place by strengthening this chain or protecting a necessary invariant.

## 3. Hard platform discipline

The shared product expresses semantics.

Leaf bindings express mechanisms.

The pre-binding implementation consists entirely of platform-neutral product code, mocks, provider code, presentation code, and tests.

Native dependencies and target-specific modules enter only after the Mock Product Gate and Architecture Gate pass.

## 4. Complexity proportionality

```text
implementation complexity
∝
demonstrated product capability
```

Every active:

```text
dependency
feature flag
build mode
service
binding
script
configuration layer
abstraction
test harness
```

answers:

> Which current product requirement or necessary invariant earns this complexity?

## 5. Simplification order

Use this order:

```text
remove requirement
→ defer requirement
→ use existing stack capability
→ use one small explicit seam
→ add new machinery after evidence
```

Deletion and deferral are engineering actions.

## 6. Guardrail-first technology

### Safe Rust

Use safe Rust for:

```text
domain
controller
revision
context policy
correction validation
OpenRouter provider binding
native binding orchestration
```

At the application crate root:

```rust
#![forbid(unsafe_code)]
```

Platform libraries may encapsulate native unsafe internals behind safe APIs. Emenda application code remains safe Rust.

### Strict TypeScript

Use strict TypeScript for the compact desktop presentation and future browser binding.

### Serde and Zod

Use Serde at Rust serialization boundaries and Zod for important runtime values entering the presentation.

### JSON Schema

Use the smallest model-response schema that supports deterministic validation.

### Tauri capabilities

Use Tauri capabilities as an explicit authority boundary for presentation surfaces.

## 7. Dependency policy

A dependency enters when all conditions hold:

1. a current capability requires it;
2. it reduces total code or risk;
3. one module clearly owns it;
4. it preserves dependency direction;
5. future agents can understand its role quickly.

Record the rationale in the introducing commit.

## 8. Abstraction policy

Abstract demonstrated external variability:

```text
TextSurface
InferenceProvider
```

Keep stable internal behavior direct and explicit.

A small port is stronger than a generalized framework.

## 9. Mock-first proof

Mocks are architecture evidence rather than test convenience.

`MockTextSurface` proves that the application owns the text-surface semantics.

`MockInferenceProvider` proves that the controller owns deterministic orchestration around probabilistic inference.

The complete product loop passes through mocks before any native binding work.

## 10. Deterministic shell around probabilistic inference

```text
deterministic state
→ explicit request
→ probabilistic model
→ machine-readable response
→ deterministic parse
→ deterministic semantic validation
→ deterministic state transition
```

OpenRouter handles linguistic judgment.

Emenda owns:

```text
timing
revision
context policy
text identity
validation
state
writer decision
side effects
```

## 11. Failure classification

Classify by causal layer:

```text
Configuration
Observation
Context
Transport
Protocol
Semantic
Validation
Stale state
Platform protection
Replacement
Environment
Release
```

Fix the layer that violated its contract.

A failed command is evidence about one gate, not a universal product verdict.

## 12. Verification follows risk

Evidence order:

```text
domain invariants
→ correction validity
→ revision freshness
→ debounce and context policy
→ mock product loop
→ provider contract
→ presentation boundary
→ architecture conformance
→ native binding behavior
→ real user loop
```

Test count is secondary to the invariant established.

## 13. Product, host, and release gates

```text
PRODUCT
shared behavior and mocks

PROVIDER
external model contract

HOST
native binding on one environment

RELEASE
packaging, signing, installation, trust
```

Each gate has its own stop condition and evidence.

## 14. Commit discipline

One commit represents one independently verifiable product invariant or architectural decision.

```text
inspect
→ implement
→ verify
→ inspect diff
→ commit
→ push
→ verify pushed state
→ continue
```

Commit messages explain:

```text
what changed
why it exists
architectural fit
important tradeoff
verification
```

## 15. Repository quality

Keep the repository:

```text
compact
typed
explicit
modular
testable
easy to navigate
easy to understand in one pass
```

Prefer moving complexity behind an existing port before adding a new layer.

## 16. Presentation engineering

V0.1 uses:

```text
strict TypeScript
HTML
CSS
```

A UI framework enters only when measured presentation complexity makes the total system simpler.

## 17. Configuration engineering

V0.1 uses a small environment contract:

```text
OPENROUTER_API_KEY
OPENROUTER_MODEL
```

Avoid configuration systems whose capability is unused by the current product.

## 18. Architecture review questions

Ask after every structural increment:

```text
Can shared logic be understood without a host operating system?
Can mocks exercise the complete product loop?
Can a native binding change without changing product semantics?
Does the frontend receive display-safe state only?
Does every dependency earn its cost?
```

## 19. Documentation discipline

Documentation is persistent governance.

Use:

```text
README.md                 human orientation
PROMPT.md                 autonomous objective
SPEC.md                   product source of truth
ROADMAP.md                milestone sequence
AGENTS.md                 agent rules
docs/ARCHITECTURE.md      dependency direction
docs/IMPLEMENTATION-PLAN.md build increments
docs/ACCEPTANCE.md        evidence standard
UX.md                     interaction rules
BRAND.md                  visual system
```

Factual status follows evidence. Constitutional changes create a new frozen package version.

## 20. Stop condition

When the active gate passes:

```text
verify
→ document evidence
→ commit
→ push
→ report
→ stop
```

Further work begins under a new objective.

## 21. Canonical principle

> **Specify the writer outcome precisely, keep shared semantics independent from environment mechanisms, prove the product through mocks, choose technology that rejects plausible agent mistakes, and let complexity grow only with measured product value.**
