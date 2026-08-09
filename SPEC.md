# Emenda V0.1

Build **Emenda**, a minimal writing assistant that improves text while preserving the author's wording, voice, rhythm, register and Duktus.

Build the complete working V0.1 through a sequence of small, verified vertical increments.

Work autonomously. Use the decision matrix and design rationale to resolve ordinary implementation details. Prioritise the complete correction loop first, then polish the implementation around that working loop.

---

# 1. Product Goal

Emenda runs locally as a lightweight application and keeps the writer's original application as the primary writing surface.

For the current V0.1 desktop workflow, the user selects text in another application, triggers Emenda with a global hotkey, receives concise corrections from OpenRouter, reviews them in a small suggestion interface, and applies selected corrections directly back to the original text.

The application handles locally:

- text capture and replacement
- application state
- snapshots and revisions
- API credentials
- response validation
- language preferences
- model selection
- correction presentation

OpenRouter provides the linguistic intelligence.

The central product principle is:

> **Emenda corrects the text while preserving the author's Duktus.**

Emenda treats the author's existing text as the source of truth.

The UI therefore centres on **correction and refinement**, with every suggested change attributable to a specific part of the submitted text.

### V0.1 audience

V0.1 is Emenda's **personal and developer-validation milestone**. It is complete when the full correction loop is repeatedly runtime-verified on Windows for the owner.

Public beta readiness is a separate release gate covering distribution trust, signing, packaging quality, update delivery and broader compatibility.

---

# 2. Design Rationale

Emenda is intentionally a thin local application around a capable AI API.

The linguistic problem belongs to the model. The application concentrates on the parts that benefit from deterministic software engineering:

- capturing the correct text
- preserving text identity
- tracking asynchronous state
- validating model output
- presenting individual suggestions
- applying edits to the intended source
- protecting credentials
- preserving predictable application behaviour

This separation keeps the application compact while giving each layer a clear responsibility.

V0.1 implements **one complete vertical slice**:

```text
select text
→ invoke Emenda
→ capture selection
→ create snapshot
→ call OpenRouter
→ validate corrections
→ review suggestions
→ apply correction
→ continue writing
```

Completing this loop first gives every later feature a reliable foundation.

The architecture is optimised for one developer working heavily with an AI coding agent. The repository should remain small enough to understand as a whole, strongly typed enough to catch generated-code mistakes early, and modular enough that each major responsibility can be tested independently.

The selected technology stack itself provides most of the engineering guardrails:

```text
Rust compiler
+ strong Rust types
+ strict TypeScript
+ Zod at frontend runtime boundaries
+ Serde at Rust data boundaries
+ Tauri capability boundaries for desktop surfaces
+ OpenRouter structured outputs
+ small modules
+ tests around the vertical slice
```

---

# 3. Decision Matrix

Use this matrix as the decision function for the implementation.

When a small implementation question remains open, prefer the choice that reinforces the selected direction.

