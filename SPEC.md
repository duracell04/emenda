# Emenda V0.1

Build **Emenda**, a minimal desktop writing assistant that improves selected text while preserving the author's wording, voice, rhythm, register and Duktus.

Build the complete working V0.1 application from this specification in one implementation pass.

Work autonomously. Use the decision matrix and design rationale to resolve ordinary implementation details. Prioritise the complete correction loop first, then polish the implementation around that working loop.

---

# 1. Product Goal

Emenda runs locally as a lightweight Tauri desktop application.

The user selects text in another desktop application, triggers Emenda with a global hotkey, receives concise corrections from OpenRouter, reviews them in a small suggestion interface, and applies selected corrections directly back to the original text.

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
+ Tauri capability boundaries
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
| Product form | **Desktop writing layer** | Browser extension first | The main product hypothesis is cross-application writing assistance. Desktop validation removes more uncertainty than browser-only validation. | Build Tauri desktop first. |
| Trigger | **Explicit selection + global hotkey** | Passive observation while typing | Selection supplies a clear text scope and hotkey invocation supplies a deterministic moment for inference. | One deliberate user action starts each correction request. |
| Desktop text transport | **Selected-text capture/replacement through the simplest reliable platform mechanism, with clipboard-assisted transport as the initial general path** | Full accessibility observation from day one | V0.1 needs to prove the cross-app loop before expanding text-surface sophistication. | Encapsulate capture/replacement behind one small adapter boundary. |
| Application architecture | **Tauri modular monolith** | Web backend, Electron service architecture | Emenda is one local product with one privileged core and one product UI. | One repository, one application, one runtime. |
| Privileged core | **Rust** | TypeScript-only system layer | OS interaction, secure storage and text replacement benefit from compiler-enforced correctness. | Rust owns privileged operations. |
| Product UI | **React + strict TypeScript** | Rust-native UI | Settings and suggestion UI benefit from fast iteration while keeping strong static checks. | React owns presentation and user interaction. |
| Linguistic intelligence | **OpenRouter** | Local model in V0.1 | OpenRouter provides immediate access to many capable models through one API. | One API integration powers all languages. |
| Default model | **`openrouter/free`** | Hardcoded individual model | Free model availability changes over time. The router keeps the default current while the model picker gives the user control. | V0.1 works immediately after API-key configuration. |
| Provider architecture | **One tiny `InferenceProvider` seam** | Separate model/vendor service architecture | Model choice is configuration. Provider replacement is one useful future seam. | Implement exactly one provider: OpenRouter. |
| Correction representation | **Individual structured corrections** | Return one rewritten paragraph | Exact corrections preserve authorship and support reviewable, attributable edits. | Each correction has range, original, replacement, category and confidence. |
| Async state | **Immutable snapshot + authoritative latest revision** | Queue every request | A new invocation represents the user's newest intent. | Every new request supersedes previous pending revisions. |
| Language handling | **AI-assisted detection inside the same correction request, guided by local defaults** | Separate language-detection service | The model already possesses the linguistic capability and six concise profiles fit naturally into the prompt. | One inference request detects language and produces corrections. |
| German default | **Swiss Standard German** | Generic German detection | Emenda's German profile is deliberately Swiss. | German text follows `de-CH`, including `ss`. |
| English default | **British English** | Generic English | `en-GB` is the default profile while strong American usage can select `en-US`. | The model preserves clearly American English. |
| Model output | **Strict JSON Schema structured output** | Free-form JSON prompting | The application needs predictable data, rather than prose parsing. | Validate every response before it reaches correction state. |
| V0.1 testing | **One simple native editor + one additional common desktop application** | Large compatibility matrix | Two successful real surfaces prove the architecture while keeping implementation focused. | Compatibility breadth becomes a later iteration. |
| Error UX | **Explicit typed states** | Generic failure message | Distinct failures produce clearer debugging and safer behaviour. | Connection, schema, stale-state and replacement errors remain distinguishable. |
| Future extensibility | **Small stable seams** | Prebuilding future subsystems | The cleanest future architecture grows from proven requirements. | Create only the seams required by V0.1 plus the obvious provider/text-surface boundaries. |

---

# 4. Why V0.1 Uses Selection and a Hotkey

Selected-text correction gives Emenda:

- an explicit text range
- clear user intent
- a deterministic inference moment
- a bounded correction target
- a simple initial cross-application interaction model

