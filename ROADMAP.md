# Emenda Roadmap

> **Frozen product roadmap, version 2.0.2**

## 1. Roadmap boundary

The current objective ends with the verified, hashed, committed, and pushed v2.0.2 documentation freeze. It does not authorize product implementation. A separate future objective must identify the implementation repository, baseline, branch, and pull-request target before building the smallest complete browser V0.1 through the ordered increments below.

V0.1 proves one correction loop before any expansion. Gates are evidence checkpoints, not releases or parallel work streams.

## 2. Canonical sequence

```text
Documentation baseline + Documentation Gate
→ Increment 1: Pure Core & Simulation
→ Increment 2: Unified State Machine
→ Increment 3: Mock Product + Architecture Gates
→ Increment 4: Unified DOM Integration
→ Increment 5: MV3 Shell + Provider
→ Increment 6: Browser Integration
→ Increment 7: V0.1 Conformance
→ stop
```

The Documentation Gate is the prerequisite baseline, not an eighth implementation increment.

## 3. Documentation baseline

The v2.0.2 baseline consists of:

- exactly 13 Markdown files frozen as `emenda-clean-room-v2.0.2-2026-08-16`;
- one documentation-only child commit of v2.0.1 at `d70b277998a23663ee6befc77dd6bb0da50ebcca`, with v2.0.0 preserved at `a1a13607867db8e6eb2ea904f6387ba130f22ce7`;
- 11 immutable documents verified by individual staged-Git-blob SHA-256 values;
- an empty mutable evidence-ledger template;
- no implementation source or unsupported product claim.

After the freeze commit is pushed, its remote identity and clean worktree are verified, and the Documentation Gate passes, the current objective stops.

## 4. Seven implementation increments

1. **Pure Core & Simulation** — establish the strict-TypeScript package, domain and model boundaries, deterministic text policy, semantic ports, scheduler seam, and simulations.
2. **Unified State Machine** — implement the reducer/effect architecture, revisions, settings authority, debounce, context, validation, presentation, Apply, Dismiss, and errors.
3. **Mock Product + Architecture Gates** — prove the full loop with simulations and fake clocks, pass the Mock Product Gate, then separately prove the Architecture Gate.
4. **Unified DOM Integration** — implement one safe browser surface for textarea and bounded contenteditable, including deterministic mapping, IME handling, self-mutation consumption, and one-step Undo.
5. **MV3 Shell + Provider** — add the Chrome 140 shell, trusted settings, origin lifecycle, protocol, options, overlay, corrected-focus provider contract, and OpenRouter adapter, then pass the Provider Gate.
6. **Browser Integration** — integrate both supported surface classes and verify permissions, storage isolation, teardown, accessibility, authority races, and editing behavior before passing the Browser Integration Gate.
7. **V0.1 Conformance** — run final audits and compatibility evidence, pass the V0.1 Conformance Gate, push and verify the final implementation, and stop.

The six gates are therefore placed as follows:

| Gate | Checkpoint |
| --- | --- |
| Documentation | Before Increment 1 |
| Mock Product | During Increment 3, before Architecture |
| Architecture | At the end of Increment 3 |
| Provider | At the end of Increment 5 |
| Browser Integration | At the end of Increment 6 |
| V0.1 Conformance | At the end of Increment 7 |

The Provider Gate requires all deterministic checks to pass and one complete sequential live qualification whose factual results remain visible. [`docs/ACCEPTANCE.md`](docs/ACCEPTANCE.md) owns the corpus and evidence requirements; the live run is not a reliability guarantee.

## 5. V0.1 evidence endpoint

Completion requires all earlier gate evidence to remain current and three distinct browser layers:

1. automated extension tests in Playwright's bundled Chromium persistent context;
2. minimum-runtime compatibility on Chromium or Chrome for Testing 140;
3. manual unpacked-extension smoke in current Chrome Stable with the real toolbar permission prompt.

Personal-use evidence records these environments separately:

```text
Windows Studio + current Chrome
MacBook + current Chrome
Chromebook + current ChromeOS/Chrome
```

Each entry reports only what ran on that device. V0.1 is complete only after the final evidence and implementation commits are pushed and verified and the worktree is clean.

## 6. Deferred horizons

Evidence from real browser use may justify later objectives for broader editing surfaces, interaction refinements, release automation, or store publication.

Native hosts, Tauri, Rust, operating-system accessibility APIs, native credential vaults, native packaging and signing, native placeholders, and generalized cross-OS runtime claims remain deferred. They require a separately versioned objective and evidence of a material browser-first limitation.