| Decision | Selected approach | Other reasonable direction | Why Emenda chooses this | V0.1 implication |
|---|---|---|---|---|
| Product form | **Cross-platform writing layer, desktop first** | Browser extension first | The main product hypothesis is writing assistance that follows the writer across applications. Desktop validation removes more uncertainty than browser-only validation. | Prove the desktop loop on Windows first while keeping shared contracts platform-neutral. |
| Trigger | **Explicit selection + global hotkey** | Passive observation while typing | Selection supplies a clear text scope and hotkey invocation supplies a deterministic moment for inference. | One deliberate user action starts each V0.1 correction request. |
| Desktop text transport | **Selected-text capture/replacement through the simplest reliable platform mechanism** | Full accessibility observation from day one | V0.1 needs to prove the cross-app loop before expanding text-surface sophistication. | Native mechanics remain inside the platform adapter. |
| Application architecture | **Tauri modular monolith for desktop** | Web backend, Electron service architecture | Emenda is one local product with one privileged core and one product UI. | One desktop application, one runtime, small internal boundaries. |
| Privileged core | **Safe Rust** | TypeScript-only system layer | OS interaction, secure storage and text replacement benefit from compiler-enforced correctness. | Rust owns privileged desktop operations. |
| Product UI | **React + strict TypeScript** | Rust-native UI | Settings and suggestion UI benefit from fast iteration while keeping strong static checks. | React owns presentation and user interaction. |
| Linguistic intelligence | **OpenRouter** | Local model in V0.1 | OpenRouter provides immediate access to many capable models through one API. | One API integration powers all languages. |
| Default model | **`openrouter/free`** | Hardcoded individual model | Free model availability changes over time. The router keeps the default current while the model picker gives the user control. | V0.1 works immediately after API-key configuration when the selected route satisfies Emenda's contract. |
| Provider architecture | **One tiny `InferenceProvider` seam** | Separate model/vendor service architecture | Model choice is configuration. Provider replacement is one useful future seam. | Keep one provider implementation in V0.1: OpenRouter. |
| Correction representation | **Individual structured corrections** | Return one rewritten paragraph | Exact corrections preserve authorship and support reviewable, attributable edits. | Each correction has range, original, replacement, category and confidence. |
| Async state | **Immutable snapshot + authoritative latest revision** | Queue every request | A new invocation represents the user's newest intent. | Every new request supersedes previous pending revisions. |
| Source identity | **Opaque adapter-owned reference + UI-safe display summary** | Shared process/window identifiers | Source identity differs fundamentally between native applications and browser surfaces. | Shared workflow stores identity but never interprets platform-specific identifiers. |
| Source replacement | **Adapter-owned `replace_if_unchanged` semantic operation** | Shared focus/revalidate/paste sequence | Focus, source revalidation and replacement are platform strategies. | Shared workflow asks for the invariant; each adapter proves and performs it. |
| Language handling | **AI-assisted detection inside the same correction request, guided by local defaults** | Separate language-detection service | The model already possesses the linguistic capability and six concise profiles fit naturally into the prompt. | One inference request detects language and produces corrections. |
| German default | **Swiss Standard German** | Generic German detection | Emenda's German profile is deliberately Swiss. | German text follows `de-CH`, including `ss`. |
| English default | **British English** | Generic English | `en-GB` is the default profile while strong American usage can select `en-US`. | The model preserves clearly American English. |
| Model output | **Strict JSON Schema structured output where the selected route supports it** | Free-form prose parsing | The application needs predictable machine-readable data. | Validate every response before it reaches correction state. |
| V0.1 testing | **Windows: one simple native editor + one additional common desktop application** | Large compatibility matrix | Two successful real surfaces prove the first native adapter while keeping implementation focused. | Compatibility breadth becomes a later milestone. |
| Error UX | **Explicit typed states** | Generic failure message | Distinct failures produce clearer debugging and safer behaviour. | Connection, schema, stale-state and replacement errors remain distinguishable. |
| Future extensibility | **Small stable seams** | Prebuilding future subsystems | The cleanest future architecture grows from proven requirements. | Create only seams required by the current workflow or an established cross-platform boundary. |

---

# 4. Why V0.1 Uses Selection and a Hotkey

Selected-text correction gives Emenda:

- an explicit text range
- clear user intent
- a deterministic inference moment
- a bounded correction target
- a simple initial cross-application interaction model

This lets V0.1 validate the full desktop hypothesis with a small amount of code.

The long-term product expands toward:

```text
passive observation
→ automatic changed-sentence detection
→ inline suggestions
→ broader application adapters
```

V0.1 establishes the reliable correction loop that those features can reuse.

---

# 5. V0.1 User Flow

Implement this complete workflow.

