# Emenda Engineering Standard

> **Frozen engineering standard, version 2.0.3**

## 1. Purpose

This document defines the V0.1 toolchain, verification policy, evidence vocabulary, and change discipline. Product behavior belongs to [`SPEC.md`](../SPEC.md), architectural ownership belongs to [`ARCHITECTURE.md`](ARCHITECTURE.md), and build order belongs to [`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md).

The current objective is the v2.0.3 documentation freeze, created as one documentation-only direct child of v2.0.2 commit `6a4ddc65fa9067f94023f87aebe48840e1b88bc2`. Implementation begins only under a separate future objective.

## 2. Toolchain and dependency policy

Use one npm package, strict TypeScript, exact direct dependency versions, and a committed npm lockfile. Reproducible verification begins with `npm ci` from a clean checkout and records the Node and npm versions.

The only direct runtime dependency is Zod. Development dependencies are limited to TypeScript, esbuild, Vitest, Playwright, Chrome types, and Node types. Plain TypeScript, HTML, and CSS implement extension UI.

Do not add React, Vite, Tailwind, extension frameworks, OpenRouter SDKs, monorepo tooling, backends, databases, or code generation. Every dependency, permission, abstraction, and build output must serve a current V0.1 requirement.

Enable strictness checks for unchecked indexed access, exact optional properties, overrides, and fallthrough. Use immutable values and exhaustive discriminated unions. Parse external data through the permitted strict Zod boundaries; do not repair or guess invalid data.

## 3. Determinism and boundary discipline

- Keep policy functions and the reducer pure; drive timers with a minimal fake-clock-compatible scheduler seam.
- Treat revision equality as authority. Cancellation saves work but never establishes correctness.
- Check authority again at every asynchronous completion before state, presentation, or text can change.
- Use Unicode scalar offsets in product logic and explicit lossless conversion at browser boundaries.
- Validate messages, trusted settings, provider responses, DOM capture, range mapping, and pre-mutation state at their owning boundaries.
- Refuse ambiguity. Do not normalize, relocate, widen, retry, heal, or recover by fuzzy or unique matching.
- Keep the undo-aware mutation leaf isolated and prove its behavior in real Chromium.

## 4. Verification layers

### Static and compiled verification

Inspect dependency, import, schema, manifest, permission, and bundle boundaries. Compile the core under a configuration without DOM, Chrome, Node, React, or extension ambient types, and compile the extension under its browser configuration.

### Deterministic verification

Use Vitest, fake clocks, deterministic surface and provider simulations, and controlled fetch/message doubles. Cover Unicode ranges, exact caret ownership, trusted `beforeinput`/`input` provenance tickets, context selection, reducer transitions, configuration races, cancellation order, stale work, foreground, exposure, and selection authority, validation, redaction, IME commitment, Apply, Dismiss, self-authored selection and mutation, and refusal.

Tests assert observable contracts rather than private helper structure. Fixtures are synthetic and domain-neutral. Timing tests control exact boundaries and completion order; arbitrary waits are not acceptable fixes for flaky tests.

Every deterministic gate assertion must pass; live-provider qualification is recorded separately and does not lower that standard.

### Provider verification

Prove the canonical serialization, authored headers, redirect/credential/cache/referrer controls, prompt, routing, structured-output schema, explicit web/healing/compression/fusion plugin disabling, reasoning-trace exclusion, `max_completion_tokens: 8192`, JSON media type, fatal UTF-8, exact returned model identity, outer response projection, timeout, incremental body limit, cancellation, local derivation, and redacted error conversion with controlled doubles. Deterministic Provider Gate tests require 100% success.

Run the canonical 15-case corpus from [`ACCEPTANCE.md`](ACCEPTANCE.md) strictly sequentially through the production validation path with one configured documented direct model. Do not retry or replace a case within a run. Record the requested model once, then `case`, selected model or `unavailable`, complete request latency, outcome, failure reason when any, and linguistic correctness. The Provider Gate requires `15/15`; a failed run remains evidence and only a complete later run after an implementation, configuration, or external-service change may establish recovery. The live run qualifies observed behavior without claiming a future reliability guarantee.

### Browser verification

Use three distinct layers:

1. Automated extension tests in Playwright's bundled Chromium persistent context, following its [extension-testing guidance](https://playwright.dev/docs/chrome-extensions).
2. A direct minimum-runtime compatibility test on Chromium or Chrome for Testing 140.
3. A manual unpacked-extension smoke in current Chrome Stable, including the actual toolbar permission prompt.

Browser verification uses the production unpacked build and covers the supported textarea and refused editor classes, paired-input provenance, conservative midpoint exposure, foreground/selection invalidation, storage isolation, synchronous worker listeners and Chrome-140 response bridging, explicit-port permissions, sender validation, external permission changes, serialized activation and revocation, worker restart, BFCache and prerender lifecycle, navigation races, text-only rendering, trusted approval controls, immediate pre-Apply authorization, scoped target selection, mutation failures, overlay accessibility, IME, and exact one-step Undo. Evidence states that DOM hit-testing cannot prove compositor-only or `pointer-events: none` visual occlusion of either the textarea or inline controls, and that page work nested in a genuine trusted `beforeinput` or queued ahead of its expiry callback can consume the one-use ticket inside the explicitly enabled-origin trust boundary.

Record Windows Studio with current Chrome, MacBook with current Chrome, and Chromebook with current ChromeOS/Chrome as separate personal-device results. Do not infer an untested cross-OS support claim from any one result.

## 5. Cross-platform audit capability

The future implementation provides one cross-platform audit command that orchestrates the checks applicable to the current phase. It makes clean `npm ci` installation, documentation validation, individual checksum verification, compilation, tests, build inspection, and final audits reachable through one entry point as those capabilities exist. Its filename, path, and internal script structure are builder choices.

The audit must be read-only with respect to constitutional and implementation sources. It retains the 11 independent staged-Git-blob SHA-256 checks rather than introducing an aggregate digest. Internal helpers, presentation, and exact command output remain implementation choices; do not create parallel audit scripts.

## 6. Evidence policy

Use these evidence levels precisely:

- `inspected`: static source, diff, configuration, or artifact inspection;
- `compiled`: compiler or build completion;
- `deterministic`: controlled automated behavior;
- `integration`: automated persistent-Chromium extension behavior;
- `live`: real OpenRouter behavior;
- `runtime`: minimum-version, current-Stable, or named-device smoke.

An evidence entry records:

```text
UTC time
gate or increment
tested implementation tree
tested implementation commit
commands or actions
exact result
environment
evidence level
limitations
next checkpoint
```

A later evidence commit records an implementation commit that already exists and was actually tested. Preserve failures and later recoveries as separate facts. State what was inspected only, what was not tested, and where claims are environment-specific.

Never record API keys, authorization headers, raw private context, raw model bodies, page URLs, tab/frame/document metadata, source identities, or DOM structure. Raw private text never enters logs. Fixtures, snapshots, errors, evidence, and commits use synthetic domain-neutral text and redacted metadata.

## 7. Change discipline

Group work into coherent, independently verifiable decisions. Run focused checks before broad checks, inspect each diff, and append only factual evidence. There is no required commit for every component or internal refactor.

The constitution fixes observable behavior, safety, privacy, compatibility, reliability requirements, and deterministic outcomes. Builders retain discretion over algorithms, storage layout, helper structure, pacing code, test organization, and other internal choices that satisfy those contracts; prefer the simplest clear implementation.

The immutable constitution changes only through a newly versioned documentation objective with new checksums. The evidence ledger cannot change product behavior, architecture, acceptance, UX, brand, or governance.

## 8. Deferred engineering

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, contenteditable and broader editor support, packaging, signing, Chrome Web Store publication, release automation, native placeholders, and generalized cross-OS claims are outside V0.1. Do not scaffold them.
