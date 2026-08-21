# Emenda Engineering Standard

> **Frozen engineering standard, version 2.1.0**

## 1. Authority

This document owns implementation quality, the canonical toolchain policy, verification layers, evidence vocabulary, and review convergence. Product behavior belongs to [SPEC.md](../SPEC.md), architectural ownership belongs to [ARCHITECTURE.md](ARCHITECTURE.md), build order belongs to [IMPLEMENTATION-PLAN.md](IMPLEMENTATION-PLAN.md), gate-specific pass criteria belong to [ACCEPTANCE.md](ACCEPTANCE.md), and repository operating and Git discipline belongs to [AGENTS.md](../AGENTS.md).

## 2. Smallest sufficient implementation

- **Required:** Choose the simplest clear implementation that completely satisfies the current frozen contract.
- **Required:** Give every dependency, abstraction, interface, layer, service, configuration path, asynchronous boundary, and extension point a present-requirement justification.
- **Required:** Implement the current concrete case before generalizing from demonstrated common structure.
- **Required:** Minimize concepts, ownership boundaries, synchronization, states, interfaces, dependencies, and failure modes across the whole product.
- **Required:** Prefer direct typed code and explicit boundary conversion over defensive machinery spread through trusted internals.
- **Deferred:** Future runtime families, product surfaces, infrastructure, and speculative extension points enter only through their future objectives.

Maintenance burden is an acceptance concern. Local elegance does not justify a larger global interaction surface.

## 3. Canonical toolchain and dependency set

At future implementation preflight, select one exact Node, npm, and TypeScript version tuple. Commit that tuple before product source through exact engine metadata, an exact packageManager value, the exact TypeScript development dependency, and the implementation's toolchain record. Generate and commit the npm lockfile under that tuple.

That tuple is canonical for the objective. The audit command verifies the running versions and refuses a mismatch. A separately labelled compatibility run may use another environment; it does not replace canonical conformance evidence. Package scripts invoke committed local tools, so verification never depends on a global compiler or a download-at-run-time executable.

The direct runtime dependency set is exactly Zod. The development dependency set is exactly TypeScript, esbuild, Vitest, Playwright, Chrome types, and Node types. Every direct version is exact. Clean verification starts with npm ci from a clean checkout and validates that package metadata, direct versions, lockfile, and installed graph agree.

The product is one npm package implemented with plain TypeScript, HTML, and CSS. The allowed set supplies no UI framework, extension framework, provider SDK, monorepo layer, backend, database, code generator, native scaffold, or remote executable code.

## 4. Compiler-enforced safety

Enable strict TypeScript plus unchecked-indexed-access, exact-optional-property, override, and fallthrough checks. Use immutable domain values, narrow interfaces, exhaustive discriminated unions, and explicit conversions at external boundaries.

The deterministic compiler audit covers every repository-authored TypeScript file, including product, tests, configuration, and build scripts, while excluding installed dependencies, build output, and the copied constitution. It enforces:

- the compiler reports no implicit any;
- an explicit any type annotation, assertion, or type argument is absent;
- a nested assertion through any or unknown, including the double-assertion pattern, is absent;
- a non-null assertion expression is absent;
- ts-ignore is absent;
- ts-expect-error is accepted only in an immediately preceding line comment matching `^// @ts-expect-error EM-BOUNDARY: [A-Za-z0-9._/-]+; verified by [A-Za-z0-9._/-]+$`, with concrete contract and validator-or-test identifiers, and the compiler proves it suppresses a current diagnostic;
- as-const and satisfies remain ordinary type-preserving techniques;
- every other type assertion is immediately preceded by a line comment matching `^// EM-BOUNDARY-ASSERTION: [A-Za-z0-9._/-]+; verified by [A-Za-z0-9._/-]+$`, with concrete contract and validator-or-test identifiers;
- every dispatch over a declared discriminated union proves its remaining value is never in a final branch, either through a canonical assertNever function whose parameter type is never or a local assignment to never.

The TypeScript compiler API and deterministic source inspection enforce the enumerated syntax, scope, comment grammar, placeholder rejection, and exhaustive-never forms without a lint dependency. Placeholder identifiers such as `external-contract`, `validator`, or `test` fail the audit.

Architecture Gate review separately inspects that each assertion is necessary, uses the narrowest target type supported by the named contract, and converts only after the named validator or test establishes its runtime basis. Those necessity and narrowness conclusions are inspected judgments, not scanner proof.

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

Inspect constitution identity, dependency and import direction, permitted schema locations, type escapes, manifest, permissions, bundle contents, and secret/text leakage. Compile core with DOM, Chrome, Node, React, and extension ambient types unavailable; compile the extension under its browser configuration.

### Deterministic

Use Vitest, fake clocks, deterministic surface and provider simulations, and controlled fetch/message doubles. Cover Unicode ranges, exact caret ownership, trusted paired-input tickets, context selection, reducer transitions, configuration races, cancellation order, stale work, foreground and exposure, selection authority, validation, redaction, IME commitment, Apply, Dismiss, self-authored selection and mutation, and refusal.

