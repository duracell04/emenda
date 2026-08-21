# Emenda Engineering Standard

> **Frozen engineering standard, version 2.1.1**

## 1. Authority

This document owns implementation quality, the canonical toolchain policy, verification layers, evidence vocabulary, and review convergence. Product behavior belongs to [SPEC.md](../SPEC.md), architectural ownership belongs to [ARCHITECTURE.md](ARCHITECTURE.md), build order belongs to [IMPLEMENTATION-PLAN.md](IMPLEMENTATION-PLAN.md), gate-specific pass criteria belong to [ACCEPTANCE.md](ACCEPTANCE.md), and repository operating and Git discipline belongs to [AGENTS.md](../AGENTS.md).

## 2. Smallest sufficient implementation

- **Required:** Choose the simplest clear implementation that completely satisfies the current frozen contract.
- **Required:** Give every dependency, abstraction, interface, layer, service, configuration path, asynchronous boundary, and extension point a present-requirement justification.
- **Required:** Implement the current concrete case before generalizing from demonstrated common structure.
- **Required:** Minimize concepts, ownership boundaries, synchronization, states, interfaces, dependencies, and failure modes across the whole product.
- **Required:** Prefer direct typed code and explicit boundary conversion over defensive machinery spread through trusted internals.
- **Deferred:** Future runtime families, product surfaces, infrastructure, and speculative extension points enter only through their future objectives.

Maintenance burden is an acceptance concern. Keep the global interaction surface at the smallest size that satisfies the frozen contract, and add surface when a present requirement justifies it.

## 3. Canonical toolchain and dependency set

At future implementation preflight, select one exact Node, npm, and TypeScript version tuple. Commit that tuple before product source through exact engine metadata, an exact packageManager value, the exact TypeScript development dependency, and the implementation's toolchain record. Generate and commit the npm lockfile under that tuple.

That tuple is canonical for the objective. The audit passes when the running versions match it. Label another-environment run as compatibility evidence; canonical conformance comes from the canonical tuple. Package scripts and verification resolve executables from committed, lockfile-installed local tools.

The direct runtime dependency set is exactly Zod. The development dependency set is exactly TypeScript, esbuild, Vitest, Playwright, Chrome types, and Node types. Every direct version is exact. Clean verification starts with npm ci from a clean checkout and validates that package metadata, direct versions, lockfile, and installed graph agree.

The product is one npm package implemented with plain TypeScript, HTML, and CSS and the exact dependency set above. UI and extension frameworks, provider SDKs, monorepo layers, backends, databases, code generators, native scaffolds, and remote executable code belong to future authorized objectives.

## 4. Compiler-enforced safety

Enable TypeScript `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitOverride`, and `noFallthroughCasesInSwitch`. Use precise immutable domain values, narrow interfaces, exhaustive discriminated unions, and explicit conversions at external boundaries. Represent unchecked external input as `unknown` until its owning boundary validates and converts it.

Ordinary `tsc` compilation covers every repository-authored TypeScript file in product source, tests, configuration, and build scripts under the committed configurations. The core configuration supplies exactly its ECMAScript library types and core-authored declarations; the extension configuration supplies its declared browser and tooling types. Installed dependencies, build output, and the copied constitution sit outside the repository-authored compilation scope.

Use compiler narrowing, boundary validators, `satisfies`, and `as const` as ordinary type-preserving techniques. Assertions and diagnostic suppressions are permitted exactly where a genuine external API typing mismatch leaves the validated runtime contract inexpressible. Keep each exception at the smallest expression, add an adjacent plain-language Rationale naming the external contract, and verify that boundary with a focused compile-time or deterministic runtime test.

The Architecture Gate combines compiled evidence that every committed TypeScript configuration passes, inspected evidence that types and exceptional boundaries remain precise, and focused deterministic evidence for the runtime basis of each exception. The standard TypeScript compiler and focused tests are the complete compiler-safety mechanism.

## 5. Deterministic and boundary discipline