This lets V0.1 validate the full desktop hypothesis with a small amount of code.

The long-term product can later expand toward:

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

1. Emenda launches as a lightweight Tauri application.
2. The user opens Settings.
3. The user enters an OpenRouter API key.
4. Rust stores the API key through secure OS-appropriate credential storage.
5. The default model is `openrouter/free`.
6. The user can search and select another current OpenRouter model.
7. The user opens another desktop application.
8. The user selects text.
9. The user presses the Emenda global hotkey.
10. Emenda records the source application.
11. Emenda captures the selected text.
12. Emenda creates an immutable `TextSnapshot`.
13. Emenda increments the current revision.
14. Emenda sends the snapshot to OpenRouter.
15. OpenRouter detects the appropriate supported language profile.
16. OpenRouter returns structured corrections.
17. Rust validates the structured response.
18. Emenda confirms that the result belongs to the current revision.
19. Emenda opens a compact suggestion window.
20. The user reviews individual corrections.
21. The user chooses **Apply** on a correction.
22. Emenda updates the snapshot text deterministically.
23. Emenda returns focus to the source application.
24. Emenda replaces the original selected text with the updated text.
25. The source application's native undo history remains useful through a single replacement operation.
26. Emenda restores clipboard state when clipboard transport is used.
27. The user continues writing.

---

# 6. Text Capture and Replacement

Create one small abstraction around desktop text interaction.

Conceptually:

```rust
trait TextSurfaceAdapter {
    fn capture_selection(&self) -> Result<CapturedSelection, TextSurfaceError>;
    fn focus_source(&self, source: &SourceApplication) -> Result<(), TextSurfaceError>;
    fn replace_selection(
        &self,
        source: &SourceApplication,
        replacement: &str,
    ) -> Result<(), TextSurfaceError>;
}
```

Keep the interface small.

For V0.1, use the simplest reliable mechanism available on the host platform.

A clipboard-assisted selected-text flow is an appropriate initial general implementation:

```text
save clipboard
→ capture selected text
→ restore clipboard

...

prepare replacement
→ focus source application
→ replace selected text
→ restore clipboard
```

Keep transport details inside the Rust text-surface module.

This boundary allows later platform-specific implementations using Windows UI Automation, macOS accessibility APIs or Linux AT-SPI while the correction workflow remains unchanged.

The V0.1 goal is **reliable selected-text interaction**, rather than broad passive observation.

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
    source: SourceApplication,
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

Use OpenRouter structured outputs with a strict JSON Schema matching the Emenda correction response.

Request routing compatible with the required structured-output parameters.

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

Use one Tauri application with two internal layers:

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
│   apply action      →   text replacement   │
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

## Rust responsibilities

Use Rust for:

- global hotkey
- active/source application tracking
- text capture
- text replacement
- clipboard preservation when used
- revision state
- snapshot state
- OpenRouter communication
- structured-response deserialisation
- secure API-key storage
- Tauri commands
- privileged OS operations

### Why this architecture

Tauri keeps Emenda as one local application while creating a clear boundary between product UI and privileged operating-system functionality.

Rust owns the operations where correctness, state discipline and system access matter most.

React and strict TypeScript keep the product surface fast to develop and easy to change.

Platform-specific code remains concentrated behind the Rust text-surface boundary while the correction workflow and product behaviour stay shared.

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

Grant frontend windows only the Tauri capabilities required for their responsibilities.

Keep secure credentials, external API communication and OS-level text operations inside the Rust core.

---

# 14. Settings

Create one simple Settings view.

## OpenRouter API Key

Provide:

- secure input field
- save action
- connection test
- clear success/error state

Store the credential through secure OS-appropriate storage managed from Rust.

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

When the final correction is applied, replace the original selected passage with the resulting corrected text.

---

# 16. Error Model

Use typed errors throughout the Rust core.

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
```

Map each category to a concise user-facing state.

The correction workflow should always resolve into a clear state that the UI can display.

---

# 17. Engineering Guardrails

Use the stack as the primary correctness system.

## Rust

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

Use strict TypeScript.

```json
{
  "strict": true
}
```

Use Zod for runtime validation of data crossing important frontend boundaries.

Run the project's TypeScript type checker as part of verification.

## Tauri

Use Tauri 2 capabilities to define the authority of each frontend surface.

The Rust core owns privileged functions.

## Repository quality

Keep the repository:

- compact
- readable
- typed
- testable
- easy to navigate
- easy for another coding agent to understand in one pass

Prefer direct code over framework layers whose value has not yet appeared in the V0.1 workflow.

---

# 18. Repository Structure

Use a compact structure similar to:

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
│       ├── inference/
│       ├── snapshot/
│       ├── settings/
│       ├── error.rs
│       └── lib.rs
│
├── tests/
├── package.json
├── README.md
└── src-tauri/
    └── tauri.conf.json
```