Timing tests control exact boundaries and completion order. A timing fix changes the deterministic model or owning invariant rather than adding arbitrary waits. Every deterministic gate assertion passes.

### Provider

Controlled tests prove the canonical serialization, authored headers, fetch controls, prompt, routing, structured-output schema, disabled plugins, reasoning-trace exclusion, completion bound, response projection, exact returned model identity, timeout, incremental body limit, cancellation, local derivation, and redacted failures.

The canonical 15-case corpus runs strictly sequentially through production validation using one configured documented direct model and ephemeral environment delivery of the writer's credential. Named human reviewers collectively competent for all corpus profiles apply the semantic method in Acceptance after automated structural checks and record their profile/case coverage. A failed or interrupted run remains factual evidence; only a complete later run records recovery.

### Browser

Keep these evidence layers distinct:

1. automated production-extension tests in Playwright bundled Chromium persistent context;
2. direct minimum-runtime compatibility on Chromium or Chrome for Testing 140;
3. manual unpacked-extension smoke in current Chrome Stable with the actual toolbar permission prompt.

Browser verification covers the supported textarea and refused editor classes, paired-input provenance, exposure, foreground and selection invalidation, storage isolation, synchronous worker listeners, exact-port permissions, sender validation, external permission changes, serialized lifecycle, worker restart, BFCache and prerender, navigation races, literal rendering, trusted approval controls, immediate pre-Apply authorization, scoped target selection, mutation failures, accessibility, IME, and exact one-step Undo.

Record Windows Studio, MacBook, and Chromebook results separately with their exact OS and browser versions. Each record supports only its tested environment.

## 7. Test-value discipline

Every meaningful test protects a product behavior, state transition, trust boundary, regression, privacy rule, authority rule, or documented risk. Test externally meaningful invariants and semantic ports rather than private helper shape.

Apply verification cost where it produces information: focused deterministic checks during construction; complete deterministic gates at convergence; live provider evidence at the Provider Gate; browser and physical-device evidence at their owning gates. Contract coverage matters; raw test count does not.

## 8. Audit, CI, and review convergence

The implementation provides one cross-platform audit command as the sole audit entry point. It is read-only with respect to constitution and product sources and orchestrates every check available at the active phase:

- read-only constitution snapshot and lock verification;
- all 12 independent constitution checksums;
- canonical toolchain and clean npm installation;
- compilation and type-escape inspection;
- deterministic tests;
- production build, dependency, permission, manifest, and bundle inspection;
- browser integration when the environment supports it;
- final critical-requirement coverage and evidence checks.

Its internal helpers and output format are Builder choices. Every helper is reached through this command; no helper is exposed as another audit entry point.

The deterministic CI workflow invokes that sole audit command exactly once and adds no separately reconstructed or parallel audit path. Environment-dependent live-provider, minimum-Chrome, current-Stable, and physical-device results remain separately recorded evidence.

Use focused review while constructing and one complete consistency, security, architecture, and acceptance review for the exact final candidate at the owning gate. Repeat complete review after a substantive change to an audited invariant. Freeze when every verified actionable blocker is resolved and required checks pass.

Critical requirement definitions and semantic ownership live only in SPEC; derived documents may reference them. IDs remain stable while semantics remain stable, retired IDs are never reused, and the audit proves that every active ID has acceptance coverage.

## 9. Evidence policy

Use these exact levels:

- **inspected:** static source, diff, configuration, or artifact inspection;
- **compiled:** compiler or build completion;
- **deterministic:** controlled automated behavior;
- **integration:** automated persistent-Chromium extension behavior;
- **live:** real OpenRouter behavior;
- **runtime:** minimum-version, current-Stable, or named-device smoke.

Every entry records UTC time, gate or increment, constitution freeze ID/commit/tree, applicable critical requirement IDs, tested implementation tree/commit, commands or actions, exact result, environment and toolchain, evidence level, limitations or failures, and next checkpoint.

The canonical evidence ledger remains `docs/EVIDENCE.md` in the constitution repository; the implementation's `constitution/` snapshot remains read-only. An evidence commit in the constitution repository changes only that ledger and describes an already-existing tested implementation commit. Preserve failures and later recoveries separately. State what was inspected, what was executed, and what remains unverified.

Live credentials exist only in the qualification process environment for its lifetime. Evidence, logs, fixtures, snapshots, commits, and errors contain no API key, authorization header, raw private context, raw provider body, page URL, tab/frame/document metadata, source identity, or DOM structure. Synthetic domain-neutral fixtures carry no private text.

## 10. Deferred engineering

The Deferred set is native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, broader editor support, packaging, signing, store publication, release automation, native placeholders, commercial infrastructure, and generalized cross-OS claims. Their future objectives introduce their own present-purpose architecture and verification.