The named invariant in [SPEC.md](../SPEC.md#deterministic-authority-and-probabilistic-judgment) controls implementation:

- policy functions and the reducer are pure;
- a minimal fake-clock-compatible scheduler seam owns deterministic time;
- revision equality establishes authority, while cancellation saves work;
- every asynchronous completion rechecks current authority before state, presentation, or text can change;
- product coordinates are Unicode scalar offsets with explicit lossless browser conversion;
- validation is concentrated at model, protocol, trusted-settings, browser-authority, capture, mapping, and mutation boundaries;
- ambiguous data produces the typed fail-closed outcome owned by SPEC;
- the undo-aware mutation leaf remains isolated and receives real-browser evidence.

## 6. Verification layers

### Static and compiled

Inspect constitution identity, dependency and import direction, permitted schema locations, exceptional type boundaries, manifest, permissions, bundle contents, and credential/text confinement. Compile core with exactly its ECMAScript library types and core-authored declarations; compile the extension under its browser configuration.

### Deterministic

Use Vitest, fake clocks, deterministic surface and provider simulations, and controlled fetch/message doubles. Cover Unicode ranges, exact caret ownership, trusted paired-input tickets, context selection, reducer transitions, configuration races, cancellation order, stale work, foreground and exposure, selection authority, validation, redaction, IME commitment, Apply, Dismiss, self-authored selection and mutation, and refusal.

Timing tests control exact boundaries and completion order. A timing fix changes the deterministic model or owning invariant. Every deterministic gate assertion passes.

### Provider

Controlled tests prove the canonical serialization, authored headers, fetch controls, prompt, routing, structured-output schema, empty plugin set, declared non-reasoning response projection, completion bound, exact returned model identity, timeout, incremental body limit, cancellation, local derivation, and redacted failures.

The canonical 15-case corpus runs strictly sequentially through production validation using one configured documented direct model and ephemeral environment delivery of the writer's credential. Named human reviewers collectively competent for all corpus profiles apply the semantic method in Acceptance after automated structural checks and record their profile/case coverage. A failed or interrupted run remains factual evidence; a complete later run records recovery.

### Browser

Keep these evidence layers distinct:

1. automated production-extension tests in Playwright bundled Chromium persistent context;
2. direct minimum-runtime compatibility on Chromium or Chrome for Testing 140;
3. manual unpacked-extension smoke in current Chrome Stable with the actual toolbar permission prompt.

Browser verification covers the supported textarea and refused editor classes, paired-input provenance, exposure, foreground and selection invalidation, storage isolation, synchronous worker listeners, exact-port permissions, sender validation, external permission changes, serialized lifecycle, worker restart, BFCache and prerender, navigation races, literal rendering, trusted approval controls, immediate pre-Apply authorization, scoped target selection, mutation failures, accessibility, IME, and exact one-step Undo.

Record Windows Studio, MacBook, and Chromebook results separately with their exact OS and browser versions. Bind each record's support claim to its tested environment.

## 7. Test-value discipline

Every meaningful test protects a product behavior, state transition, trust boundary, regression, privacy rule, authority rule, or documented risk. Focus tests on externally meaningful invariants and semantic ports.

Apply verification cost where it produces information: focused deterministic checks during construction; complete deterministic gates at convergence; live provider evidence at the Provider Gate; browser and physical-device evidence at their owning gates. Evaluate the suite by contract coverage and information gained.

## 8. Audit, CI, and review convergence

The implementation provides one cross-platform audit command as the sole audit entry point. It is read-only with respect to constitution and product sources and orchestrates every check available at the active phase:

- read-only constitution snapshot and lock verification;
- all 11 independent constitution checksums;
- canonical toolchain and clean npm installation;
- strict compilation and focused exceptional-boundary verification;
- deterministic tests;
- production build, dependency, permission, manifest, and bundle inspection;
- browser integration when the environment supports it;
- final critical-requirement coverage and evidence checks.

Its internal helpers and output format are Builder choices. Keep every helper internal and reach it through this command.

The deterministic CI workflow uses one invocation of that audit command as its complete audit path. Record environment-dependent live-provider, minimum-Chrome, current-Stable, and physical-device results as their separate evidence layers.

Use focused review while constructing and one complete consistency, security, architecture, and acceptance review for the exact final candidate at the owning gate. Repeat complete review after a substantive change to an audited invariant. Freeze when every verified actionable blocker is resolved and required checks pass.

SPEC is the sole home for critical-requirement definitions and semantic ownership; derived documents reference their IDs. Keep each ID permanently bound to one semantic requirement, assign a fresh ID to new semantics, and audit acceptance coverage for every active ID.

## 9. Evidence policy

Use these exact levels:

- **inspected:** static source, diff, configuration, or artifact inspection;
- **compiled:** compiler or build completion;
- **deterministic:** controlled automated behavior;
- **integration:** automated persistent-Chromium extension behavior;
- **live:** real OpenRouter behavior;
- **runtime:** minimum-version, current-Stable, or named-device smoke.

Every entry records UTC time, gate or increment, constitution freeze ID/commit/tree, applicable critical requirement IDs, tested implementation tree/commit, commands or actions, exact result, environment and toolchain, evidence level, limitations or failures, and next checkpoint.

The canonical evidence ledger is `docs/EVIDENCE.md` in the constitution repository; the implementation's `constitution/` snapshot remains read-only. An evidence commit in the constitution repository changes exactly that ledger and describes an already-existing tested implementation commit. Preserve failures and later recoveries separately. State the inspected scope, executed procedures, verified results, and open verification fields.

The live qualification process environment exclusively owns the API key and authorization header for its lifetime. The active provider boundary exclusively owns raw private context and response bodies during the bounded call. The current browser-authorization path exclusively owns page URL, tab/frame/document metadata, source identity, and DOM structure while establishing authority. Durable and observable records admit exactly synthetic domain-neutral fixtures, typed redacted outcomes, sanitized evidence fields, and build or test metadata.

## 10. Deferred engineering

The Deferred set is native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, broader editor support, packaging, signing, store publication, release automation, native placeholders, commercial infrastructure, and generalized cross-OS claims. Their future objectives introduce their own present-purpose architecture and verification.