1. Emenda launches as a lightweight Tauri desktop application.
2. The user opens Settings.
3. The user enters an OpenRouter API key.
4. Rust stores the API key through secure OS-appropriate credential storage.
5. The default model is `openrouter/free`.
6. The user can search and select another current OpenRouter model.
7. The user opens another desktop application.
8. The user selects text.
9. The user presses the Emenda global hotkey.
10. The native text adapter captures the selected text and creates an adapter-owned source reference plus a human-readable source summary.
11. Emenda creates an immutable `TextSnapshot`.
12. Emenda increments the current revision.
13. Emenda sends the snapshot text and language context to OpenRouter.
14. OpenRouter detects the appropriate supported language profile.
15. OpenRouter returns structured corrections.
16. Rust validates the structured response.
17. Emenda confirms that the result belongs to the current revision.
18. Emenda opens a compact suggestion window.
19. The user reviews individual corrections.
20. The user chooses **Apply** on a correction.
21. Emenda updates the corrected working text deterministically.
22. When source replacement is requested, the shared workflow asks the adapter to replace the captured selection only if the adapter can verify that the original source and expected text remain authoritative.
23. The adapter performs its platform-specific verification and one coherent replacement operation.
24. The user continues writing.

---

# 6. Text Capture and Replacement

The shared workflow expresses **semantic invariants**. Native desktop adapters own native source identity, focusing, selection revalidation and replacement mechanics.