Adapt small details where Tauri conventions make another layout cleaner.

---

# 19. Minimal Test Strategy

The test suite follows the main sources of risk in the correction loop:

```text
external AI data
asynchronous state
text identity
source-application replacement
```

V0.1 therefore uses a deliberately narrow test target.

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

Verify successful deserialisation.

## Test 6: Desktop text loop

Validate manually or through host-appropriate integration tooling:

```text
select text
→ invoke Emenda
→ receive suggestion
→ apply correction
→ corrected text appears in source
```

Run this in:

1. one simple native text editor available on the host OS
2. one additional ordinary desktop application with editable text

Record the validated applications in the README.

### Why the test target is small

These tests cover the architecture's highest-risk boundaries while keeping V0.1 focused on one complete user outcome.

The first integration target proves the text transport.

The second demonstrates that the approach extends beyond a single controlled editor.

Broader application compatibility can then expand from measured behaviour.

---

# 20. Implementation Order

Implement in this order.

## Step 1: Scaffold

Create the Tauri 2 + React + strict TypeScript project.

Add the Rust module structure and Tauri capabilities.

Confirm the application builds.

## Step 2: OpenRouter

Implement:

```text
API-key storage
health check
model listing
structured correction request
typed response parsing
```

Test it independently.

## Step 3: Correction Core

Implement:

```text
Correction
TextSnapshot
revision counter
stale-result handling
validation
language profiles
system prompt
```

Add unit tests.

## Step 4: Desktop Text Transport

Implement:

```text
source application capture
selected-text capture
clipboard preservation where used
source refocus
replacement
```

Validate in one simple native editor.

## Step 5: Vertical Slice

Connect:

```text
hotkey
→ capture
→ snapshot
→ OpenRouter
→ suggestions
→ apply
→ source replacement
```

Make this loop reliable before additional UI refinement.

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

## Step 7: Second Application Test

Run the same complete flow in one additional common desktop application.

Fix issues exposed by that test while preserving the small architecture.

## Step 8: Verification

Run:

```text
Rust formatting
Rust Clippy
Rust tests
TypeScript type checking
frontend build
Tauri build
core integration tests
desktop smoke tests
```

Resolve errors until the repository reaches a clean final state.

---

# 21. V0.1 Scope

V0.1 consists of:

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
individual correction application
source-text replacement
secure API-key storage
typed errors
two desktop integration targets
```

This defines the complete V0.1 product.

The architecture leaves clean future seams for:

```text
passive background observation
Grammarly-style inline suggestions
Windows accessibility integration
macOS accessibility integration
Linux AT-SPI integration
browser extension / ChromeOS
personal dictionary
accepted/rejected correction history
local OpenAI-compatible inference
local LLMs
additional writing profiles
```

Each future capability can build on the same snapshot, correction and inference contracts.

---

# 22. Definition of Done

Emenda V0.1 is complete when this real workflow succeeds reliably:

```text
Launch Emenda
→ configure OpenRouter
→ select text in another desktop application
→ press the Emenda hotkey
→ capture the selected text
→ receive validated structured corrections
→ review suggestions
→ apply a correction
→ corrected text appears in the original application
→ continue writing
```

Verify that workflow repeatedly in:

```text
one simple native text editor
+
one additional ordinary desktop application
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

The primary success criterion is a **small, understandable and reliable complete correction loop**.

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

# 24. Final Engineering Principle

Use this principle when an implementation choice remains ambiguous:

> **Emenda delegates linguistic judgment to the AI model and keeps context, state, validation, security and text replacement deterministic in the local application.**

And use this product principle when a linguistic or UX choice remains ambiguous:

> **Emenda corrects the text while preserving the author's Duktus.**

Build the smallest coherent application that fully proves those two principles.

Complete the vertical slice, run the checks, fix the failures, validate the two desktop targets, and leave the repository in a working state with concise setup instructions in the README.
