# Emenda Engineering Standard

> **Frozen engineering standard, version 2.3.0**

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

At initial implementation preflight, select one exact Node, npm, and TypeScript version tuple. The v2.3.0 continuation retains the baseline’s committed tuple unless an evidence-backed compatibility fix requires a separately coherent update. Commit that tuple before product source through exact engine metadata, an exact packageManager value, the exact TypeScript development dependency, and the implementation's toolchain record. Generate and commit the npm lockfile under that tuple.

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

Controlled tests prove canonical serialization, headers, fetch controls, prompt, schema, routing, plugin policy, completion/response bounds, exact returned model, timeout, incremental parsing, derivation and redacted outcomes. Add fake-clock and controlled-transport proof for lease ownership, coalescing, cache bounds, uncertain completion, local one-shot recovery, remote deadline release, restart markers and permanently fenced late callbacks. Distinguish client waiting/authority from unknown provider execution.

The unchanged 15-case corpus and semantic method remain authoritative. For this successor, deterministic equivalence against implementation `5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63` permits the existing sealed 15/15 `gemma-3-12b-it-4bit` run and two independent complete agent reviews to supply inherited linguistic qualification under Acceptance Section 6.3. Compare exact prompt/schema/serialized input/generation semantics, bounded context/focus and replayed validation/derivation. Scheduler and browser evidence are freshly established at the successor, not inherited from the model run.

A model/weight, prompt, profile, focus/context, schema, generation or derivation semantic change requires a new full qualification boundary. When fresh qualification is required, use one directly configured documented model, ephemeral credential delivery, strict sequential production validation and complete independent named agent semantic review. All 15 official cases retain 15 seconds; no implicit warm-up, preload, retry or case replacement occurs. Preserve failures and later recoveries separately.

Sprint 3/4 verification uses synthetic/mocked providers and deterministic equivalence. Sprint 5 uses one bounded synthetic explicit deep smoke on the exact release candidate, not exploratory model testing. The existing explicit local Settings Test alone retains its immutable fixture and 25-second bound; discovery/writing/official corpus stay at 15 seconds. A smoke admission failure records a limitation and ends that attempt; it does not authorize a model search, guard relaxation or retry loop.

### Browser

Keep these evidence layers distinct:

1. automated production-extension tests in Playwright bundled Chromium persistent context;
2. direct minimum-runtime compatibility on Chromium or Chrome for Testing 140;
3. actual installed-Brave unpacked-extension and personal-Mac smoke, including the toolbar permission prompt and restart.

Direct Chrome 140, current Chrome Stable and other-device compatibility results remain separately named evidence when performed; untested claims remain explicitly unproven.

Browser verification covers the supported textarea and refused editor classes, paired-input provenance, exposure, foreground and selection invalidation, storage isolation, synchronous worker listeners, exact-port permissions, sender validation, external permission changes, serialized lifecycle, worker restart, BFCache and prerender, navigation races, literal rendering, trusted approval controls, immediate pre-Apply authorization, scoped target selection, mutation failures, accessibility, IME, and exact one-step Undo.

Record the personal Mac/Brave results with exact chip, RAM, OS, browser/Chromium, oMLX and model versions, including cold/warm request latency and memory observations. Record Windows, Chromebook and other compatibility runs separately only when performed. Bind every support claim to its tested environment.

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
- **live:** real selected-provider behavior (local oMLX or explicitly selected OpenRouter);
- **runtime:** exact installed-Brave, minimum-version/current-Stable compatibility, or named-device smoke.

Every entry records UTC time, gate or increment, constitution freeze ID/commit/tree, applicable critical requirement IDs, tested implementation tree/commit, commands or actions, exact result, environment and toolchain, evidence level, limitations or failures, and next checkpoint.

The canonical evidence ledger is `docs/EVIDENCE.md` in the constitution repository; the implementation's `constitution/` snapshot remains read-only. An evidence commit in the constitution repository changes exactly that ledger and describes an already-existing tested implementation commit. Preserve failures and later recoveries separately. State the inspected scope, executed procedures, verified results, and open verification fields.

The live qualification process owns the selected credential and authorization header ephemerally for its lifetime; local unauthenticated calls require no fabricated key. Never persist or print credentials. Local server logging is configured to `critical`, loopback/model-fallback policy is verified, and synthetic canaries inspect ordinary app logs and Emenda artifacts. Local model KV state may remain enabled; native crash diagnostics are outside the extension’s guarantee. The active provider boundary owns raw private context and response bodies during the bounded call; raw bodies are discarded after processing. The worker registry alone may retain exact bounded input and validated derived outcomes within SPEC's transient cache. Durable recovery state contains only the strict text-free marker. Neither exception permits private text or browser/source identity in storage, logs, snapshots, errors or evidence. The current browser-authorization path exclusively owns page URL, tab/frame/document metadata, source identity, and DOM structure while establishing authority. Durable and observable records admit exactly synthetic domain-neutral fixtures, typed redacted outcomes, sanitized evidence fields, and build or test metadata.

## 10. Deferred engineering

The Deferred set is native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, broader editor support, packaging, signing, store publication, release automation, native placeholders, commercial infrastructure, and generalized cross-OS claims. Their future objectives introduce their own present-purpose architecture and verification.
