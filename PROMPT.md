# Build Emenda V0.1

> **Frozen clean-room build objective, version 2.0.0**

Build Emenda from this documentation package. Read the complete package before implementation and treat it as the product constitution.

## Objective

Deliver one OS-agnostic strict-TypeScript product core and one active V0.1 runtime: an unpacked Chromium Manifest V3 extension.

Emenda is a personal writing assistant that:

- observes committed changes in explicitly enabled browser text surfaces;
- reserves revision authority immediately and checks only after typing settles;
- selects a deterministic, bounded linguistic context;
- delegates linguistic judgment to a user-configured structured-output model through OpenRouter;
- validates every response locally and strictly;
- presents at most one exact correction;
- changes text only after explicit writer approval and complete current-state verification;
- preserves the writer's meaning, terminology, register, rhythm, and Duktus.

The active product loop is:

```text
eligible committed input
→ reserve RevisionId immediately
→ 600 ms trailing-edge debounce
→ capture SurfaceSnapshot
→ derive bounded TextContext and focus
→ check through InferenceProvider
→ validate corrections: [] | [Correction]
├─ [] → Idle
└─ [Correction] → Suggestion
   → Apply(SuggestionId) or Dismiss
   → verified undo-aware replacement or no edit
   → Idle
```

## Hard architecture invariant

The product core is strict TypeScript with no DOM, Chrome, Node, React, or extension types. Browser mechanisms exist only in `extension/` adapters and composition code. Source identity and snapshot identity remain opaque. Raw DOM data never leaves the content script.

The only active V0.1 composition is:

```text
strict-TypeScript core
+ BrowserTextSurface
+ content-script controller and overlay
+ MV3 service worker
+ options page
+ OpenRouterProvider
```

## Locked semantic ports

```ts
interface TextSurface {
  observe(sink: (signal: SurfaceSignal) => void): Disposable;
  capture(change: ObservedChange): Promise<SurfaceSnapshot>;
  replaceIfCurrent(request: ReplacementRequest): Promise<ReplacementResult>;
}

interface InferenceProvider {
  check(request: CheckRequest): PendingCheck;
}

type PendingCheck = {
  result: Promise<CheckResult>;
  cancel(): void;
};
```

`SurfaceSnapshot` contains opaque source and snapshot references, logical text, and a focus range. It contains no DOM reference. Apply accepts only `SuggestionId`; the controller owns all source, snapshot, revision, and correction authority.

## Locked product limits

- `DEBOUNCE_MS = 600`, trailing-edge only.
- `MAX_CONTEXT_SCALARS = 1200`, measured in Unicode scalar values.
- Half-open scalar ranges.
- One provider request for each current eligible controller revision.
- One response containing zero or one correction.
- Eight-second timeout and 32 KiB response-body limit.
- No streaming, retry, response healing, fallback model, persistent text cache, telemetry, or analytics.
- No compiled model default. The user configures a concrete structured-output model.
- Supported profiles: `auto`, `de-CH`, `en-GB`, `en-US`, `fr-FR`, `ka-GE`, `ru-RU`, and fail-closed `unsupported`.
- No translation.
- State: `Idle | Debouncing | Checking | Suggestion | Applying | Error`.

The full behavioral contract is defined in [`SPEC.md`](SPEC.md).

## Canonical implementation sequence

Use this exact sequence without reordering or parallel implementation tracks:

```text
Documentation baseline + Documentation Gate
→ strict-TypeScript domain and schemas
→ TextSurface + MockTextSurface
→ InferenceProvider + MockInferenceProvider
→ controller, scheduler, context, and revision
→ validator + presentation state
→ complete mock product + Mock Product Gate
→ Architecture Gate
→ BrowserTextSurface
→ MV3 worker, options, and overlay
→ OpenRouterProvider + Provider Gate
→ textarea runtime
→ conventional contenteditable runtime
→ Browser Integration + V0.1 Conformance Gate
→ stop
```

Active gates are:

```text
Documentation
→ Mock Product
→ Architecture
→ Provider
→ Browser Integration
→ V0.1 Conformance
```

Presentation and accessibility evidence belongs to Browser Integration.

## Execution discipline

For each independently verifiable increment:

```text
inspect
→ implement one invariant
→ run the smallest relevant verification
→ inspect the diff
→ record factual evidence
→ commit
→ push
→ verify the pushed state
→ continue
```

Initialize [`docs/EVIDENCE.md`](docs/EVIDENCE.md) only after the Documentation Gate passes. The evidence ledger is mutable and non-constitutional. The other immutable documents remain frozen; a material change requires a newly versioned constitution.

## V0.1 scope

Positively support only explicitly enabled top-level HTTP(S) pages containing a visible, focused, writable light-DOM `<textarea>` or a simple `contenteditable="true"` or `contenteditable="plaintext-only"` whose logical text and correction range map losslessly.

Inputs, iframes, shadow DOM, rich, virtualized, or canvas editors, Google Docs-style surfaces, restricted pages, file URLs, PDFs, readonly or disabled surfaces, and incognito are outside V0.1.

## Explicitly deferred

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential stores, native packaging, signing, Chrome Web Store publication, release automation, native placeholders, and cross-OS runtime claims are deferred to separately versioned objectives. Native work begins only after real browser usage demonstrates a material unmet need.

## Completion and stop rule

Complete every criterion in [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md). V0.1 ends after the Browser Integration and V0.1 Conformance gates pass, the unpacked extension is smoked on current Chrome Stable, evidence is recorded, the final commit is pushed and verified, and the worktree is clean.

Then stop. Distribution and all deferred work require a new explicit objective.