Use platform-neutral shared types conceptually equivalent to:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceReference {
    /// Interpreted only by the adapter that created it.
    opaque_id: String,

    /// Safe, human-readable information for UI presentation.
    display: SourceDisplay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDisplay {
    pub application_name: String,
    pub context_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedSelection {
    pub text: String,
    pub source: SourceReference,
}
```

The native desktop contract should be conceptually equivalent to:

```rust
pub trait TextSurfaceAdapter: Send + Sync {
    fn capture_selection(
        &self,
    ) -> Result<CapturedSelection, TextSurfaceError>;

    /// Replace only while the adapter can verify that the original source
    /// and selected text are still authoritative.
    fn replace_if_unchanged(
        &self,
        source: &SourceReference,
        expected_text: &str,
        replacement: &str,
    ) -> Result<(), TextSurfaceError>;
}
```

The important boundary is responsibility:

```text
Shared workflow owns:
revision current?
replacement validated?
user accepted it?

Adapter owns:
how source identity works
how focus works
how selection is revalidated
how replacement is performed
clipboard preservation when relevant
protected/elevated-surface handling
```

Shared Rust modules must never compare process IDs, executable paths, native window handles, browser tab identifiers, DOM references or other adapter-specific source identifiers.

The frontend may receive `SourceDisplay`. It must not receive or interpret native source identity.

### Windows V0.1

The current Windows adapter may use clipboard-assisted capture/replacement, source-window tracking and Windows-specific verification internally. Preserve its proven fail-closed behaviour behind the adapter boundary.

### Browser semantic equivalent

The browser extension is not a Rust `target_os` adapter. It implements the **same semantic contract in TypeScript** using browser-native concepts such as tab, frame, editable element and selection state.

Shared desktop and browser semantics are:

```text
capture current text/source
→ retain opaque source identity inside the adapter
→ analyse against an immutable revision
→ replace only if the adapter can verify the original source/text remain authoritative
→ return typed errors
```

A Rust `browser.rs` stub is not required.

macOS and Linux adapter modules begin when their implementations begin, together with their contract tests and native verification.

---

# 7. Snapshot and Revision Model

Every correction request operates against an immutable snapshot.

Use a simple monotonically increasing revision identifier for the current application session.

Conceptually:

```rust
struct TextSnapshot {
    revision_id: u64,
    text: String,
    created_at: SystemTime,
    source: SourceReference,
}
```

The newest invocation becomes authoritative.

Example:

```text
revision 41 starts
revision 42 starts
revision 41 returns
revision 42 returns
```

Application behaviour:

```text
revision 41 result → stale
revision 42 result → current
```

The core rule is:

```text
result.revision_id == current_revision_id
```

A matching revision can enter current suggestion state.

A stale result becomes a typed stale-result state and leaves the current text untouched.

### Why snapshots are explicit

AI responses are asynchronous while the user continues working.

The snapshot binds every proposal to the exact text that produced it.

Revision identity therefore protects:

- text integrity
- suggestion relevance
- source application state
- replacement correctness

The AI produces proposals against an immutable snapshot. Emenda applies them only while that proposal still belongs to the active revision.

---

# 8. Correction Contract

Use one conceptual correction contract throughout Emenda.

```ts
type Correction = {
  start: number;
  end: number;
  original: string;
  replacement: string;
  category: "spelling" | "grammar" | "punctuation" | "style";
  confidence: "high" | "medium" | "low";
  explanation?: string;
};
```

Create the corresponding strongly typed Rust representation.

Define `start` and `end` as **Unicode scalar-value positions within the submitted snapshot text**.

Validate every correction against the original snapshot.

A valid applicable correction satisfies:

```text
range exists
+ original matches snapshot at that range
+ replacement is a string
+ category is valid
+ confidence is valid
```

When the returned range and `original` disagree, the core may resolve the correction when `original` occurs exactly once in the snapshot.

Ambiguous matches produce a typed non-applicable correction state.

### Why corrections are explicit

Individual correction objects preserve the relationship between:

```text
source text
↔ proposed change
↔ exact location
↔ explanation
```

This lets Emenda behave as an editor rather than a rewrite engine.

Exact, validated corrections also make application state testable and make later UI features such as inline underlines possible without changing the correction protocol.

---

# 9. Language Profiles

Support:

```text
de-CH
en-GB
en-US
fr-FR
ka-GE
ru-RU
```

Use `auto` as the default language mode.

Keep the profile instructions locally inside the repository.

Use the same OpenRouter request for language identification and correction.

Apply these defaults:

```text
German  → de-CH
English → en-GB
```

Preserve `en-US` when the text shows clearly American spelling or language conventions.

Recognise Georgian script as `ka-GE`.

Recognise Russian text as `ru-RU`.

French maps to `fr-FR`.

Expose a manual language override in Settings for cases where the user wants a fixed profile.

### Swiss Standard German

The `de-CH` profile should use:

- `ss`
- Swiss Standard German spelling
- Swiss vocabulary where relevant
- the author's existing level of formality
- existing names, legal terms, brands and domain vocabulary

### Multilingual text

Preserve quotations, names and short embedded passages in another language.

Choose the profile according to the dominant language of the selected passage while preserving embedded language fragments.

---

# 10. Emenda Linguistic System Prompt

Store the system prompt as an easily editable local resource.

Use this behavioural foundation:

```text
You are Emenda, a restrained multilingual writing editor.

Improve the submitted text through the smallest useful corrections.

Preserve the author's:
- meaning
- voice
- rhythm
- register
- sentence structure where it already works
- terminology
- names
- intentional informality
- language variety
- domain-specific vocabulary

Prioritise:
1. spelling
2. grammar
3. punctuation
4. clear word misuse
5. restrained stylistic improvement

Treat the submitted text as the source of truth.

A grammatically correct and effective formulation should remain unchanged.

Offer stylistic changes selectively when they clearly improve clarity, precision or readability while preserving the author's Duktus.

Identify the best matching supported language profile:
de-CH, en-GB, en-US, fr-FR, ka-GE, ru-RU.

Use de-CH for German.
Use en-GB as the default English profile.
Preserve clearly American English as en-US.

Return only data matching the provided structured-output schema.

Each correction must describe one specific change and include the exact original text.

Keep explanations concise and useful.
```

Append the relevant concise language-profile instructions to the request context.

---

# 11. Inference Architecture

Use one small provider boundary in the Rust core.

Conceptually:

```rust
trait InferenceProvider {
    async fn list_models(&self) -> Result<Vec<Model>, InferenceError>;
    async fn check_text(
        &self,
        request: CheckRequest,
    ) -> Result<CheckResult, InferenceError>;
    async fn health_check(&self) -> Result<(), InferenceError>;
}
```

Implement exactly one V0.1 provider:

```text
OpenRouterProvider
```

Use:

```text
GET  https://openrouter.ai/api/v1/models
POST https://openrouter.ai/api/v1/chat/completions
```

Default model:

```text
openrouter/free
```

Use OpenRouter structured outputs with a strict JSON Schema matching the Emenda correction response where the selected model/provider path supports the required parameters.

Keep production fail-closed when the returned content cannot satisfy the correction contract.

Keep model choice as configuration.

### Why one inference abstraction

OpenRouter already provides access to many models through one stable API.

Therefore:

```text
model choice = setting
provider choice = architecture seam
```

The provider boundary exists so a future OpenAI-compatible local endpoint can use the same correction workflow.

V0.1 gains the benefit of that seam through one interface and one implementation.

---

# 12. Frontend and Local Core

Use one Tauri desktop application with two internal layers:

```text
┌────────────────────────────────────────────┐
│                   Emenda                   │
│                                            │
│   FRONTEND              LOCAL CORE         │
│   React / TypeScript    Rust               │
│                                            │
│   settings          →   secure storage     │
│   model picker      →   OpenRouter         │
│   suggestions       ←   snapshots          │
│   apply action      →   workflow           │
│   status            ←   typed errors       │
│                                            │
└────────────────────────────────────────────┘
```

## Frontend responsibilities

Use React + strict TypeScript for:

- Settings
- model search and selection
- language preference
- suggestion presentation
- Apply actions
- visible application state
- error presentation
- displaying the UI-safe source summary

## Rust responsibilities

Use Rust for:

- global hotkey
- native adapter orchestration
- text capture
- text replacement
- revision state
- snapshot state
- OpenRouter communication
- structured-response deserialisation
- secure API-key storage
- Tauri commands
- privileged OS operations

### Why this architecture

Tauri keeps desktop Emenda as one local application while creating a clean boundary between product UI and privileged operating-system functionality.

Rust owns the operations where correctness, state discipline and system access matter most.

React and strict TypeScript keep the product surface fast to develop and easy to change.

Native implementation details remain concentrated behind the Rust text-surface boundary while correction semantics and product behaviour stay shared.

The browser extension implements equivalent semantics in TypeScript rather than becoming a Rust `target_os` branch.

---

# 13. Tauri Command Surface

Keep the frontend-to-core API small.

A sensible command surface is approximately:

```text
get_settings
save_settings
test_openrouter
list_models
check_current_selection
apply_correction
dismiss_suggestions
```

Grant desktop frontend windows only the Tauri capabilities required for their responsibilities.

Keep secure credentials, external API communication and native text operations inside the Rust core.

Browser-extension permissions are declared separately through the extension configuration.

---

# 14. Settings

Create one simple Settings view.

## OpenRouter API Key

Provide:

- secure input field
- save action
- connection test
- clear success/error state

Store the credential through secure OS-appropriate storage managed from Rust on desktop.

## AI Model

Default:

```text
Free automatically
openrouter/free
```

Load the current OpenRouter model catalogue programmatically.

Provide:

- search
- model name
- model ID
- selected state

Store the selected model locally.

## Language

Default:

```text
Automatic
```

Options:

```text
Automatic
Swiss Standard German
British English
American English
French
Georgian
Russian
```

## Hotkey

Display the active V0.1 global hotkey clearly.

A fixed default hotkey is sufficient for the first complete vertical slice.

---

# 15. Suggestion Window

Create a compact, visually quiet suggestion window.

For each correction show:

```text
original → replacement
category
short explanation
Apply
```

Use restrained visual hierarchy.

Useful application states:

```text
Checking…
Corrections found
Text looks good
Connection error
Invalid AI response
Stale result
Replacement error
```

After one correction is applied, update the local corrected snapshot so subsequent corrections refer to the new working text.

When the user commits the reviewed changes, ask the active text adapter to replace the original captured passage only if the adapter verifies that the source and expected source text are still authoritative.

---

# 16. Error Model

Use typed errors throughout the core.

Distinguish at least:

```text
ConfigurationError
AuthenticationError
NetworkError
InferenceError
StructuredOutputError
ValidationError
StaleRevisionError
TextCaptureError
TextReplacementError
ProtectedSurfaceError
Unsupported
```

Map each category to a concise user-facing state.

The correction workflow should always resolve into a clear state that the UI can display.

---

# 17. Engineering Guardrails

Use the stack as the primary correctness system.

## Rust

Emenda desktop product logic uses safe Rust.

At the crate root:

```rust
#![forbid(unsafe_code)]
```

Use:

- idiomatic Rust types
- explicit `Result` propagation
- narrow modules
- typed errors
- Serde for request/response structures
- deterministic state transitions

Run:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## TypeScript

Emenda frontend and browser product logic use strict TypeScript.

```json
{
  "strict": true
}
```

Use Zod for runtime validation of data crossing important frontend boundaries.

Run the project's TypeScript type checker as part of verification.

## Native build infrastructure

MSVC, the Windows SDK, Xcode command-line tools and Linux system packages are compilation infrastructure rather than additional Emenda application languages.

Adding another application language requires an explicit documented architecture decision.

## Tauri

Use Tauri 2 capabilities to define the authority of desktop frontend surfaces.

The Rust core owns privileged desktop functions.

## Repository quality

Keep the repository:

- compact
- readable
- typed
- testable
- easy to navigate
- easy for another coding agent to understand in one pass

Prefer direct code over framework layers whose value has not yet appeared in the product workflow.

---

# 18. Repository Structure

Keep one modular monolith and add platform modules when their real implementation begins.

A compact desktop structure may look like:

```text
emenda/
├── src/
│   ├── components/
│   ├── settings/
│   ├── suggestions/
│   ├── language/
│   ├── schemas/
│   └── types/
│
├── src-tauri/
│   └── src/
│       ├── text/
│       │   ├── mod.rs
│       │   └── windows.rs
│       ├── inference/
│       ├── snapshot/
│       ├── settings/
│       ├── error.rs
│       └── lib.rs
│
├── tests/
├── package.json
├── README.md
├── SPEC.md
├── AGENTS.md
├── UX.md
└── BRAND.md
```

Add `macos.rs` and `linux.rs` when implementation of those adapters begins. Empty stubs are not evidence of platform support.

The browser extension may live in its own application directory when implementation begins and should implement the same semantic contracts in TypeScript.

---

# 19. Minimal Test Strategy

The test suite follows the main sources of risk in the correction loop:

```text
external AI data
asynchronous state
text identity
adapter-owned source verification
source replacement
```

## Shared tests

Exercise correction and workflow logic through a mock text-surface adapter in every normal test run.

Use platform-neutral fixtures. Shared tests should not depend on process IDs, native window handles, executable paths, browser tab IDs or DOM references.

## Test 1: Structured correction parsing

Given:

```text
I liek this sentence.
```

Verify that a valid correction response becomes a typed `Correction`.

Verify the expected original and replacement.

## Test 2: Correction range validation

Verify that a correction refers to the intended substring of the immutable snapshot.

Include one Unicode example from `ka-GE` or `ru-RU`.

## Test 3: Stale revision

Create revision 41.

Create revision 42.

Return the result for revision 41.

Verify that revision 42 remains authoritative.

## Test 4: Invalid structured response

Pass malformed or schema-incompatible correction data.

Verify that Emenda produces a typed validation error.

## Test 5: OpenRouter integration

With a configured API key:

```text
health check
→ retrieve model catalogue
→ perform one structured correction request
```

Verify successful deserialisation or a correctly classified fail-closed contract failure.

## Test 6: Windows desktop text loop

Validate manually or through host-appropriate integration tooling:

```text
select text
→ invoke Emenda
→ receive suggestion
→ apply correction
→ adapter verifies source/text
→ corrected text appears in source
```

Run this in:

1. one simple native Windows text editor
2. one additional ordinary Windows desktop application with editable text

Record validated applications in the README.

### Cross-platform CI

CI should compile and run the strongest applicable shared checks on Windows, macOS and Linux runners.

Native GUI smoke tests remain platform-specific and opt-in where CI cannot reliably exercise real desktop surfaces.

### Why the test target is small

These tests cover the architecture's highest-risk boundaries while keeping V0.1 focused on one complete user outcome.

The Windows integrations prove the first native adapter. Shared mock tests prove that correction workflow semantics are independent from native windowing mechanisms.

---

# 20. Implementation Order

Implement through small, verified increments.

For each increment:

```text
inspect
→ implement one independently verifiable architectural decision or product invariant
→ run the smallest relevant checks
→ review the diff
→ commit with detailed rationale
→ push
→ verify the pushed state
→ continue
```

A realistic sequence for the current architecture is:

## Step 1: Verification foundation

Establish a reliable automated verification path for the shared repository on Windows, macOS and Linux hosts.

Keep real desktop smoke tests separate and platform-specific.

## Step 2: Core contracts

Keep the correction schema, inference contract, snapshot/revision semantics and platform-neutral text-surface contract deterministic and tested.

## Step 3: Provider implementation

Implement and verify `OpenRouterProvider` independently behind `InferenceProvider`.

## Step 4: Native adapter

Implement one native adapter behind the shared text-surface semantics and test its platform-specific behaviour.

Windows is the first verified native adapter.

## Step 5: Vertical slice

Connect:

```text
hotkey
→ capture
→ snapshot
→ OpenRouter
→ suggestions
→ apply
→ replace_if_unchanged
```

## Step 6: Settings and Suggestion UI

Complete:

```text
API key
model picker
language mode
connection state
suggestion cards
typed error states
```

## Step 7: Real application verification

Run the complete flow in the two V0.1 Windows integration targets.

## Step 8: Verification

Run the strongest applicable repository checks and report precisely what compiled, ran and passed.

---

# 21. V0.1 Scope

V0.1 consists of the personal/developer-validation path:

```text
Tauri desktop application
Rust local core
React + strict TypeScript UI
global hotkey
selected-text capture
OpenRouter
openrouter/free default
OpenRouter model picker
automatic language profile selection
manual language override
structured corrections
immutable snapshots
revision handling
suggestion review
adapter-owned safe source replacement
secure API-key storage
typed errors
Windows runtime verification in two desktop integration targets
```

The architecture leaves clean future capability milestones for:

```text
passive background observation
Grammarly-style inline suggestions
richer Windows accessibility integration
macOS adapter implementation
Linux adapter implementation
browser extension / ChromeOS implementation
personal dictionary
accepted/rejected correction history
local OpenAI-compatible inference
local LLMs
additional writing profiles
```

Each future capability can build on the same correction, inference, revision, error and adapter semantics.

---

# 22. Definition of Done

V0.1 is Emenda's **personal and developer-validation milestone**.

It is complete when this real workflow succeeds reliably for the owner on Windows:

```text
Launch Emenda
→ configure OpenRouter
→ select text in another desktop application
→ press the Emenda hotkey
→ capture the selected text
→ receive validated structured corrections
→ review suggestions
→ apply a correction
→ adapter verifies the original source/text
→ corrected text appears in the original application
→ continue writing
```

Verify that workflow repeatedly in:

```text
one simple native Windows text editor
+
one additional ordinary Windows desktop application
```

Also verify:

```text
stale response → recognised
invalid AI response → recognised
OpenRouter failure → recognised
text capture failure → recognised
text replacement failure → recognised
zero corrections → clear success state
```

Public beta readiness is a separate release gate.

### Platform terminology

Use these terms consistently:

```text
Architectural target
= intended platform represented by shared product contracts and design decisions

Compiles
= repository builds successfully on that host

Supported platform
= adapter implemented
+ shared platform-agnostic tests pass
+ platform-specific integration tests pass on that OS

Distribution-ready
= supported platform
+ packaging and platform trust requirements satisfied
```

Windows is currently the first supported and runtime-verified native adapter.

macOS, Linux and Browser/ChromeOS remain first-class architectural targets until their adapters and platform-specific integration tests exist.

The primary V0.1 success criterion is a **small, understandable and reliable complete correction loop**.

---

# 23. Product Identity

Use these identifiers consistently:

```text
Product / UI:          Emenda
GitHub repository:     emenda
CLI / binary:          emenda
Rust crate:            emenda
Tauri identifier:      ch.zbinden.emenda
```

Use:

```text
OPENROUTER_API_KEY
```

as the conventional development environment variable when an environment-based credential is useful.

Runtime user credentials belong in Emenda's secure local credential storage.

---

# 24. Platform Foundation

Emenda's shared product semantics are platform-independent.

Windows is the current development and runtime-verification environment. macOS, Linux, and Browser/ChromeOS are first-class architectural targets.

The shared product logic remains identical across platforms:

- correction workflow
- inference contract
- correction schema
- snapshots and revisions
- language profiles
- settings concepts
- application state semantics
- typed error meanings
- UX decision rules
- personalisation logic when introduced

## Desktop native contract

Desktop Emenda uses the Rust `TextSurfaceAdapter` semantic boundary:

```text
Rust TextSurfaceAdapter
├── Windows implementation
├── macOS implementation, when development begins
└── Linux implementation, when development begins
```

Native adapter modules own:

- native source identity
- focus mechanics
- selection/text revalidation
- replacement mechanics
- native permission/protection checks
- native clipboard handling where relevant

Shared Rust code depends only on generic source references, UI-safe source summaries, typed errors and semantic adapter operations.

## Browser and ChromeOS contract

The browser extension is a first-class Emenda surface and the primary ChromeOS path.

It implements the same **semantic text-surface contract in TypeScript**, rather than as a Rust `target_os` module.

Desktop and browser share:

- correction schema
- inference contract
- revision semantics
- language profiles
- settings concepts
- typed error meanings
- UX decision rules

The browser implementation may retain an opaque adapter-owned token referring to tab, frame, element and selection state. Shared UI/product logic must not parse that representation.

Desktop and browser releases may proceed on independent release cadences while preserving shared semantics.

## Platform boundaries

Keep platform-specific types, APIs, constants, capabilities, identifiers and implementation details inside their respective native or browser adapter modules.

Expose generic shared types and semantic contracts to the rest of Emenda.

Shared modules must not interpret:

```text
process IDs
native window handles
executable paths
browser tab IDs
frame IDs
DOM node references
selection handles
other platform-specific source identity
```

## Adapter semantics

Every text-surface implementation provides equivalent outcomes:

```text
capture current text and source
retain source identity privately
report user-safe source information
replace only if original source/text remain authoritative
return typed failures
```

Represent unavailable functionality through typed `Unsupported` results.

Provide a mock adapter that supports the complete shared workflow:

```text
detect
→ indicate
→ suggest
→ apply
```

This keeps correction and workflow logic independently testable from native accessibility, windowing and browser APIs.

## Testing

Test shared logic through mock adapters in every normal CI run.

Keep native verification inside platform-specific tests.

Examples:

```text
Windows → Notepad / VS Code / supported Windows surfaces
macOS   → host-appropriate native editors
Linux   → host-appropriate native editors
Browser → browser integration fixtures
```

CI compiles and runs the strongest applicable shared suite on Windows, macOS and Linux hosts.

A passing cross-platform build establishes compilation evidence. It does not by itself establish runtime platform support.

## Capabilities and permissions

Declare each desktop adapter's required native and Tauri capabilities explicitly.

Declare browser-extension permissions through the browser extension configuration.

Shared application state responds to adapter outcomes through the common typed semantic interface.

Introduce a richer capability model only when the UI must choose between materially different interaction modes such as selected-text correction, ambient observation, inline anchoring, read-only suggestions or copy fallback.

## Packaging

Treat installers, code signing, notarization, package formats, store distribution and publisher reputation as deployment concerns.

Keep packaging configuration outside the shared correction, inference, state and text-surface architecture.

## Platform decision rule

When an implementation choice remains open:

> **Keep shared product behaviour platform-independent and place native operating-system or browser behaviour behind the smallest appropriate adapter boundary.**

---

# 25. Final Engineering Principle

Use this principle when an implementation choice remains ambiguous:

> **Emenda delegates linguistic judgment to the AI model and keeps context, state, validation, security and text replacement deterministic in the local application.**

And use this product principle when a linguistic or UX choice remains ambiguous:

> **Emenda corrects the text while preserving the author's Duktus.**

Build the smallest coherent application that fully proves those two principles.

Complete the current vertical increment, run the relevant checks, fix the failures, commit and push the verified decision, then continue to the next increment.
