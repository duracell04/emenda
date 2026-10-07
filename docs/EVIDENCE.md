# Emenda Implementation Evidence

> **Mutable evidence-ledger template for constitution version 2.2.1**

This ledger is a mutable factual record governed by its ledger-only procedure and sits outside the immutable checksum table in `PACKAGE-MANIFEST.md`. It remains empty in the documentation-only v2.2.1 freeze.

Implementation evidence may be added to this canonical ledger only under the separately authorized implementation objective. Each ledger-only commit identifies an already-existing implementation commit that was actually tested and leaves every frozen file unchanged. It records that fact; it does not claim to have tested itself.

## Baseline template

```text
constitution version:
freeze ID:
constitution commit:
constitution tree:
implementation objective:
UTC time:
environment:
toolchain:
limitations:
```

## Evidence entry template

```text
UTC time:
gate or increment:
constitution freeze ID:
constitution commit:
constitution tree:
critical requirement IDs:
tested implementation tree:
tested implementation commit:
commands or actions:
exact results:
evidence level: inspected | compiled | deterministic | integration | live | runtime
environment:
toolchain:
limitations or failures:
next checkpoint:
```

Preserve failures and later recoveries as separate entries. Never record credentials, authorization headers, raw private text, page URLs, tab/frame/document metadata, source identity, DOM structures, or raw provider bodies.

## Live provider evidence extension

For each complete Provider Gate run, append once:

```text
provider: localOmlx | openrouter
requested model:
server or provider build and verified policy:
enforced provider plugin policy: none | not applicable
cold/warm latency and memory observations:
startup preparation: explicit Settings synthetic local Test | none
separate startup-test outcome, full-processing latency and actual cold/warm condition:
semantic reviewers (agent/model identities):
reviewer profile/case coverage:
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning
```

For each official case in that run, append only:

```text
case:
selected model: <model ID | unavailable>
complete request latency:
outcome:
failure reason: <reason | none>
linguistic correctness (independent agent judgment):
```

After the 15 sequential cases, report `success count: x/15`. Do not retry or replace a case within the run. Preserve a failed run; record a complete recovery run separately after an implementation, configuration, or external-service change.

## Browser evidence extension

For browser or device evidence, append only the relevant fields:

```text
browser and exact version:
operating system and version:
device:
tester:
checklist results:
failures or limitations:
```

## Evidence entries

The documentation-only v2.2.1 freeze records an empty implementation-evidence state.

### Historical failure: first clean-clone run of the predecessor candidate

```text
UTC time: 2026-10-07T09:52:43.361938+00:00 (record assembly; historical execution UTC is unavailable unless stated below)
gate or increment: Browser Integration; historical failed final automated audit
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-APPLY-001, EM-PERM-002
tested implementation tree: ce0a6ef05adb3f492c7d37a2c3817d625e4a887b
tested implementation commit: 0df256caffbc0372182eb1127f707fd6a145b228
commands or actions: Clean local clone; locked npm install; full final automated audit.
exact results: 406 unit assertions, 40 surface tests and 47 shell tests passed. Browser integration reported 21 passed and 1 failed. A cold-worker Apply adversarial fixture expected one initial observation but saw zero within 5 seconds. Saved failed-run artifact SHA-256 66e7c6e795fd5cb11f0e10f3bcf9582409a0c14506bf94cdc96102b90fff6cab.
evidence level: integration
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; bundled Chromium 151.0.7922.34. Exact native Brave version is not asserted by these bundled-browser runs.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Zod 4.1.5; esbuild 0.28.2; Vitest 3.2.7; Playwright 1.62.1.
limitations or failures: The initial fixture failure was not reproduced in the next complete run. Its cause was not established; this is not evidence of a provider failure or proof that the later Settings startup repair caused the recovery. No failed assertion was removed or weakened.
next checkpoint: Retain this failed run and its separate same-commit recovery; final conformance uses the later tested implementation identity.
```

### Historical recovery: separate complete rerun of the same predecessor candidate

```text
UTC time: 2026-10-07T09:52:43.361938+00:00 (record assembly; historical execution UTC is unavailable unless stated below)
gate or increment: Browser Integration; historical complete automated audit
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-AUTH-003, EM-PERM-001, EM-PERM-002, EM-PERM-003, EM-PRIV-001, EM-PRIV-002, EM-PRIV-003, EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-APPLY-001, EM-APPLY-002, EM-APPLY-003, EM-SEC-001, EM-SEC-002, EM-SEC-003
tested implementation tree: ce0a6ef05adb3f492c7d37a2c3817d625e4a887b
tested implementation commit: 0df256caffbc0372182eb1127f707fd6a145b228
commands or actions: A new complete clean local clone run and full final automated audit at the same predecessor commit.
exact results: 406 unit assertions, 40 surface tests, 47 shell tests and 22 browser-integration tests passed. Constitution identity, architecture/assertion/schema placement, frozen prompt/privacy copy, dependency direction, deterministic CI, candidate secret-shape scan, production build/manifest/assets/branding and bundle confinement checks passed. Saved recovery artifact SHA-256 42ba8008da3b754ee2d87dd01d030f734d5059ab681a0147b99648811f04ac27.
evidence level: integration
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; bundled Chromium 151.0.7922.34. Exact native Brave version is not asserted by these bundled-browser runs.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Zod 4.1.5; esbuild 0.28.2; Vitest 3.2.7; Playwright 1.62.1.
limitations or failures: This separate recovery does not erase or explain the preceding failure. These results apply to the predecessor commit; they do not qualify a local linguistic model or prove personal-profile daily use.
next checkpoint: Use the later final implementation audit and installed checks below; retain both predecessor attempts.
```

### Historical limitation: sandboxed native Brave launch abort

```text
UTC time: 2026-10-06T19:49:08Z (reported crash time)
gate or increment: Browser Integration; historical environment limitation
constitution freeze ID: not established for the historical aborted launch; no qualification claim
constitution commit: not established for the historical aborted launch
constitution tree: not established for the historical aborted launch
critical requirement IDs: EM-SEC-003
tested implementation tree: unavailable in retained launch summary
tested implementation commit: unavailable in retained launch summary; not an acceptance-qualified implementation identity
commands or actions: Earlier sandboxed native Brave launch attempt; inspect the user-provided macOS crash summary.
exact results: A SIGABRT occurred in HIServices application-registration/TransformProcessType before ChromeMain initialization. The aborted launch supplied no valid extension-browser acceptance result. Later approved native Brave launches completed the automated browser suites below.
evidence level: runtime
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; reported Brave 154.1.96.59. Exact underlying Chromium build unavailable for the aborted launch.
toolchain: Not established for the historical aborted launch; final implementation toolchain recorded separately below.
limitations or failures: Sandboxed GUI-registration is the suspected launch context, not a proven extension defect. Exact tested source/constitution identity and an exact successful counterpart for this earlier attempt are not established in the retained crash summary. No authentication, SIP, or browser-security setting was weakened. The historical crash belongs to the earlier environment and must not be assigned the final implementation identity.
next checkpoint: Confirm historical identity if available; retain this limitation separately from successful final native-Brave automated runs.
browser and exact version: Brave 154.1.96.59 at the earlier reported environment; final saved environment is newer.
operating system and version: macOS 26.6 build 25G72.
device: Apple M5; 16 GB RAM.
tester: Autonomous implementation agent; crash summary supplied by the user.
checklist results: Native launch aborted before valid extension acceptance evidence.
failures or limitations: Exact historical source snapshot unresolved; no personal-use pass inferred.
```

### Historical uncertainty: earlier native Settings startup diagnostic

```text
UTC time: 2026-10-07T09:52:43.361938+00:00 (record assembly; historical execution UTC is unavailable unless stated below)
gate or increment: Browser Integration; Settings startup investigation
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-002, EM-AUTH-003, EM-PROV-001
tested implementation tree: pending confirmation for the earlier installed native observation
tested implementation commit: pending confirmation for the earlier installed native observation
commands or actions: Investigate the first Settings diagnostic around worker startup; inspect saved startup/probe evidence without repeating inference.
exact results: The earlier native diagnostic terminal outcome was not conclusively captured. A separate saved worker fixture probe reported 1 passed; that fixture does not establish the native model-test result. Saved worker-probe artifact SHA-256 7b85b7c574131d86630bcf954855c3e4a6ae0fe4dfda0c126170c773c85653c2.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; current native Brave 154.1.96.61 with Chromium 154.0.8037.98, arm64; bundled Chromium 151.0.7922.34. The current native version was captured after browser restart; earlier runs do not borrow that identity.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Zod 4.1.5; esbuild 0.28.2; Vitest 3.2.7; Playwright 1.62.1.
limitations or failures: Do not label the unknown native outcome as success, timeout, authentication failure, or provider failure. Its exact source identity and UTC need confirmation if it is retained as a detailed historical ledger entry. This observation is distinct from the unreproduced Apply fixture failure.
next checkpoint: Preserve uncertainty; use the reviewed startup ordering repair and later installed one-click Ready observation below.
```

### Completed: Settings startup ordering repair and final clean-clone verification

```text
UTC time: 2026-10-07T09:52:43.361938+00:00 (record assembly; historical execution UTC is unavailable unless stated below)
gate or increment: Mock Product; Architecture; deterministic Provider; Browser Integration automated coverage
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-AUTH-003, EM-PERM-001, EM-PERM-002, EM-PERM-003, EM-PRIV-001, EM-PRIV-002, EM-PRIV-003, EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-APPLY-001, EM-APPLY-002, EM-APPLY-003, EM-SEC-001, EM-SEC-002, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Settle worker startup diagnostic invalidation before trusted settings initialization. Confirm saved configuration before binding diagnostic generation, preserve drafts, and retain revision/sender/deadline guards. Run full canonical final clean-clone audit with locked install, type checks, all unit suites, production build and bundled-browser suites.
exact results: 411 unit assertions, 40 surface tests, 60 shell tests and 22 browser-integration tests passed. The focused startup coverage includes first diagnostics after idle restart, saved-configuration matching, preserved unsaved drafts, delayed/rejected initialization, obsolete diagnostic suppression and sticky storage-isolation failure. All 18 requirement mappings, constitution byte identity, locked toolchain/dependencies, strict architecture/assertion/schema placement, frozen prompt/privacy copy, dependency boundaries, secret-shape/source confinement, deterministic CI, production manifest/assets/branding and bundle confinement passed. Saved final clean-clone artifact SHA-256 4f4b7e0f5e54ae98b7a3d465cd1becc7eb13573b4235682bb05ecb5690a0c605.
evidence level: integration
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; current native Brave 154.1.96.61 with Chromium 154.0.8037.98, arm64; bundled Chromium 151.0.7922.34. The current native version was captured after browser restart; earlier runs do not borrow that identity.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Zod 4.1.5; esbuild 0.28.2; Vitest 3.2.7; Playwright 1.62.1.
limitations or failures: Automated production-path transport tests use synthetic provider fixtures; these counts do not substitute for live 15/15 linguistic qualification or personal native approval/restart checks. Direct minimum Chromium 140 compatibility remains unproven. No source or configuration change follows this tested commit in this draft.
next checkpoint: Complete one immutable official 15-case production run, independent semantic review, personal native/restart checks and exact remote publication verification.
```

### Completed: earlier actual-Brave automated suites; exact binary version unbound

```text
UTC time: 2026-10-07T09:52:43.361938+00:00 (record assembly; historical execution UTC is unavailable unless stated below)
gate or increment: Browser Integration; actual-Brave automated coverage
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-AUTH-003, EM-PERM-001, EM-PERM-002, EM-PERM-003, EM-PRIV-001, EM-PRIV-002, EM-PRIV-003, EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-APPLY-001, EM-APPLY-002, EM-APPLY-003, EM-SEC-001, EM-SEC-002, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Run the existing surface, extension-shell and browser-integration suites against the installed Brave executable with native GUI authorization.
exact results: 40 surface tests, 60 shell tests and 22 integration tests passed: 122 browser tests total. Coverage includes explicit site enable/revoke, trusted input/debounce, suggestions and Dismiss, off-caret insertion/deletion/replacement, immediate Apply authority, one native Undo, stale/navigation/worker invalidation, local authenticated/unauthenticated transport, permissions, malformed failures, deadlines and no cross-provider fallback. Saved actual-Brave artifact SHA-256 0e1d6003f0f535b93a2a7ea00bd0c1daddca538ece17ef51cf446bf47be0935e.
evidence level: integration
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; current native Brave 154.1.96.61 with Chromium 154.0.8037.98, arm64; bundled Chromium 151.0.7922.34. The current native version was captured after browser restart; earlier runs do not borrow that identity.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Zod 4.1.5; esbuild 0.28.2; Vitest 3.2.7; Playwright 1.62.1.
limitations or failures: These automated browser runs use synthetic provider fixtures and an isolated harness profile; they do not establish the pending personal-profile final checklist or linguistic qualification. The earlier harness executable version was not bound to its test execution; do not assign the later current version to this earlier run.
next checkpoint: Record the current version-bound 122-test rerun separately; complete final native personal-profile workflow and worker/server restart recovery on the byte-identical installed build.
browser and exact version: Installed Brave executable used, but exact version was not bound to this earlier harness execution. A separate earlier environment snapshot showed 154.1.96.60. Current native version is 154.1.96.61/Chromium 154.0.8037.98; the separate 122-test rerun with start-time executable/version metadata is in progress.
operating system and version: macOS 26.6 build 25G72.
device: Apple M5; 16 GB RAM.
tester: Autonomous implementation agent using the existing browser harness.
checklist results: All 122 actual-Brave automated surface/shell/integration tests passed.
failures or limitations: Linguistic qualification, personal approval/restart completion and direct Chromium 140 proof are not inferred.
```

### Completed: byte-identical production build installed

```text
UTC time: 2026-10-07T09:45:41.745635+00:00
gate or increment: V0.1 Conformance; installed-build identity
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-AUTH-003, EM-PERM-001, EM-PERM-002, EM-PERM-003, EM-PRIV-001, EM-PRIV-002, EM-PRIV-003, EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-APPLY-001, EM-APPLY-002, EM-APPLY-003, EM-SEC-001, EM-SEC-002, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Install the verified production extension build in the existing personal Brave profile and compare its entire file inventory and bytes against the tested build.
exact results: All 14 production build files are byte-identical. Manifest and inventory remain unchanged. Aggregate installed-build SHA-256 ace801dd08efe48ae8e044dd37f9431e4203dcfc58289b153ff141a558c7cd1b. Saved installation-identity artifact SHA-256 ccf9f2bb0cfbe2a029ab5d85e3713d0ed20e96a23150c0dd1f2cf704c1965ead.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; current native Brave 154.1.96.61 with Chromium 154.0.8037.98, arm64; bundled Chromium 151.0.7922.34. The current native version was captured after browser restart; earlier runs do not borrow that identity.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Zod 4.1.5; esbuild 0.28.2; Vitest 3.2.7; Playwright 1.62.1.
limitations or failures: Build installation and byte identity are confirmed. This entry alone does not claim a successful later browser/server restart, full personal daily-use checklist or linguistic qualification.
next checkpoint: Complete the final installed native workflow and restart checks before declaring personal usability.
```

### Completed: one explicit installed cold Settings Test reached Ready

```text
UTC time: 2026-10-07T09:47:00.732Z
gate or increment: Browser Integration; separate local startup readiness
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-002, EM-AUTH-003, EM-PROV-001, EM-PROV-002, EM-PRIV-001, EM-PRIV-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: From no loaded model and zero active server requests, click the installed Settings Test once for the directly configured gemma-3-12b-it-4bit model. Observe the later complete native accessibility state.
exact results: One Test click began at 2026-10-07T09:47:00.732Z. The later full accessibility tree showed Ready, and the selected Gemma 3 model was the sole loaded model afterward. No second Test was performed. Saved native cold-test artifact SHA-256 abc7f602420787e01ffdab547fece9a9c02a183af482dd766cbb22d72399a5e0. The production local readiness operation retains its fixed 25,000 ms deadline; all writing/discovery/OpenRouter/official corpus operations retain 15,000 ms.
evidence level: runtime
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; current native Brave 154.1.96.61 with Chromium 154.0.8037.98, arm64; bundled Chromium 151.0.7922.34. The current native version was captured after browser restart; earlier runs do not borrow that identity.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Zod 4.1.5; esbuild 0.28.2; Vitest 3.2.7; Playwright 1.62.1.
limitations or failures: Exact full-processing latency is unavailable: initial accessibility diffs omitted updates, and the later observedReady time is not the completion time. Do not subtract those timestamps or treat the gap as provider latency. Ready is ephemeral compatibility evidence only; it grants no linguistic qualification, origin/Apply authority, persistence or future latency guarantee.
next checkpoint: Record the official complete 15-case run and independent semantic review separately; complete installed browser/server restart recovery.
```


### Completed: official 15/15 local production qualification and two independent sealed agent reviews

```text
UTC time: 2026-10-07T09:48:40.223Z to 2026-10-07T09:49:42.494Z; capture and sealed review finalized afterward
gate or increment: Provider Gate; one official final production qualification run
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-PRIV-001, EM-PRIV-003, EM-SEC-001, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Run the canonical 15 cases once, strictly sequentially, through production parsing and derivation at the unchanged 15,000 ms full-processing deadline. Seal that capture; two independent agents review all 15 results; finalize qualification offline from their sealed judgments without further inference.
exact results: One production run completed all 15 cases with exact requested/returned model identity, strict parsing/derivation, required result and deadline success. Both agents judged every case correct: 30 independent positive judgments. Offline runner finalization reported success count 15/15, status qualified, exit 0. The initial sealed capture retained pending-review status; offline judgments finalized the same run, not a second inference attempt. Sealed capture digest 746ecd6ce19bbfb0528d106582a89791e14305b6cf3071489a9ff27414ff83e1; capture artifact SHA-256 d53968dd9a1e01c1af2cc318fe2bf3ba21121473fb1ba8e0c30be2c8c74b5fb0; combined review artifact SHA-256 6fc5f90a7644fd5f15927a3b02a01c3c314ea8e1123bc9ee489686f7123920db; qualified output SHA-256 49db315aaa83cb4c3e3d2790309be03c4568fc319ef90916e8c7b622ab32eacb.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; current native Brave 154.1.96.61 with Chromium 154.0.8037.98, arm64; bundled Chromium 151.0.7922.34. The current native version was captured after browser restart; earlier runs do not borrow that identity. Brave was running during qualification; its exact executable version was not separately bound to that earlier production run.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; compiled production provider implementation at the exact tested commit.
limitations or failures: This qualifies the observed model/environment, not universal linguistic correctness, future latency, permanent residency or other devices. All earlier failed attempts remain retained. No case was retried, replaced or repaired. No configuration/model/prompt/source change, lifecycle operation or extra warm-up occurred during this run. System swap was already nonzero with Brave and other Mac applications running; aggregate memory/swap changes cannot be attributed solely to this model.
next checkpoint: Complete final installed native Apply/Dismiss/Undo, revoke/navigation and server-restart recovery; record the current binary-bound browser rerun and fresh remote identity verification.
provider: localOmlx
requested model: gemma-3-12b-it-4bit
server or provider build and verified policy: oMLX 0.7.0.dev4; loopback binding; authentication enabled; model_fallback false; critical logging; strict production schema/transport; source mlx-community/gemma-3-12b-it-4bit revision 86cc6a8dedbc456dd0e4af01a9d09f396f77e558, weight bytes 8028675248.
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: The official run followed the separately recorded explicit cold Settings Test and used the warm resident model. Before/during/after oMLX process memory: 8327650360/8751406160/8329845864 bytes; dynamic process maxima 12490480696/11608292432/12481387624 bytes. Server guard pressure was ok in all three snapshots, with one loaded model. Mac swap used 9327.44/8726.88/9474.38 MiB before/during/after; system-wide memory free 33/22/34 percent. Swap was nonzero before the run and cumulative swap counters rose; the mixed desktop workload is a confounder. Per-case measured latency is recorded below, with no percentile or aggregate performance claim.
startup preparation: explicit Settings synthetic local Test
separate startup-test outcome, full-processing latency and actual cold/warm condition: One installed Test started 2026-10-07T09:47:00.732Z with no model loaded and zero active requests; eventual full accessibility state showed Ready. Exact full-processing latency unavailable; do not use late accessibility observation time as completion latency. The internal 25,000 ms deadline was enforced. Model was warm/resident for the official corpus.
semantic reviewers (agent/model identities): /root/semantic_review and /root/final_semantic_review; each model identity: Codex agent; inherited model, exact deployment ID unavailable. Both are independent of the implementation owner; this is agent review, not human review.
reviewer profile/case coverage: Each reviewer independently covered all 15 case IDs, all six supported profiles, Auto, clean/correction decisions and fixed-language/unsupported cases; both assessed actual category, explanation, correction language and meaning preservation.
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning
```

```text
case: de-CH-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 8146ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: de-CH-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 2913ms
outcome: Clean:de-CH
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: en-GB-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 5069ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: en-GB-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 2920ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: en-US-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 5035ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: en-US-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 2884ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: fr-FR-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 5622ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: fr-FR-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 3305ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: ka-GE-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 5851ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: ka-GE-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 3291ms
outcome: Clean:ka-GE
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: ru-RU-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 4977ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: ru-RU-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 3094ms
outcome: Clean:ru-RU
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: auto-fr-FR
selected model: gemma-3-12b-it-4bit
complete request latency: 3129ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: fixed-en-GB-German
selected model: gemma-3-12b-it-4bit
complete request latency: 3045ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

```text
case: auto-unsupported-Japanese
selected model: gemma-3-12b-it-4bit
complete request latency: 2970ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): true; independently confirmed by both named agents
```

success count: 15/15

### Completed: installed browser restart and current native version

```text
UTC time: 2026-10-07T10:00:49.588588+00:00 (record assembled from the parent-reported completed browser restart)
gate or increment: Browser Integration; installed personal browser restart
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Restart the personal Brave browser with the byte-identical installed extension; inspect restored browser state and current native version.
exact results: Browser restart succeeded and existing tabs restored. Current native Brave version is 154.1.96.61 with Chromium 154.0.8037.98, arm64. A separate full 122-test browser rerun was started against this exact executable with start metadata and executable SHA-256 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
evidence level: runtime
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; current native Brave 154.1.96.61 with Chromium 154.0.8037.98, arm64; bundled Chromium 151.0.7922.34. The current native version was captured after browser restart; earlier runs do not borrow that identity.
toolchain: Installed production build at the exact tested commit; no source, prompt or build change.
limitations or failures: A successful browser restart alone does not establish final personal writing types, site revocation, navigation invalidation or server-restart recovery. The new binary-bound 122-test rerun is in progress; its result is not claimed here.
next checkpoint: Record the binary-bound rerun result and complete the remaining native workflow/server-restart checks.
browser and exact version: Brave 154.1.96.61; Chromium 154.0.8037.98; arm64.
operating system and version: macOS 26.6 build 25G72.
device: Apple M5; 16 GB RAM.
tester: Autonomous implementation agent using native Brave.
checklist results: Browser restart succeeded; tabs restored; exact current native browser version captured.
failures or limitations: Final native writing types, revoke/navigation and server restart remain pending.
```


### Failure: first current-binary Brave surface run

```text
UTC time: 2026-10-07T09:54:31.754109+00:00
gate or increment: Browser Integration; first version-bound final browser attempt
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-SEC-002, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Start the full harness against the recorded current Brave executable, with unchanged final source/build.
exact results: Surface suite reported 39 passed and 1 failed: fixture page.addScriptTag exceeded its 30,000 ms test timeout. Shell and integration suites were not reached in that attempt. Initial failed-run log SHA-256 a053fd1d0b9b7dcd616c74d52c642f51d9fe95dbf71ff6eebffe9ce612609f02.
evidence level: integration
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. The bound harness executable SHA-256 is 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: The initial timeout cause is unproven. No model-RAM causal claim is supported. Product source, tests, assertions, prompt and provider settings were not altered to obtain a later pass.
next checkpoint: Retain the failure; diagnose with separate probes and record the complete normal recovery below.
```

### Diagnostic observation: tracing changed the value-read counter

```text
UTC time: 2026-10-07T10:11:53.227433+00:00 (draft record assembly; earlier action UTC unavailable unless stated)
gate or increment: Browser Integration; targeted traced diagnostic
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-SEC-002, EM-PRIV-001
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Run the targeted refused-surface fixture with tracing; independently inspect the Playwright snapshot collector.
exact results: The traced probe reported 6 observed getter reads where 0 were expected. Independent reviewer /root/audit_migration confirmed that Playwright snapshot collection itself calls the instrumented input.value getter six times. Product inference requests and mutations were 0. Traced-probe log SHA-256 f368c9f65df3dcd17de3966082e89c01bf5970add44760449acd98adf7dbb48a.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. The bound harness executable SHA-256 is 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: These six reads are diagnostic-tool instrumentation, not product-originated reads. This explains the traced probe counter but does not prove the cause of the earlier page.addScriptTag timeout. No private field values, DOM structure or browser identity metadata are recorded.
next checkpoint: Retain the traced observation and independently run the unchanged fixture under its normal no-trace test configuration.
```

### Recovery: targeted normal refused-surface fixture

```text
UTC time: 2026-10-07T10:11:53.227433+00:00 (draft record assembly; earlier action UTC unavailable unless stated)
gate or increment: Browser Integration; separate normal targeted rerun
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-SEC-002, EM-PRIV-001
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Run the same unchanged targeted fixture with normal tracing disabled.
exact results: The targeted normal run passed in 17.1 seconds. Normal-probe log SHA-256 f9c7e282c543062ffada9da9e46ebf95a817d869d4982e9d8271944ac69f7d0f.
evidence level: integration
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. The bound harness executable SHA-256 is 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: The normal configuration avoids trace collector interference with this deliberate getter counter. No product/test assertion was weakened. A targeted pass does not replace the required complete suite.
next checkpoint: Run and preserve the complete normal surface/shell/integration suites against the same bound binary.
```

### Completed recovery: full current-binary Brave gate

```text
UTC time: 2026-10-07T10:07:11.181575+00:00
gate or increment: Browser Integration; complete version-bound actual-Brave suites
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-AUTH-003, EM-PERM-001, EM-PERM-002, EM-PERM-003, EM-PRIV-001, EM-PRIV-002, EM-PRIV-003, EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-APPLY-001, EM-APPLY-002, EM-APPLY-003, EM-SEC-001, EM-SEC-002, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Run a new complete normal surface, shell and integration attempt with the unchanged final source/build and exact current Brave binary. Verify executable identity remains unchanged.
exact results: 40 surface,60 shell and 22 integration tests passed: 122 total. Start/completion metadata binds Brave 154.1.96.61 and native Chromium 154.0.8037.98 to executable SHA-256 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468. Executable hash remained unchanged. Completed-gate artifact SHA-256 030325971d4155bdc6e753f6488cdd0b96936f9cc91d78ba522295fc1e056962; complete recovery-log SHA-256 0d5ec4df00fab984d032851790710db58dcb607fc23d455beac5cdde62c4d202.
evidence level: integration
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. The bound harness executable SHA-256 is 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: The model was unloaded during this successful final browser run. That condition does not establish model RAM as the cause of the earlier timeout. The browser fixtures use synthetic responses; live model qualification is the separately sealed 15/15 run above. This normal recovery preserves the initial failed run and traced-probe observation; no source/prompt/schema/deadline/permission change occurred.
next checkpoint: Complete remaining actual personal native inference/Apply/Undo/navigation/worker-restart checks.
browser and exact version: Brave 154.1.96.61; Chromium 154.0.8037.98; arm64; exact executable SHA bound and unchanged.
operating system and version: macOS 26.6 build 25G72.
device: Apple M5; 16 GB RAM.
tester: Autonomous implementation agent using the existing normal browser harness.
checklist results: Complete40 surface+60 shell+22 integration gate passed on the recorded current binary.
failures or limitations: Earlier39/40timeout retained; traced six-read instrumentation effect retained; no timeout cause inferred.
```

### Completed: one oMLX restart recovered with unchanged persisted policy

```text
UTC time: 2026-10-07T10:00:07.574632+00:00 to 2026-10-07T10:00:16.383469+00:00
gate or increment: V0.1 Conformance; local server restart observation
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PRIV-001, EM-PRIV-003, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Issue one documented server restart request; observe health/model/auth status and compare persisted settings bytes. Perform no inference, readiness test, load/unload request or configuration mutation in this server check.
exact results: Restart returned 202 after 13 ms. Temporary unavailability was observed; healthy recovery was first observed at 8682 ms from request start, bounded by the preceding failed poll at 8255 ms. Polling gives a recovery bound, not an exact internal availability time. Post-restart loaded-model count was 0. Authenticated catalog returned 200; unauthenticated catalog returned 401. Persisted settings bytes/object, authentication and KV/cache policy were unchanged; loopback binding, critical logging and model_fallback:false were preserved. Sealed production capture remained unchanged. Server-restart artifact SHA-256 9ef62d888be47c9b909683f37853cadc88f99882202596c05920effb658617f3.
evidence level: runtime
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. The bound harness executable SHA-256 is 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: No extension inference/readiness test was performed after server recovery in this check. Healthy/catalog/auth status does not establish linguistic readiness. The model is unloaded, so any later Settings warming must be explicit and separately recorded. Historical server evidence remains retained; native crash-diagnostic and future residency/latency limitations still apply.
next checkpoint: Complete installed extension recovery after this server restart through the remaining explicitly authorized native workflow.
```

### Historical permission proof: initial grant and denial on prior implementation

```text
UTC time: 2026-10-07T10:11:53.227433+00:00 (draft record assembly; earlier action UTC unavailable unless stated)
gate or increment: Browser Integration; historical first site permission prompt
constitution freeze ID: emenda-clean-room-v2.2.0-2026-10-06
constitution commit: 11080a3c4be943f57ea35293b995d80453ffecad
constitution tree: 6955e79551d9a4ea372729bf75e7c3317e3da894
critical requirement IDs: EM-PERM-001, EM-PERM-002, EM-PERM-003
tested implementation tree: 6129220941d659aa4c332807581e538b0b124df3
tested implementation commit: 51b03071e7909d5211a433bd45c65408a4fca5cc
commands or actions: Exercise the initial native writing-site permission grant and denial under the prior implementation state.
exact results: Parent reports that both initial native grant and denial were verified on 51b03071e7909d5211a433bd45c65408a4fca5cc. Denial did not activate the writing site. These are historical prior-state observations; no fresh native permission prompt has yet been shown for the final 5a state.
evidence level: runtime
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; personal native Brave. Exact earlier browser binary version was not bound to this historical permission-prompt observation; later current native version is recorded separately.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: Do not assign these historical first-prompt results to the final implementation. Exact earlier action UTC and browser binary binding are not established in this draft; the final current state has separate enable/revoke observations below.
next checkpoint: Retain this prior-state evidence; explicitly distinguish any later final-state fresh permission-prompt verification.
```

### Completed: final native exact-port site revoke

```text
UTC time: 2026-10-07T10:11:53.227433+00:00 (draft record assembly; earlier action UTC unavailable unless stated)
gate or increment: Browser Integration; installed personal site-authority lifecycle
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PERM-001, EM-PERM-002, EM-PERM-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: After the successful personal browser restart, enable the authorized synthetic writing site. Open fresh Settings, inspect its exact-port enabled row, revoke it and inspect current toolbar state.
exact results: The enabled site appeared as one exact-port Settings row. Revoke completed; Settings showed no enabled sites and the toolbar offered Enable for the site. This observed final-state enable/revoke sequence passed on the unchanged final implementation.
evidence level: runtime
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. The bound harness executable SHA-256 is 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: No page URL, tab identity or DOM metadata is recorded. This uses existing native site authority and does not claim a new final-state Chrome permission prompt. Native inference/Apply types, one-step Undo, navigation invalidation and worker restart remain pending.
next checkpoint: Complete the remaining final native writing, approval, Undo and lifecycle checks before claiming personal daily-use completion.
browser and exact version: Brave 154.1.96.61; Chromium 154.0.8037.98; arm64.
operating system and version: macOS 26.6 build 25G72.
device: Apple M5; 16 GB RAM.
tester: Autonomous implementation agent using native personal Brave.
checklist results: Enabled exact-port Settings row observed; Revoke succeeded; no-enabled-sites state and toolbar Enable observed.
failures or limitations: Fresh final-state initial permission prompt not yet verified; final inference/Apply/Undo/navigation/worker-restart checks pending.
```

### Completed: exact final implementation remote ref verified

```text
UTC time: 2026-10-07T09:56Z (parent-reported fresh remote API verification)
gate or increment: V0.1 Conformance; tested-state publication identity
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PRIV-003, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Publish the exact already-tested implementation through a lease against the preserved prior branch tip; independently inspect the fresh remote API ref and align local tracking.
exact results: Parent freshly API-verified the implementation branch at 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63 at 09:56 UTC; expected tested tree is 154341857664f7da22560a6cd4980008b0f46609. Local tracking is aligned. Blueprint main and successor freeze remain cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d; preserved v2.2.0 freeze is 11080a3c4be943f57ea35293b995d80453ffecad and implementation build/v0.1 baseline is fa27dbfe5d18f8c8cec5d5c5a6c04cf7259f66aa.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. The bound harness executable SHA-256 is 2ea490b3b0b7765ef4c7817828f2ee9446fce2b28a435fc8b5f174323d2f4468.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Vitest 3.2.7; Playwright 1.62.1; exact final source and production build unchanged.
limitations or failures: Publication establishes the tested code identity; it does not complete the remaining native personal-use checks. The factual blueprint ledger-only publication is still pending and must leave every frozen file unchanged.
next checkpoint: Finish final native evidence, then append the sanitized ledger-only record for this already-tested commit and verify ledger publication/clean tracked worktrees.
```


### Historical preflight attempt: Llama-3.2-3B-Instruct-4bit / preflight-1

```text
UTC time: 2026-10-06T19:25:04.375005+00:00 to 2026-10-06T19:25:25.869230+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model Llama-3.2-3B-Instruct-4bit; attempt preflight-1; prompt SHA-256 107dfd38467cf1e481cf13d3998959d8fef22ba79abd839d75713124a49d76a3 (summary-recorded; request-time prompt bytes not independently sealed by the artifact; reconstructed from recorded baseline fa27dbf).
exact results: 0/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, de-CH-clean, en-GB-correction, en-GB-clean, en-US-correction, en-US-clean, fr-FR-correction, fr-FR-clean, ka-GE-correction, ka-GE-clean, ru-RU-correction, ru-RU-clean, auto-fr-FR, fixed-en-GB-German, auto-unsupported-Japanese. Returned identity counts: Llama-3.2-3B-Instruct-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=4384 ms/Correction:grammar/match=false; de-CH-clean=1106 ms/Correction:grammar/match=false; en-GB-correction=944 ms/invalid model result/match=false; en-GB-clean=1013 ms/Correction:grammar/match=false; en-US-correction=905 ms/Correction:grammar/match=false; en-US-clean=1040 ms/Correction:punctuation/match=false; fr-FR-correction=1195 ms/Correction:punctuation/match=false; fr-FR-clean=1111 ms/Correction:punctuation/match=false; ka-GE-correction=1913 ms/Correction:punctuation/match=false; ka-GE-clean=2464 ms/Correction:punctuation/match=false; ru-RU-correction=1457 ms/Correction:grammar/match=false; ru-RU-clean=1080 ms/Correction:grammar/match=false; auto-fr-FR=911 ms/Correction:punctuation/match=false; fixed-en-GB-German=946 ms/Correction:grammar/match=false; auto-unsupported-Japanese=976 ms/Correction:grammar/match=false. Artifact SHA-256 f3b08e808d1760d6de243bc52bdb8fe31388feed2f52551f600f06f451d34854.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=6454707880, tracked model memory=0, loaded count=0; after: ceiling=6879276152, tracked model memory=1897871091, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical preflight attempt: Llama-3.2-3B-Instruct-4bit / preflight-2-clarified

```text
UTC time: 2026-10-06T19:42:38.229641+00:00 to 2026-10-06T19:42:49.620315+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model Llama-3.2-3B-Instruct-4bit; attempt preflight-2-clarified; prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 (artifact-recorded).
exact results: 7/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: en-GB-correction, en-US-correction, fr-FR-correction, ka-GE-correction, ka-GE-clean, ru-RU-correction, auto-fr-FR, fixed-en-GB-German. Returned identity counts: Llama-3.2-3B-Instruct-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=4157 ms/Correction:spelling/match=true; de-CH-clean=478 ms/Clean:de-CH/match=true; en-GB-correction=382 ms/Clean:en-GB/match=false; en-GB-clean=382 ms/Clean:en-GB/match=true; en-US-correction=431 ms/Unsupported/match=false; en-US-clean=447 ms/Clean:en-US/match=true; fr-FR-correction=440 ms/Unsupported/match=false; fr-FR-clean=466 ms/Clean:fr-FR/match=true; ka-GE-correction=470 ms/Unsupported/match=false; ka-GE-clean=473 ms/Unsupported/match=false; ru-RU-correction=466 ms/Clean:ru-RU/match=false; ru-RU-clean=464 ms/Clean:ru-RU/match=true; auto-fr-FR=434 ms/Unsupported/match=false; fixed-en-GB-German=1372 ms/Correction:spelling/match=false; auto-unsupported-Japanese=481 ms/Unsupported/match=true. Artifact SHA-256 353878e1d3e913908846b40724061624c4f0df4610a42d4bb89d8e6cf2b7b630.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=9596995336, tracked model memory=0, loaded count=0; after: ceiling=8631307384, tracked model memory=1897871091, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical preflight attempt: Qwen3-4B-Instruct-2507-4bit / preflight-1

```text
UTC time: 2026-10-06T19:26:47.696667+00:00 to 2026-10-06T19:27:10.133644+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model Qwen3-4B-Instruct-2507-4bit; attempt preflight-1; prompt SHA-256 107dfd38467cf1e481cf13d3998959d8fef22ba79abd839d75713124a49d76a3 (summary-recorded; request-time prompt bytes not independently sealed by the artifact; reconstructed from recorded baseline fa27dbf).
exact results: 7/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, de-CH-clean, en-US-clean, fr-FR-correction, ka-GE-correction, ru-RU-correction, auto-fr-FR, fixed-en-GB-German. Returned identity counts: Qwen3-4B-Instruct-2507-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=5095 ms/Correction:grammar/match=false; de-CH-clean=1737 ms/Correction:grammar/match=false; en-GB-correction=1831 ms/Correction:spelling/match=true; en-GB-clean=543 ms/Clean:en-GB/match=true; en-US-correction=1462 ms/Correction:spelling/match=true; en-US-clean=1289 ms/Correction:grammar/match=false; fr-FR-correction=1809 ms/Correction:grammar/match=false; fr-FR-clean=536 ms/Clean:fr-FR/match=true; ka-GE-correction=4040 ms/Correction:grammar/match=false; ka-GE-clean=599 ms/Clean:ka-GE/match=true; ru-RU-correction=599 ms/Clean:ru-RU/match=false; ru-RU-clean=585 ms/Clean:ru-RU/match=true; auto-fr-FR=1196 ms/Correction:grammar/match=false; fixed-en-GB-German=591 ms/Clean:en-GB/match=false; auto-unsupported-Japanese=473 ms/Unsupported/match=true. Artifact SHA-256 8935d13801d41d2d4b15ce02818437b9823f1ba91cc9f47b453dffd04b22be9f.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=7431227168, tracked model memory=0, loaded count=0; after: ceiling=7080824168, tracked model memory=2376173537, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical preflight attempt: Qwen3-4B-Instruct-2507-4bit / preflight-2-clarified

```text
UTC time: 2026-10-06T19:29:47.020512+00:00 to 2026-10-06T19:30:21.913688+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model Qwen3-4B-Instruct-2507-4bit; attempt preflight-2-clarified; prompt SHA-256 ce25e4301ecfc685fa0d5a3bd32d850b5d5fffc5eb1ef02079ad2c568898cfa4 (artifact-recorded).
exact results: 11/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-clean, ka-GE-clean, ru-RU-clean, fixed-en-GB-German. Returned identity counts: Qwen3-4B-Instruct-2507-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=5266 ms/Correction:spelling/match=true; de-CH-clean=1917 ms/Correction:spelling/match=false; en-GB-correction=1760 ms/Correction:spelling/match=true; en-GB-clean=582 ms/Clean:en-GB/match=true; en-US-correction=1684 ms/Correction:spelling/match=true; en-US-clean=575 ms/Clean:en-US/match=true; fr-FR-correction=1985 ms/Correction:spelling/match=true; fr-FR-clean=591 ms/Clean:fr-FR/match=true; ka-GE-correction=5325 ms/Correction:punctuation/match=true; ka-GE-clean=7305 ms/Correction:style/match=false; ru-RU-correction=3048 ms/Correction:punctuation/match=true; ru-RU-clean=2895 ms/Correction:spelling/match=false; auto-fr-FR=749 ms/Clean:fr-FR/match=true; fixed-en-GB-German=610 ms/Clean:en-GB/match=false; auto-unsupported-Japanese=549 ms/Unsupported/match=true. Artifact SHA-256 e12786cb24941a68883c20d28d02f7e18830b6d857c41af5d577051cf35bfaa3.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=6899155688, tracked model memory=0, loaded count=0; after: ceiling=7271549808, tracked model memory=2376173537, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical preflight attempt: Qwen3.5-4B-4bit / preflight-1-clarified

```text
UTC time: 2026-10-06T19:31:44.344975+00:00 to 2026-10-06T19:32:10.677589+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model Qwen3.5-4B-4bit; attempt preflight-1-clarified; prompt SHA-256 ce25e4301ecfc685fa0d5a3bd32d850b5d5fffc5eb1ef02079ad2c568898cfa4 (artifact-recorded).
exact results: 12/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, ka-GE-correction, ka-GE-clean. Returned identity counts: Qwen3.5-4B-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=8038 ms/Correction:grammar/match=false; de-CH-clean=943 ms/Clean:de-CH/match=true; en-GB-correction=2363 ms/Correction:spelling/match=true; en-GB-clean=897 ms/Clean:en-GB/match=true; en-US-correction=2180 ms/Correction:spelling/match=true; en-US-clean=876 ms/Clean:en-US/match=true; fr-FR-correction=2694 ms/Correction:spelling/match=true; fr-FR-clean=909 ms/Clean:fr-FR/match=true; ka-GE-correction=866 ms/Unsupported/match=false; ka-GE-clean=853 ms/Unsupported/match=false; ru-RU-correction=2113 ms/Correction:punctuation/match=true; ru-RU-clean=946 ms/Clean:ru-RU/match=true; auto-fr-FR=896 ms/Clean:fr-FR/match=true; fixed-en-GB-German=853 ms/Unsupported/match=true; auto-unsupported-Japanese=851 ms/Unsupported/match=true. Artifact SHA-256 5dd82afa6ee66b3b06c003e9bb63b32ca7a2ae6ba29ab7350c31c1744faa622b.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=8942553968, tracked model memory=2376173537, loaded count=1; after: ceiling=9375732920, tracked model memory=5562189266, loaded count=2. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical preflight attempt: gemma-4-12B-it-4bit / preflight-1

```text
UTC time: 2026-10-06T19:19:40.573784+00:00 to 2026-10-06T19:19:41.491032+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model gemma-4-12B-it-4bit; attempt preflight-1; prompt SHA-256 107dfd38467cf1e481cf13d3998959d8fef22ba79abd839d75713124a49d76a3 (summary-recorded; request-time prompt bytes not independently sealed by the artifact; reconstructed from recorded baseline fa27dbf).
exact results: 0/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, de-CH-clean, en-GB-correction, en-GB-clean, en-US-correction, en-US-clean, fr-FR-correction, fr-FR-clean, ka-GE-correction, ka-GE-clean, ru-RU-correction, ru-RU-clean, auto-fr-FR, fixed-en-GB-German, auto-unsupported-Japanese. Returned identity counts: unavailable:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=2 ms/HTTP 507/match=false; de-CH-clean=1 ms/HTTP 507/match=false; en-GB-correction=1 ms/HTTP 507/match=false; en-GB-clean=1 ms/HTTP 507/match=false; en-US-correction=1 ms/HTTP 507/match=false; en-US-clean=1 ms/HTTP 507/match=false; fr-FR-correction=1 ms/HTTP 507/match=false; fr-FR-clean=1 ms/HTTP 507/match=false; ka-GE-correction=1 ms/HTTP 507/match=false; ka-GE-clean=1 ms/HTTP 507/match=false; ru-RU-correction=1 ms/HTTP 507/match=false; ru-RU-clean=1 ms/HTTP 507/match=false; auto-fr-FR=1 ms/HTTP 507/match=false; fixed-en-GB-German=1 ms/HTTP 507/match=false; auto-unsupported-Japanese=1 ms/HTTP 507/match=false. Artifact SHA-256 14ce6abefb7c9e2b8207f64d79a25892f1f0e98f473c97b0c1198381ae9471b8.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=12713115648, tracked model memory=7078091486, loaded count=1; after: ceiling=6122744736, tracked model memory=0, loaded count=0. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical preflight attempt: gemma-4-12B-it-4bit / preflight-2-clarified

```text
UTC time: 2026-10-06T19:34:24.311404+00:00 to 2026-10-06T19:35:15.540773+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model gemma-4-12B-it-4bit; attempt preflight-2-clarified; prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 (artifact-recorded).
exact results: 12/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: ka-GE-correction, ru-RU-correction, auto-unsupported-Japanese. Returned identity counts: gemma-4-12B-it-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=14313 ms/Correction:spelling/match=true; de-CH-clean=1932 ms/Clean:de-CH/match=true; en-GB-correction=3866 ms/Correction:spelling/match=true; en-GB-clean=1989 ms/Clean:en-GB/match=true; en-US-correction=3906 ms/Correction:spelling/match=true; en-US-clean=1918 ms/Clean:en-US/match=true; fr-FR-correction=4643 ms/Correction:spelling/match=true; fr-FR-clean=2283 ms/Clean:fr-FR/match=true; ka-GE-correction=4649 ms/Correction:punctuation/match=false; ka-GE-clean=2017 ms/Clean:ka-GE/match=true; ru-RU-correction=1967 ms/Clean:ru-RU/match=false; ru-RU-clean=1938 ms/Clean:ru-RU/match=true; auto-fr-FR=1917 ms/Clean:fr-FR/match=true; fixed-en-GB-German=1917 ms/Unsupported/match=true; auto-unsupported-Japanese=1925 ms/Clean:fr-FR/match=false. Artifact SHA-256 92ae9772a5c7bd3e627711b256f194cacee578589197bab7669e11c2f937ed0b.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=9545606608, tracked model memory=0, loaded count=0; after: ceiling=10716707176, tracked model memory=7078091486, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical preflight attempt: gemma-4-e4b-it-4bit / preflight-1-clarified

```text
UTC time: 2026-10-06T19:46:15.328751+00:00 to 2026-10-06T19:46:41.383681+00:00
gate or increment: Historical candidate diagnostic only; synthetic-preflight-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Direct synthetic HTTP preflight; whole attempt recorded separately. It is not a production qualification run. Exact probe code commit/tree and frozen constitution identity are unavailable. Requested model gemma-4-e4b-it-4bit; attempt preflight-1-clarified; prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: ru-RU-clean. Returned identity counts: gemma-4-e4b-it-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=10430 ms/Correction:spelling/match=true; de-CH-clean=737 ms/Clean:de-CH/match=true; en-GB-correction=1575 ms/Correction:spelling/match=true; en-GB-clean=722 ms/Clean:en-GB/match=true; en-US-correction=1520 ms/Correction:spelling/match=true; en-US-clean=726 ms/Clean:en-US/match=true; fr-FR-correction=1630 ms/Correction:spelling/match=true; fr-FR-clean=726 ms/Clean:fr-FR/match=true; ka-GE-correction=1500 ms/Correction:punctuation/match=true; ka-GE-clean=723 ms/Clean:ka-GE/match=true; ru-RU-correction=1342 ms/Correction:punctuation/match=true; ru-RU-clean=1833 ms/Correction:spelling/match=false; auto-fr-FR=1069 ms/Clean:fr-FR/match=true; fixed-en-GB-German=771 ms/Unsupported/match=true; auto-unsupported-Japanese=705 ms/Unsupported/match=true. Artifact SHA-256 88116495a5d0093fa4e21010aae0b848faa31ae8bfa40e85fc085eb2a4ac48d8.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: not independently recorded. Recorded before memory point: ceiling=8826750704, tracked model memory=0, loaded count=0; after: ceiling=9251288512, tracked model memory=5404140560, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Raw diagnostic exact-match counts and the 15-second observation threshold do not establish strict production parsing/derivation or linguistic qualification. Semantic review was not established for this attempt. The prompt hash is retained with its stated provenance, not treated as an independently sealed source identity.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Llama-3.2-3B-Instruct-4bit / v221-diagnostic-1

```text
UTC time: 2026-10-06T20:29:56.797638+00:00 to 2026-10-06T20:30:05.920304+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Llama-3.2-3B-Instruct-4bit; attempt v221-diagnostic-1; prompt SHA-256 34a4b474579cf6596ebcde20fdc7967ef3628297658912b145c5f387d237221c (artifact-recorded).
exact results: 5/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, en-GB-correction, en-GB-clean, en-US-correction, fr-FR-correction, ka-GE-correction, ka-GE-clean, ru-RU-correction, auto-fr-FR, fixed-en-GB-German. Returned identity counts: Llama-3.2-3B-Instruct-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=2881 ms/Clean:de-CH/match=false; de-CH-clean=582 ms/Clean:de-CH/match=true; en-GB-correction=408 ms/Unsupported/match=false; en-GB-clean=397 ms/Clean:en-US/match=false; en-US-correction=424 ms/Unsupported/match=false; en-US-clean=435 ms/Clean:en-US/match=true; fr-FR-correction=417 ms/Unsupported/match=false; fr-FR-clean=467 ms/Clean:fr-FR/match=true; ka-GE-correction=453 ms/Unsupported/match=false; ka-GE-clean=454 ms/Unsupported/match=false; ru-RU-correction=427 ms/Unsupported/match=false; ru-RU-clean=466 ms/Clean:ru-RU/match=true; auto-fr-FR=421 ms/Unsupported/match=false; fixed-en-GB-German=441 ms/Clean:en-US/match=false; auto-unsupported-Japanese=422 ms/Unsupported/match=true. Artifact SHA-256 8946f8ed6564ce1758ec8bee84f3d796ace9d4e8d745912d7f59117202bf8070.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10392201400, tracked model memory=0, loaded count=0; after: ceiling=10501508600, tracked model memory=1897871091, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3-4B-Instruct-2507-4bit / v221-diagnostic-1

```text
UTC time: 2026-10-06T20:30:44.833963+00:00 to 2026-10-06T20:31:09.768792+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3-4B-Instruct-2507-4bit; attempt v221-diagnostic-1; prompt SHA-256 34a4b474579cf6596ebcde20fdc7967ef3628297658912b145c5f387d237221c (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: fixed-en-GB-German. Returned identity counts: Qwen3-4B-Instruct-2507-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=5789 ms/Correction:spelling/match=true; de-CH-clean=674 ms/Clean:de-CH/match=true; en-GB-correction=1802 ms/Correction:spelling/match=true; en-GB-clean=621 ms/Clean:en-GB/match=true; en-US-correction=1770 ms/Correction:spelling/match=true; en-US-clean=605 ms/Clean:en-US/match=true; fr-FR-correction=2029 ms/Correction:spelling/match=true; fr-FR-clean=619 ms/Clean:fr-FR/match=true; ka-GE-correction=5156 ms/Correction:punctuation/match=true; ka-GE-clean=683 ms/Clean:ka-GE/match=true; ru-RU-correction=2709 ms/Correction:punctuation/match=true; ru-RU-clean=621 ms/Clean:ru-RU/match=true; auto-fr-FR=624 ms/Clean:fr-FR/match=true; fixed-en-GB-German=623 ms/Clean:en-GB/match=false; auto-unsupported-Japanese=577 ms/Unsupported/match=true. Artifact SHA-256 d9476879663884af044b657017a0c035dc8382d9688298b619fe2c4cc7da0cfe.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10407995552, tracked model memory=0, loaded count=0; after: ceiling=10022014744, tracked model memory=2376173537, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3.5-4B-4bit / v221-diagnostic-1

```text
UTC time: 2026-10-06T20:31:48.265796+00:00 to 2026-10-06T20:32:15.601875+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3.5-4B-4bit; attempt v221-diagnostic-1; prompt SHA-256 34a4b474579cf6596ebcde20fdc7967ef3628297658912b145c5f387d237221c (artifact-recorded).
exact results: 11/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: ka-GE-correction, ka-GE-clean, ru-RU-clean, auto-fr-FR. Returned identity counts: Qwen3.5-4B-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=6320 ms/Correction:spelling/match=true; de-CH-clean=1086 ms/Clean:de-CH/match=true; en-GB-correction=2514 ms/Correction:spelling/match=true; en-GB-clean=1095 ms/Clean:en-GB/match=true; en-US-correction=2453 ms/Correction:spelling/match=true; en-US-clean=1067 ms/Clean:en-US/match=true; fr-FR-correction=2604 ms/Correction:spelling/match=true; fr-FR-clean=1123 ms/Clean:fr-FR/match=true; ka-GE-correction=1043 ms/Unsupported/match=false; ka-GE-clean=1045 ms/Unsupported/match=false; ru-RU-correction=2719 ms/Correction:punctuation/match=true; ru-RU-clean=1047 ms/Unsupported/match=false; auto-fr-FR=1094 ms/Unsupported/match=false; fixed-en-GB-German=1045 ms/Unsupported/match=true; auto-unsupported-Japanese=1041 ms/Unsupported/match=true. Artifact SHA-256 a12218c25f0d0a600b5accf8d5390becbd89effc21a2c2b23ba125ec3e7ffae5.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10006128824, tracked model memory=0, loaded count=0; after: ceiling=10514313416, tracked model memory=3186015729, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: gemma-4-e4b-it-4bit / v221-diagnostic-1

```text
UTC time: 2026-10-06T20:33:34.107382+00:00 to 2026-10-06T20:34:02.948524+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model gemma-4-e4b-it-4bit; attempt v221-diagnostic-1; prompt SHA-256 34a4b474579cf6596ebcde20fdc7967ef3628297658912b145c5f387d237221c (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: ka-GE-correction. Returned identity counts: gemma-4-e4b-it-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=11358 ms/Correction:spelling/match=true; de-CH-clean=741 ms/Clean:de-CH/match=true; en-GB-correction=2213 ms/Correction:spelling/match=true; en-GB-clean=669 ms/Clean:en-GB/match=true; en-US-correction=2195 ms/Correction:spelling/match=true; en-US-clean=729 ms/Clean:en-US/match=true; fr-FR-correction=2353 ms/Correction:spelling/match=true; fr-FR-clean=926 ms/Clean:fr-FR/match=true; ka-GE-correction=879 ms/Clean:ka-GE/match=false; ka-GE-clean=928 ms/Clean:ka-GE/match=true; ru-RU-correction=2143 ms/Correction:punctuation/match=true; ru-RU-clean=895 ms/Clean:ru-RU/match=true; auto-fr-FR=954 ms/Clean:fr-FR/match=true; fixed-en-GB-German=904 ms/Unsupported/match=true; auto-unsupported-Japanese=926 ms/Unsupported/match=true. Artifact SHA-256 c252aff1edd44654e22fba9104be44be8b2c187be632828b8592ede7e9f00778.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10034423896, tracked model memory=0, loaded count=0; after: ceiling=9561552560, tracked model memory=5404140560, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3.5-9B-4bit / v221-diagnostic-1

```text
UTC time: 2026-10-06T20:36:04.400645+00:00 to 2026-10-06T20:36:50.215133+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3.5-9B-4bit; attempt v221-diagnostic-1; prompt SHA-256 34a4b474579cf6596ebcde20fdc7967ef3628297658912b145c5f387d237221c (artifact-recorded).
exact results: 12/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, ka-GE-correction, auto-fr-FR. Returned identity counts: Qwen3.5-9B-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=9835 ms/Correction:punctuation/match=false; de-CH-clean=1906 ms/Clean:de-CH/match=true; en-GB-correction=4475 ms/Correction:spelling/match=true; en-GB-clean=1915 ms/Clean:en-GB/match=true; en-US-correction=4359 ms/Correction:spelling/match=true; en-US-clean=1842 ms/Clean:en-US/match=true; fr-FR-correction=4397 ms/Correction:spelling/match=true; fr-FR-clean=1913 ms/Clean:fr-FR/match=true; ka-GE-correction=1812 ms/Unsupported/match=false; ka-GE-clean=1897 ms/Clean:ka-GE/match=true; ru-RU-correction=4113 ms/Correction:punctuation/match=true; ru-RU-clean=1892 ms/Clean:ru-RU/match=true; auto-fr-FR=1809 ms/Unsupported/match=false; fixed-en-GB-German=1801 ms/Unsupported/match=true; auto-unsupported-Japanese=1805 ms/Unsupported/match=true. Artifact SHA-256 965395d2431518acb1c2d4344abac083f81c9e0d2e8fdab71dd166a3636873e7.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=9898715176, tracked model memory=0, loaded count=0; after: ceiling=9732798392, tracked model memory=6247732125, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3-8B-4bit / v221-diagnostic-1

```text
UTC time: 2026-10-06T20:37:17.980918+00:00 to 2026-10-06T20:37:54.472084+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3-8B-4bit; attempt v221-diagnostic-1; prompt SHA-256 34a4b474579cf6596ebcde20fdc7967ef3628297658912b145c5f387d237221c (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: en-GB-correction. Returned identity counts: Qwen3-8B-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=7328 ms/Correction:spelling/match=true; de-CH-clean=1070 ms/Clean:de-CH/match=true; en-GB-correction=2875 ms/Correction:spelling/match=false; en-GB-clean=1006 ms/Clean:en-GB/match=true; en-US-correction=2881 ms/Correction:spelling/match=true; en-US-clean=999 ms/Clean:en-US/match=true; fr-FR-correction=3140 ms/Correction:spelling/match=true; fr-FR-clean=1007 ms/Clean:fr-FR/match=true; ka-GE-correction=7599 ms/Correction:punctuation/match=true; ka-GE-clean=1082 ms/Clean:ka-GE/match=true; ru-RU-correction=3582 ms/Correction:punctuation/match=true; ru-RU-clean=1014 ms/Clean:ru-RU/match=true; auto-fr-FR=1008 ms/Clean:fr-FR/match=true; fixed-en-GB-German=927 ms/Unsupported/match=true; auto-unsupported-Japanese=921 ms/Unsupported/match=true. Artifact SHA-256 5971435ddf105ac936a8292872deb0e63f9526772542722723db45a52bd4c133.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10219218984, tracked model memory=0, loaded count=0; after: ceiling=10442454200, tracked model memory=4838226932, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Llama-3.2-3B-Instruct-4bit / v221-draft2-diagnostic-1

```text
UTC time: 2026-10-06T20:38:23.921936+00:00 to 2026-10-06T20:38:33.967626+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Llama-3.2-3B-Instruct-4bit; attempt v221-draft2-diagnostic-1; prompt SHA-256 1daeaf7321cab236520eedb482b5822c3ceca7b0ea9cecd609ddf733502925ae (artifact-recorded).
exact results: 3/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, en-GB-correction, en-GB-clean, en-US-correction, fr-FR-correction, fr-FR-clean, ka-GE-correction, ka-GE-clean, ru-RU-correction, ru-RU-clean, auto-fr-FR, fixed-en-GB-German. Returned identity counts: Llama-3.2-3B-Instruct-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=2764 ms/Unsupported/match=false; de-CH-clean=590 ms/Clean:de-CH/match=true; en-GB-correction=501 ms/Unsupported/match=false; en-GB-clean=517 ms/Clean:en-US/match=false; en-US-correction=500 ms/Unsupported/match=false; en-US-clean=516 ms/Clean:en-US/match=true; fr-FR-correction=498 ms/Unsupported/match=false; fr-FR-clean=500 ms/Unsupported/match=false; ka-GE-correction=512 ms/Unsupported/match=false; ka-GE-clean=508 ms/Unsupported/match=false; ru-RU-correction=526 ms/Unsupported/match=false; ru-RU-clean=538 ms/Unsupported/match=false; auto-fr-FR=501 ms/Unsupported/match=false; fixed-en-GB-German=537 ms/Clean:en-GB/match=false; auto-unsupported-Japanese=503 ms/Unsupported/match=true. Artifact SHA-256 d4f94f08b141aae89b483310d50c58a9aa7a80709eeb438d5740b0aab0b7aa36.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10406987984, tracked model memory=0, loaded count=0; after: ceiling=10374598184, tracked model memory=1897871091, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3-4B-Instruct-2507-4bit / v221-draft2-diagnostic-1

```text
UTC time: 2026-10-06T20:38:52.798099+00:00 to 2026-10-06T20:39:25.250679+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3-4B-Instruct-2507-4bit; attempt v221-draft2-diagnostic-1; prompt SHA-256 1daeaf7321cab236520eedb482b5822c3ceca7b0ea9cecd609ddf733502925ae (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: fixed-en-GB-German. Returned identity counts: Qwen3-4B-Instruct-2507-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=5055 ms/Correction:spelling/match=true; de-CH-clean=743 ms/Clean:de-CH/match=true; en-GB-correction=1937 ms/Correction:spelling/match=true; en-GB-clean=750 ms/Clean:en-GB/match=true; en-US-correction=1921 ms/Correction:spelling/match=true; en-US-clean=723 ms/Clean:en-US/match=true; fr-FR-correction=2042 ms/Correction:spelling/match=true; fr-FR-clean=754 ms/Clean:fr-FR/match=true; ka-GE-correction=8020 ms/Correction:punctuation/match=true; ka-GE-clean=761 ms/Clean:ka-GE/match=true; ru-RU-correction=3760 ms/Correction:punctuation/match=true; ru-RU-clean=900 ms/Clean:ru-RU/match=true; auto-fr-FR=1281 ms/Clean:fr-FR/match=true; fixed-en-GB-German=800 ms/Clean:en-GB/match=false; auto-unsupported-Japanese=763 ms/Unsupported/match=true. Artifact SHA-256 8b68e09e094a6ae4a7200cb6e31550a9f857c656da7b33bf9a416039ae2353a4.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10031450320, tracked model memory=0, loaded count=0; after: ceiling=8157687600, tracked model memory=2376173537, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: gemma-4-e4b-it-4bit / v221-draft2-diagnostic-1

```text
UTC time: 2026-10-06T20:39:43.338140+00:00 to 2026-10-06T20:40:08.496648+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model gemma-4-e4b-it-4bit; attempt v221-draft2-diagnostic-1; prompt SHA-256 1daeaf7321cab236520eedb482b5822c3ceca7b0ea9cecd609ddf733502925ae (artifact-recorded).
exact results: 11/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, en-GB-correction, ka-GE-correction, ru-RU-correction. Returned identity counts: gemma-4-e4b-it-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=10924 ms/Clean:de-CH/match=false; de-CH-clean=841 ms/Clean:de-CH/match=true; en-GB-correction=708 ms/Clean:en-GB/match=false; en-GB-clean=692 ms/Clean:en-GB/match=true; en-US-correction=2590 ms/Correction:spelling/match=true; en-US-clean=833 ms/Clean:en-US/match=true; fr-FR-correction=1800 ms/Correction:spelling/match=true; fr-FR-clean=744 ms/Clean:fr-FR/match=true; ka-GE-correction=695 ms/Clean:ka-GE/match=false; ka-GE-clean=778 ms/Clean:ka-GE/match=true; ru-RU-correction=913 ms/Clean:ru-RU/match=false; ru-RU-clean=765 ms/Clean:ru-RU/match=true; auto-fr-FR=956 ms/Clean:fr-FR/match=true; fixed-en-GB-German=881 ms/Unsupported/match=true; auto-unsupported-Japanese=1003 ms/Unsupported/match=true. Artifact SHA-256 4cf68465150cc942d19027ab1c7024a748b70c918cc0b4d17b7ffd243ba6c07d.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=8789510352, tracked model memory=0, loaded count=0; after: ceiling=9680524904, tracked model memory=5404140560, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Llama-3.2-3B-Instruct-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:40:28.340064+00:00 to 2026-10-06T20:40:38.143850+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Llama-3.2-3B-Instruct-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 4/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, en-GB-correction, en-GB-clean, en-US-correction, fr-FR-correction, ka-GE-correction, ka-GE-clean, ru-RU-correction, ru-RU-clean, auto-fr-FR, fixed-en-GB-German. Returned identity counts: Llama-3.2-3B-Instruct-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=2545 ms/Unsupported/match=false; de-CH-clean=538 ms/Clean:de-CH/match=true; en-GB-correction=497 ms/Unsupported/match=false; en-GB-clean=519 ms/Clean:en-US/match=false; en-US-correction=502 ms/Unsupported/match=false; en-US-clean=523 ms/Clean:en-US/match=true; fr-FR-correction=504 ms/Unsupported/match=false; fr-FR-clean=540 ms/Clean:fr-FR/match=true; ka-GE-correction=516 ms/Unsupported/match=false; ka-GE-clean=521 ms/Unsupported/match=false; ru-RU-correction=521 ms/Unsupported/match=false; ru-RU-clean=501 ms/Unsupported/match=false; auto-fr-FR=503 ms/Unsupported/match=false; fixed-en-GB-German=540 ms/Clean:en-GB/match=false; auto-unsupported-Japanese=499 ms/Unsupported/match=true. Artifact SHA-256 128f34b188c28fee1b0d0f1d1d11e11a523f2cd9da1f4b5677e0b11a7393263c.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=9941043264, tracked model memory=0, loaded count=0; after: ceiling=9875787304, tracked model memory=1897871091, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3-4B-Instruct-2507-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:41:12.178409+00:00 to 2026-10-06T20:41:41.300999+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3-4B-Instruct-2507-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: fixed-en-GB-German. Returned identity counts: Qwen3-4B-Instruct-2507-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=5628 ms/Correction:spelling/match=true; de-CH-clean=902 ms/Clean:de-CH/match=true; en-GB-correction=2362 ms/Correction:spelling/match=true; en-GB-clean=782 ms/Clean:en-GB/match=true; en-US-correction=1989 ms/Correction:spelling/match=true; en-US-clean=748 ms/Clean:en-US/match=true; fr-FR-correction=2452 ms/Correction:spelling/match=true; fr-FR-clean=729 ms/Clean:fr-FR/match=true; ka-GE-correction=7407 ms/Correction:punctuation/match=true; ka-GE-clean=723 ms/Clean:ka-GE/match=true; ru-RU-correction=2709 ms/Correction:punctuation/match=true; ru-RU-clean=736 ms/Clean:ru-RU/match=true; auto-fr-FR=655 ms/Clean:fr-FR/match=true; fixed-en-GB-German=656 ms/Clean:en-GB/match=false; auto-unsupported-Japanese=603 ms/Unsupported/match=true. Artifact SHA-256 84ccfc0692b6dab3a797ea7f5669fda4b63612520fdd44557beea28d64963f6a.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=9446959312, tracked model memory=0, loaded count=0; after: ceiling=9835122480, tracked model memory=2376173537, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3-8B-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:41:59.130311+00:00 to 2026-10-06T20:42:33.437578+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3-8B-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: ka-GE-correction. Returned identity counts: Qwen3-8B-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=7635 ms/Correction:spelling/match=true; de-CH-clean=1349 ms/Clean:de-CH/match=true; en-GB-correction=3430 ms/Correction:spelling/match=true; en-GB-clean=1231 ms/Clean:en-GB/match=true; en-US-correction=3126 ms/Correction:spelling/match=true; en-US-clean=1184 ms/Clean:en-US/match=true; fr-FR-correction=3627 ms/Correction:spelling/match=true; fr-FR-clean=1478 ms/Clean:fr-FR/match=true; ka-GE-correction=1404 ms/Clean:ka-GE/match=false; ka-GE-clean=1369 ms/Clean:ka-GE/match=true; ru-RU-correction=3757 ms/Correction:punctuation/match=true; ru-RU-clean=1254 ms/Clean:ru-RU/match=true; auto-fr-FR=1181 ms/Clean:fr-FR/match=true; fixed-en-GB-German=1120 ms/Unsupported/match=true; auto-unsupported-Japanese=1124 ms/Unsupported/match=true. Artifact SHA-256 ca013c8800b7967007b2e8f121084ffbf8e37451d133c9184a68e0c1167d4635.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=9799264464, tracked model memory=0, loaded count=0; after: ceiling=9347437776, tracked model memory=4838226932, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: gemma-4-e4b-it-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:43:01.886215+00:00 to 2026-10-06T20:43:24.713723+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model gemma-4-e4b-it-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 12/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, ka-GE-correction, ru-RU-correction. Returned identity counts: gemma-4-e4b-it-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=8876 ms/Clean:de-CH/match=false; de-CH-clean=725 ms/Clean:de-CH/match=true; en-GB-correction=2239 ms/Correction:spelling/match=true; en-GB-clean=761 ms/Clean:en-GB/match=true; en-US-correction=1587 ms/Correction:spelling/match=true; en-US-clean=681 ms/Clean:en-US/match=true; fr-FR-correction=2340 ms/Correction:spelling/match=true; fr-FR-clean=695 ms/Clean:fr-FR/match=true; ka-GE-correction=664 ms/Clean:ka-GE/match=false; ka-GE-clean=654 ms/Clean:ka-GE/match=true; ru-RU-correction=665 ms/Clean:ru-RU/match=false; ru-RU-clean=661 ms/Clean:ru-RU/match=true; auto-fr-FR=655 ms/Clean:fr-FR/match=true; fixed-en-GB-German=793 ms/Unsupported/match=true; auto-unsupported-Japanese=793 ms/Unsupported/match=true. Artifact SHA-256 8c660c557df763b61f9d317cfb27d1d0dde93cdde700edb10422234bf06bed5c.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=9347811584, tracked model memory=0, loaded count=0; after: ceiling=10070447744, tracked model memory=5404140560, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3.5-9B-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:43:52.464811+00:00 to 2026-10-06T20:44:44.194029+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3.5-9B-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 13/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction, auto-fr-FR. Returned identity counts: Qwen3.5-9B-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=11031 ms/Correction:punctuation/match=false; de-CH-clean=2049 ms/Clean:de-CH/match=true; en-GB-correction=4502 ms/Correction:spelling/match=true; en-GB-clean=2003 ms/Clean:en-GB/match=true; en-US-correction=4396 ms/Correction:spelling/match=true; en-US-clean=1974 ms/Clean:en-US/match=true; fr-FR-correction=4794 ms/Correction:spelling/match=true; fr-FR-clean=2005 ms/Clean:fr-FR/match=true; ka-GE-correction=5024 ms/Correction:punctuation/match=true; ka-GE-clean=2003 ms/Clean:ka-GE/match=true; ru-RU-correction=4062 ms/Correction:punctuation/match=true; ru-RU-clean=2024 ms/Clean:ru-RU/match=true; auto-fr-FR=1929 ms/Unsupported/match=false; fixed-en-GB-German=1930 ms/Unsupported/match=true; auto-unsupported-Japanese=1957 ms/Unsupported/match=true. Artifact SHA-256 3845f3b7498e0682f382a9f718ecfcea7a83324512da1019dad14023394f15ec.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=9842313376, tracked model memory=0, loaded count=0; after: ceiling=10256275480, tracked model memory=6247732125, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: gemma-4-12B-it-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:45:19.271839+00:00 to 2026-10-06T20:46:19.384138+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model gemma-4-12B-it-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 13/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: ka-GE-correction, ru-RU-correction. Returned identity counts: gemma-4-12B-it-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=13199 ms/Correction:spelling/match=true; de-CH-clean=2834 ms/Clean:de-CH/match=true; en-GB-correction=5090 ms/Correction:spelling/match=true; en-GB-clean=2839 ms/Clean:en-GB/match=true; en-US-correction=5074 ms/Correction:spelling/match=true; en-US-clean=2827 ms/Clean:en-US/match=true; fr-FR-correction=5334 ms/Correction:spelling/match=true; fr-FR-clean=2845 ms/Clean:fr-FR/match=true; ka-GE-correction=2849 ms/Clean:ka-GE/match=false; ka-GE-clean=2886 ms/Clean:ka-GE/match=true; ru-RU-correction=2874 ms/Clean:ru-RU/match=false; ru-RU-clean=2876 ms/Clean:ru-RU/match=true; auto-fr-FR=2874 ms/Clean:fr-FR/match=true; fixed-en-GB-German=2824 ms/Unsupported/match=true; auto-unsupported-Japanese=2832 ms/Unsupported/match=true. Artifact SHA-256 b67e08bea993950c11cdc3e688eb56e6e37405bad27dcabde8c4b35ccdc79846.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10485393544, tracked model memory=0, loaded count=0; after: ceiling=11317729952, tracked model memory=7078091486, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3.5-4B-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:47:09.167616+00:00 to 2026-10-06T20:47:35.715206+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3.5-4B-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 10/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: ka-GE-correction, ka-GE-clean, ru-RU-correction, ru-RU-clean, auto-fr-FR. Returned identity counts: Qwen3.5-4B-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=6245 ms/Correction:spelling/match=true; de-CH-clean=1146 ms/Clean:de-CH/match=true; en-GB-correction=2582 ms/Correction:spelling/match=true; en-GB-clean=1147 ms/Clean:en-GB/match=true; en-US-correction=2551 ms/Correction:spelling/match=true; en-US-clean=1130 ms/Clean:en-US/match=true; fr-FR-correction=2775 ms/Correction:spelling/match=true; fr-FR-clean=1157 ms/Clean:fr-FR/match=true; ka-GE-correction=1113 ms/Unsupported/match=false; ka-GE-clean=1103 ms/Unsupported/match=false; ru-RU-correction=1106 ms/Unsupported/match=false; ru-RU-clean=1110 ms/Unsupported/match=false; auto-fr-FR=1109 ms/Unsupported/match=false; fixed-en-GB-German=1103 ms/Unsupported/match=true; auto-unsupported-Japanese=1113 ms/Unsupported/match=true. Artifact SHA-256 18e111894120c138cfd945a7256e20951403cf2861f718e975ce603e8c3413a2.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=11364108544, tracked model memory=0, loaded count=0; after: ceiling=10875572616, tracked model memory=3186015729, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: gemma-3-12b-it-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:48:07.727296+00:00 to 2026-10-06T20:49:16.834025+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model gemma-3-12b-it-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 14/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction. Returned identity counts: gemma-3-12b-it-4bit:14; unavailable:1. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=15002 ms/transport exception/match=false; de-CH-clean=2796 ms/Clean:de-CH/match=true; en-GB-correction=4806 ms/Correction:spelling/match=true; en-GB-clean=2804 ms/Clean:en-GB/match=true; en-US-correction=5321 ms/Correction:spelling/match=true; en-US-clean=2887 ms/Clean:en-US/match=true; fr-FR-correction=6078 ms/Correction:spelling/match=true; fr-FR-clean=3232 ms/Clean:fr-FR/match=true; ka-GE-correction=5904 ms/Correction:punctuation/match=true; ka-GE-clean=3200 ms/Clean:ka-GE/match=true; ru-RU-correction=4801 ms/Correction:punctuation/match=true; ru-RU-clean=2977 ms/Clean:ru-RU/match=true; auto-fr-FR=3209 ms/Clean:fr-FR/match=true; fixed-en-GB-German=3077 ms/Unsupported/match=true; auto-unsupported-Japanese=2957 ms/Unsupported/match=true. Artifact SHA-256 acacef7870f1d1100fd1e3cb09a4c10cdd26d06cd19448847fcd1ed20ae1105e.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=10663643488, tracked model memory=0, loaded count=0; after: ceiling=10846764016, tracked model memory=8430109010, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: Qwen3-14B-4bit / v221-draft3-diagnostic-1

```text
UTC time: 2026-10-06T20:49:54.779588+00:00 to 2026-10-06T20:51:06.224613+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model Qwen3-14B-4bit; attempt v221-draft3-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 14/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction. Returned identity counts: Qwen3-14B-4bit:14; unavailable:1. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=15003 ms/transport exception/match=false; de-CH-clean=3267 ms/Clean:de-CH/match=true; en-GB-correction=6384 ms/Correction:spelling/match=true; en-GB-clean=1905 ms/Clean:en-GB/match=true; en-US-correction=6295 ms/Correction:spelling/match=true; en-US-clean=1898 ms/Clean:en-US/match=true; fr-FR-correction=6536 ms/Correction:spelling/match=true; fr-FR-clean=1782 ms/Clean:fr-FR/match=true; ka-GE-correction=11481 ms/Correction:punctuation/match=true; ka-GE-clean=2164 ms/Clean:ka-GE/match=true; ru-RU-correction=5915 ms/Correction:punctuation/match=true; ru-RU-clean=2232 ms/Clean:ru-RU/match=true; auto-fr-FR=2204 ms/Clean:fr-FR/match=true; fixed-en-GB-German=2205 ms/Unsupported/match=true; auto-unsupported-Japanese=2102 ms/Unsupported/match=true. Artifact SHA-256 3995ce5f1fdcce4be979aa70784d5903ed9e7b037d51287a5662430522833be1.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: True. Recorded before memory point: ceiling=11890854264, tracked model memory=0, loaded count=0; after: ceiling=10796973424, tracked model memory=8723293439, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: gemma-3-12b-it-4bit / v221-draft3-preloaded-diagnostic-1

```text
UTC time: 2026-10-06T20:53:08.750112+00:00 to 2026-10-06T20:54:40.031189+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model gemma-3-12b-it-4bit; attempt v221-draft3-preloaded-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 14/15 diagnostic decision/edit matches; 14/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: de-CH-correction. Returned identity counts: gemma-3-12b-it-4bit:14; unavailable:1. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=15076 ms/transport exception/match=false; de-CH-clean=4035 ms/Clean:de-CH/match=true; en-GB-correction=6660 ms/Correction:spelling/match=true; en-GB-clean=4759 ms/Clean:en-GB/match=true; en-US-correction=9626 ms/Correction:spelling/match=true; en-US-clean=4735 ms/Clean:en-US/match=true; fr-FR-correction=8018 ms/Correction:spelling/match=true; fr-FR-clean=4197 ms/Clean:fr-FR/match=true; ka-GE-correction=7015 ms/Correction:punctuation/match=true; ka-GE-clean=4474 ms/Clean:ka-GE/match=true; ru-RU-correction=5866 ms/Correction:punctuation/match=true; ru-RU-clean=4186 ms/Clean:ru-RU/match=true; auto-fr-FR=4516 ms/Clean:fr-FR/match=true; fixed-en-GB-German=4012 ms/Unsupported/match=true; auto-unsupported-Japanese=3997 ms/Unsupported/match=true. Artifact SHA-256 0ba91b2e2236f828176385eb0e085cd5521d145b10a00e8dffa1709217124b5b.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: False. Recorded before memory point: ceiling=12431711144, tracked model memory=8430109010, loaded count=1; after: ceiling=10749598752, tracked model memory=8430109010, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification. This attempt records a noncold start; the separate preload/warming preparation is retained below.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical draft attempt: gemma-3-12b-it-4bit / v221-draft3-warmed-diagnostic-1

```text
UTC time: 2026-10-06T20:57:30.122458+00:00 to 2026-10-06T20:58:39.094014+00:00
gate or increment: Historical candidate diagnostic only; draft-v221-diagnostic-not-production-qualification
constitution freeze ID: not established; prefreeze/unversioned diagnostic artifact
constitution commit: unavailable; not inferred from later freeze or matching prompt
constitution tree: unavailable; not inferred
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-SEC-003
tested implementation tree: unavailable; nonproduction probe source not locked to an exact Git tree
tested implementation commit: unavailable; do not substitute a later implementation commit
commands or actions: Run the draft shared prompt as a direct synthetic diagnostic; preserve this entire attempt independently of other prompt/model attempts. This is a draft diagnostic, not an official production qualification run. Requested model gemma-3-12b-it-4bit; attempt v221-draft3-warmed-diagnostic-1; prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710 (artifact-recorded).
exact results: 15/15 diagnostic decision/edit matches; 15/15 measured within 15 seconds; 15 sequential cases retained. Failure/mismatch case IDs: none structurally; qualification not established. Returned identity counts: gemma-3-12b-it-4bit:15. Header warning present count=0; no warning body retained. Ordered diagnostic request latencies/outcomes/match flags: de-CH-correction=13291 ms/Correction:spelling/match=true; de-CH-clean=3292 ms/Clean:de-CH/match=true; en-GB-correction=5432 ms/Correction:spelling/match=true; en-GB-clean=3120 ms/Clean:en-GB/match=true; en-US-correction=5362 ms/Correction:spelling/match=true; en-US-clean=3083 ms/Clean:en-US/match=true; fr-FR-correction=5942 ms/Correction:spelling/match=true; fr-FR-clean=3135 ms/Clean:fr-FR/match=true; ka-GE-correction=5470 ms/Correction:punctuation/match=true; ka-GE-clean=3182 ms/Clean:ka-GE/match=true; ru-RU-correction=4763 ms/Correction:punctuation/match=true; ru-RU-clean=3222 ms/Clean:ru-RU/match=true; auto-fr-FR=3152 ms/Clean:fr-FR/match=true; fixed-en-GB-German=3088 ms/Unsupported/match=true; auto-unsupported-Japanese=3376 ms/Unsupported/match=true. Artifact SHA-256 2038f18161b14418562016b6d057282e64ed05d5bd010fbe75c33db70bf4c613.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72. Brave/other desktop activity varied; exact browser version/workload for each earlier run is not independently bound. First-case cold flag: False. Recorded before memory point: ceiling=12401359904, tracked model memory=8430109010, loaded count=1; after: ceiling=11060313120, tracked model memory=8430109010, loaded count=1. Memory values are recorded server observations, not isolated model/process attribution.
toolchain: Saved direct-probe artifact; exact running probe/tool versions unavailable. oMLX 0.7.0.dev4 measured environment, selected catalog ID above; no credential value recorded.
limitations or failures: Exact decision/edit and deadline-match counts are diagnostic observations, not a linguistic/production gate pass. Explanations may remain wrong even when decision/edit matches. Exact tested probe code/tree and committed constitution identity are unavailable. No final 5a identity is assigned. A draft 15/15 diagnostic does not replace the later official sealed qualification. This attempt records a noncold start; the separate preload/warming preparation is retained below.
next checkpoint: Retain this historical attempt and its artifact unchanged. The final official 5a 15/15 run remains a separate record; never retry or replace a historical case in place.
```

### Historical startup measurement: v221-draft3-startup-warmup-1

```text
UTC time: 2026-10-06T20:56:23.605845+00:00 to 2026-10-06T20:56:36.077242+00:00
gate or increment: Historical synthetic startup measurement only; explicit-load-and-synthetic-startup-measurement-not-qualification
constitution freeze ID: not established; prefreeze startup-policy measurement
constitution commit: unavailable; diagnostic source not an official freeze intake
constitution tree: unavailable
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-SEC-003
tested implementation tree: unavailable; direct probe
tested implementation commit: unavailable; direct probe
commands or actions: One separate synthetic startup measurement for gemma-3-12b-it-4bit; attempt v221-draft3-startup-warmup-1; artifact-recorded prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710.
exact results: One diagnostic request measured 6538 ms, Clean:en-GB, HTTP 200 and exact synthetic match true. Separate administrative load latency 5915 ms; explicit-load-plus-synthetic combined measurement 12453 ms. Recorded headers/first-body-byte measurements: unavailable/unavailable ms. Diagnostic ceiling 45,000 ms; ordinary writing measurement threshold 15,000 ms. No inference retries and no persisted configuration change were recorded. Artifact SHA-256 0449916c533d3d8ef9b19b103ae3d8c78978b1dbadceba668a8b8efa9cd96e94.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; varied desktop workload. Before memory point ceiling=11261045088, tracked model memory=0, loaded count=0; after ceiling=11957328928, tracked model memory=8430109010, loaded count=1.
toolchain: Saved direct synthetic probe; exact code/tool identity not independently locked.
limitations or failures: This single synthetic measurement is neither a qualification case nor a latency/residency guarantee. The generous 45-second diagnostic ceiling was measurement only; it was not published as writing policy or the final Settings deadline. The later frozen policy is 25 seconds only for explicit local Settings Test, with writing/corpus 15 seconds unchanged.
next checkpoint: Retain this preparation separately from the warmed draft corpus and final official qualification; do not count it as qualification or silently add preload/retry behavior.
```

### Historical startup measurement: v221-draft3-direct-cold-startup-1

```text
UTC time: 2026-10-06T20:59:53.633621+00:00 to 2026-10-06T21:00:05.995344+00:00
gate or increment: Historical synthetic startup measurement only; direct-cold-synthetic-startup-measurement-not-qualification
constitution freeze ID: not established; prefreeze startup-policy measurement
constitution commit: unavailable; diagnostic source not an official freeze intake
constitution tree: unavailable
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-SEC-003
tested implementation tree: unavailable; direct probe
tested implementation commit: unavailable; direct probe
commands or actions: One separate synthetic startup measurement for gemma-3-12b-it-4bit; attempt v221-draft3-direct-cold-startup-1; artifact-recorded prompt SHA-256 27b10f73f94a6b609dc2730543d18397fc7209de33338cb9f7e8ac8fdff98710.
exact results: One diagnostic request measured 12343 ms, Clean:en-GB, HTTP 200 and exact synthetic match true. No separate administrative preload was performed. Recorded headers/first-body-byte measurements: 10153/10153 ms. Diagnostic ceiling 45,000 ms; ordinary writing measurement threshold 15,000 ms. No inference retries and no persisted configuration change were recorded. Artifact SHA-256 1868ad97c8eee37214807c4ef3cf72f27398a87444e3fe1c0e124893e4110fc4.
evidence level: live
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; varied desktop workload. Before memory point ceiling=11517897056, tracked model memory=0, loaded count=0; after ceiling=12179102752, tracked model memory=8430109010, loaded count=1.
toolchain: Saved direct synthetic probe; exact code/tool identity not independently locked.
limitations or failures: This single synthetic measurement is neither a qualification case nor a latency/residency guarantee. The generous 45-second diagnostic ceiling was measurement only; it was not published as writing policy or the final Settings deadline. The later frozen policy is 25 seconds only for explicit local Settings Test, with writing/corpus 15 seconds unchanged.
next checkpoint: Retain this preparation separately from the warmed draft corpus and final official qualification; do not count it as qualification or silently add preload/retry behavior.
```

# Preserved historical failed production attempts

These seven archived attempts remain separate from final qualification. This fragment records retrospective inspection of sealed synthetic captures; no inference was performed during this verification. Each capture contains all 15 ordered cases and records zero application retries. Later independent reviews, if recorded elsewhere, are outside these capture states. Exact implementation and constitution identities are unavailable in the saved artifacts; none is attributed to the current implementation.

## Historical failed attempt 1: gemma-4-e4b-it-4bit

UTC time: 2026-10-06T19:47:59.124Z through 2026-10-06T19:48:16.955Z
gate or increment: historical failed complete production capture; preserved, not final qualification
constitution freeze ID: unavailable in the saved capture
constitution commit: unavailable in the saved capture
constitution tree: unavailable in the saved capture
critical requirement IDs: not recorded in the saved capture
tested implementation tree: unavailable in the saved capture
tested implementation commit: unavailable in the saved capture
commands or actions: retrospectively verified file SHA-256, canonical capture digest, metadata, 15 ordered case rows, structural match count, exact latencies, and failure reasons against the preserved summary; no new inference or individual case replacement
exact results: sealed capture state Incomplete; structural and exact-string matches 14/15; capture qualification-ready success count 0/15 with independent review pending; elapsed run time 17831 ms; file SHA-256 d45057d2055a420f61f4a6f253db5739a1eab0f7ac233f64dded867ff178e5ea; canonical capture digest b6a809100ac052bf99c4d5c01151cfadbf6c3fa3899360f791e45ab441d03684; summary SHA-256 3345408a8d5d171e7e286a341c9154097eac9f1f741c19433b35f733b7273967; summary-reported shared prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 is not bound into the sealed capture and exact used prompt identity is unavailable from the capture
evidence level: inspected
environment: historical local provider run; exact operating system, browser, device, and server identity not recorded in the saved capture
toolchain: unavailable in the saved capture
limitations or failures: the failed case facts below are preserved; all independent linguistic judgments remain pending within this capture; exact implementation, constitution, and prompt identity cannot be independently established from this artifact
next checkpoint: preserve this failed attempt separately; final qualification and any later independent review require their own evidence entries
provider: localOmlx
requested model: gemma-4-e4b-it-4bit
server or provider build and verified policy: build unavailable; capture metadata affirms documented direct model selection and disabled model fallback, with strictly sequential execution and zero application retries
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: first case 2124 ms, contemporaneous summary reports warm; remaining 14 cases range 704–1947 ms and were labelled warm in that summary; actual residency and memory observations are not recorded in the capture
startup preparation: unavailable in the saved capture
separate startup-test outcome, full-processing latency and actual cold/warm condition: no separate startup-test record is bound into this capture; actual cold/warm condition is unverified; measured complete request latencies are preserved per case below
semantic reviewers (agent/model identities): none recorded in this saved capture
reviewer profile/case coverage: 0/15 at capture sealing; no profiles covered; incomplete
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning

case: de-CH-correction
selected model: gemma-4-e4b-it-4bit
complete request latency: 2124 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: de-CH-clean
selected model: gemma-4-e4b-it-4bit
complete request latency: 724 ms
outcome: Clean:de-CH
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-correction
selected model: gemma-4-e4b-it-4bit
complete request latency: 1519 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-clean
selected model: gemma-4-e4b-it-4bit
complete request latency: 718 ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-correction
selected model: gemma-4-e4b-it-4bit
complete request latency: 1567 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-clean
selected model: gemma-4-e4b-it-4bit
complete request latency: 814 ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-correction
selected model: gemma-4-e4b-it-4bit
complete request latency: 1947 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-clean
selected model: gemma-4-e4b-it-4bit
complete request latency: 838 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-correction
selected model: gemma-4-e4b-it-4bit
complete request latency: 1670 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-clean
selected model: gemma-4-e4b-it-4bit
complete request latency: 754 ms
outcome: Clean:ka-GE
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-correction
selected model: gemma-4-e4b-it-4bit
complete request latency: 1412 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-clean
selected model: gemma-4-e4b-it-4bit
complete request latency: 1563 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-fr-FR
selected model: gemma-4-e4b-it-4bit
complete request latency: 760 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fixed-en-GB-German
selected model: gemma-4-e4b-it-4bit
complete request latency: 704 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-unsupported-Japanese
selected model: gemma-4-e4b-it-4bit
complete request latency: 707 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

success count: 0/15

## Historical failed attempt 2: Qwen3.5-9B-4bit

UTC time: 2026-10-06T19:53:52.511Z through 2026-10-06T19:54:34.544Z
gate or increment: historical failed complete production capture; preserved, not final qualification
constitution freeze ID: unavailable in the saved capture
constitution commit: unavailable in the saved capture
constitution tree: unavailable in the saved capture
critical requirement IDs: not recorded in the saved capture
tested implementation tree: unavailable in the saved capture
tested implementation commit: unavailable in the saved capture
commands or actions: retrospectively verified file SHA-256, canonical capture digest, metadata, 15 ordered case rows, structural match count, exact latencies, and failure reasons against the preserved summary; no new inference or individual case replacement
exact results: sealed capture state Incomplete; structural and exact-string matches 12/15; capture qualification-ready success count 0/15 with independent review pending; elapsed run time 42033 ms; file SHA-256 f7a90ae1e20649a1826f229a0c976c91b685aa92462daf09acaf674a00ed7a85; canonical capture digest 7bb66284d4f9229cc9b368301a1bbce36410fbcb2b0ae7de6037c2c21b1bc90e; summary SHA-256 3345408a8d5d171e7e286a341c9154097eac9f1f741c19433b35f733b7273967; summary-reported shared prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 is not bound into the sealed capture and exact used prompt identity is unavailable from the capture
evidence level: inspected
environment: historical local provider run; exact operating system, browser, device, and server identity not recorded in the saved capture
toolchain: unavailable in the saved capture
limitations or failures: the failed case facts below are preserved; all independent linguistic judgments remain pending within this capture; exact implementation, constitution, and prompt identity cannot be independently established from this artifact
next checkpoint: preserve this failed attempt separately; final qualification and any later independent review require their own evidence entries
provider: localOmlx
requested model: Qwen3.5-9B-4bit
server or provider build and verified policy: build unavailable; capture metadata affirms documented direct model selection and disabled model fallback, with strictly sequential execution and zero application retries
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: first case 9700 ms, contemporaneous summary reports cold; remaining 14 cases range 1468–4708 ms and were labelled warm in that summary; actual residency and memory observations are not recorded in the capture
startup preparation: unavailable in the saved capture
separate startup-test outcome, full-processing latency and actual cold/warm condition: no separate startup-test record is bound into this capture; actual cold/warm condition is unverified; measured complete request latencies are preserved per case below
semantic reviewers (agent/model identities): none recorded in this saved capture
reviewer profile/case coverage: 0/15 at capture sealing; no profiles covered; incomplete
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning

case: de-CH-correction
selected model: Qwen3.5-9B-4bit
complete request latency: 9700 ms
outcome: Correction:punctuation
failure reason: ExpectedResultMismatch
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: de-CH-clean
selected model: Qwen3.5-9B-4bit
complete request latency: 1594 ms
outcome: Clean:de-CH
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-correction
selected model: Qwen3.5-9B-4bit
complete request latency: 4281 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-clean
selected model: Qwen3.5-9B-4bit
complete request latency: 1635 ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-correction
selected model: Qwen3.5-9B-4bit
complete request latency: 4084 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-clean
selected model: Qwen3.5-9B-4bit
complete request latency: 1544 ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-correction
selected model: Qwen3.5-9B-4bit
complete request latency: 4708 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-clean
selected model: Qwen3.5-9B-4bit
complete request latency: 1590 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-correction
selected model: Qwen3.5-9B-4bit
complete request latency: 1507 ms
outcome: Unsupported
failure reason: ExpectedResultMismatch
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-clean
selected model: Qwen3.5-9B-4bit
complete request latency: 1650 ms
outcome: Unsupported
failure reason: ExpectedResultMismatch
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-correction
selected model: Qwen3.5-9B-4bit
complete request latency: 3674 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-clean
selected model: Qwen3.5-9B-4bit
complete request latency: 1561 ms
outcome: Clean:ru-RU
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-fr-FR
selected model: Qwen3.5-9B-4bit
complete request latency: 1556 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fixed-en-GB-German
selected model: Qwen3.5-9B-4bit
complete request latency: 1468 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-unsupported-Japanese
selected model: Qwen3.5-9B-4bit
complete request latency: 1476 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

success count: 0/15

## Historical failed attempt 3: Qwen3-8B-4bit

UTC time: 2026-10-06T20:01:49.046Z through 2026-10-06T20:02:38.166Z
gate or increment: historical failed complete production capture; preserved, not final qualification
constitution freeze ID: unavailable in the saved capture
constitution commit: unavailable in the saved capture
constitution tree: unavailable in the saved capture
critical requirement IDs: not recorded in the saved capture
tested implementation tree: unavailable in the saved capture
tested implementation commit: unavailable in the saved capture
commands or actions: retrospectively verified file SHA-256, canonical capture digest, metadata, 15 ordered case rows, structural match count, exact latencies, and failure reasons against the preserved summary; no new inference or individual case replacement
exact results: sealed capture state Incomplete; structural and exact-string matches 13/15; capture qualification-ready success count 0/15 with independent review pending; elapsed run time 49120 ms; file SHA-256 2a040b26f52924638bb28c5c2de6c701339225e9b21b6ec4990a6c1fa3883fa8; canonical capture digest d67f749e874454f9f4dab54743069da383b9d667e2382751dee02bc84c5d6bc4; summary SHA-256 3345408a8d5d171e7e286a341c9154097eac9f1f741c19433b35f733b7273967; summary-reported shared prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 is not bound into the sealed capture and exact used prompt identity is unavailable from the capture
evidence level: inspected
environment: historical local provider run; exact operating system, browser, device, and server identity not recorded in the saved capture
toolchain: unavailable in the saved capture
limitations or failures: the failed case facts below are preserved; all independent linguistic judgments remain pending within this capture; exact implementation, constitution, and prompt identity cannot be independently established from this artifact
next checkpoint: preserve this failed attempt separately; final qualification and any later independent review require their own evidence entries
provider: localOmlx
requested model: Qwen3-8B-4bit
server or provider build and verified policy: build unavailable; capture metadata affirms documented direct model selection and disabled model fallback, with strictly sequential execution and zero application retries
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: first case 7489 ms, contemporaneous summary reports cold; remaining 14 cases range 969–12397 ms and were labelled warm in that summary; actual residency and memory observations are not recorded in the capture
startup preparation: unavailable in the saved capture
separate startup-test outcome, full-processing latency and actual cold/warm condition: no separate startup-test record is bound into this capture; actual cold/warm condition is unverified; measured complete request latencies are preserved per case below
semantic reviewers (agent/model identities): none recorded in this saved capture
reviewer profile/case coverage: 0/15 at capture sealing; no profiles covered; incomplete
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning

case: de-CH-correction
selected model: Qwen3-8B-4bit
complete request latency: 7489 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: de-CH-clean
selected model: Qwen3-8B-4bit
complete request latency: 3373 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-correction
selected model: Qwen3-8B-4bit
complete request latency: 3183 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-clean
selected model: Qwen3-8B-4bit
complete request latency: 1058 ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-correction
selected model: Qwen3-8B-4bit
complete request latency: 3204 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-clean
selected model: Qwen3-8B-4bit
complete request latency: 1033 ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-correction
selected model: Qwen3-8B-4bit
complete request latency: 3341 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-clean
selected model: Qwen3-8B-4bit
complete request latency: 1079 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-correction
selected model: Qwen3-8B-4bit
complete request latency: 12397 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-clean
selected model: Qwen3-8B-4bit
complete request latency: 1117 ms
outcome: Clean:ka-GE
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-correction
selected model: Qwen3-8B-4bit
complete request latency: 3606 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-clean
selected model: Qwen3-8B-4bit
complete request latency: 5181 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-fr-FR
selected model: Qwen3-8B-4bit
complete request latency: 1111 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fixed-en-GB-German
selected model: Qwen3-8B-4bit
complete request latency: 971 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-unsupported-Japanese
selected model: Qwen3-8B-4bit
complete request latency: 969 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

success count: 0/15

## Historical failed attempt 4: Qwen3-4B-Instruct-2507-4bit

UTC time: 2026-10-06T20:03:19.416Z through 2026-10-06T20:03:55.709Z
gate or increment: historical failed complete production capture; preserved, not final qualification
constitution freeze ID: unavailable in the saved capture
constitution commit: unavailable in the saved capture
constitution tree: unavailable in the saved capture
critical requirement IDs: not recorded in the saved capture
tested implementation tree: unavailable in the saved capture
tested implementation commit: unavailable in the saved capture
commands or actions: retrospectively verified file SHA-256, canonical capture digest, metadata, 15 ordered case rows, structural match count, exact latencies, and failure reasons against the preserved summary; no new inference or individual case replacement
exact results: sealed capture state Incomplete; structural and exact-string matches 11/15; capture qualification-ready success count 0/15 with independent review pending; elapsed run time 36293 ms; file SHA-256 2d2d33793249368976bd3c477d0a81b8053336205812093b23c607a4ee251c2e; canonical capture digest e6cb17c5bcf8a58f5ab3be7c6a179b0653f18c7deaaca26eff160f1320e6ae6b; summary SHA-256 3345408a8d5d171e7e286a341c9154097eac9f1f741c19433b35f733b7273967; summary-reported shared prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 is not bound into the sealed capture and exact used prompt identity is unavailable from the capture
evidence level: inspected
environment: historical local provider run; exact operating system, browser, device, and server identity not recorded in the saved capture
toolchain: unavailable in the saved capture
limitations or failures: the failed case facts below are preserved; all independent linguistic judgments remain pending within this capture; exact implementation, constitution, and prompt identity cannot be independently established from this artifact
next checkpoint: preserve this failed attempt separately; final qualification and any later independent review require their own evidence entries
provider: localOmlx
requested model: Qwen3-4B-Instruct-2507-4bit
server or provider build and verified policy: build unavailable; capture metadata affirms documented direct model selection and disabled model fallback, with strictly sequential execution and zero application retries
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: first case 4208 ms, contemporaneous summary reports cold; remaining 14 cases range 549–8808 ms and were labelled warm in that summary; actual residency and memory observations are not recorded in the capture
startup preparation: unavailable in the saved capture
separate startup-test outcome, full-processing latency and actual cold/warm condition: no separate startup-test record is bound into this capture; actual cold/warm condition is unverified; measured complete request latencies are preserved per case below
semantic reviewers (agent/model identities): none recorded in this saved capture
reviewer profile/case coverage: 0/15 at capture sealing; no profiles covered; incomplete
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning

case: de-CH-correction
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 4208 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: de-CH-clean
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 1794 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-correction
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 1860 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-clean
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 588 ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-correction
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 1659 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-clean
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 569 ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-correction
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 1958 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-clean
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 598 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-correction
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 5830 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-clean
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 8808 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-correction
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 2764 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-clean
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 2290 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-fr-FR
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 593 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fixed-en-GB-German
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 2216 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-unsupported-Japanese
selected model: Qwen3-4B-Instruct-2507-4bit
complete request latency: 549 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

success count: 0/15

## Historical failed attempt 5: Qwen3.5-4B-4bit

UTC time: 2026-10-06T20:04:31.542Z through 2026-10-06T20:04:55.965Z
gate or increment: historical failed complete production capture; preserved, not final qualification
constitution freeze ID: unavailable in the saved capture
constitution commit: unavailable in the saved capture
constitution tree: unavailable in the saved capture
critical requirement IDs: not recorded in the saved capture
tested implementation tree: unavailable in the saved capture
tested implementation commit: unavailable in the saved capture
commands or actions: retrospectively verified file SHA-256, canonical capture digest, metadata, 15 ordered case rows, structural match count, exact latencies, and failure reasons against the preserved summary; no new inference or individual case replacement
exact results: sealed capture state Incomplete; structural and exact-string matches 12/15; capture qualification-ready success count 0/15 with independent review pending; elapsed run time 24423 ms; file SHA-256 74c681773bf707918b7f908d1b45c146a015da33886d5b4baf52d7af56c363bc; canonical capture digest ca0c7e416b2d7e08d9b29a36841016ff780db52440b9e66c84622dd0ef2f6566; summary SHA-256 3345408a8d5d171e7e286a341c9154097eac9f1f741c19433b35f733b7273967; summary-reported shared prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 is not bound into the sealed capture and exact used prompt identity is unavailable from the capture
evidence level: inspected
environment: historical local provider run; exact operating system, browser, device, and server identity not recorded in the saved capture
toolchain: unavailable in the saved capture
limitations or failures: the failed case facts below are preserved; all independent linguistic judgments remain pending within this capture; exact implementation, constitution, and prompt identity cannot be independently established from this artifact
next checkpoint: preserve this failed attempt separately; final qualification and any later independent review require their own evidence entries
provider: localOmlx
requested model: Qwen3.5-4B-4bit
server or provider build and verified policy: build unavailable; capture metadata affirms documented direct model selection and disabled model fallback, with strictly sequential execution and zero application retries
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: first case 6135 ms, contemporaneous summary reports cold; remaining 14 cases range 856–2810 ms and were labelled warm in that summary; actual residency and memory observations are not recorded in the capture
startup preparation: unavailable in the saved capture
separate startup-test outcome, full-processing latency and actual cold/warm condition: no separate startup-test record is bound into this capture; actual cold/warm condition is unverified; measured complete request latencies are preserved per case below
semantic reviewers (agent/model identities): none recorded in this saved capture
reviewer profile/case coverage: 0/15 at capture sealing; no profiles covered; incomplete
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning

case: de-CH-correction
selected model: Qwen3.5-4B-4bit
complete request latency: 6135 ms
outcome: Correction:grammar
failure reason: ExpectedResultMismatch
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: de-CH-clean
selected model: Qwen3.5-4B-4bit
complete request latency: 905 ms
outcome: Clean:de-CH
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-correction
selected model: Qwen3.5-4B-4bit
complete request latency: 2356 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-clean
selected model: Qwen3.5-4B-4bit
complete request latency: 900 ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-correction
selected model: Qwen3.5-4B-4bit
complete request latency: 2184 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-clean
selected model: Qwen3.5-4B-4bit
complete request latency: 895 ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-correction
selected model: Qwen3.5-4B-4bit
complete request latency: 2810 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-clean
selected model: Qwen3.5-4B-4bit
complete request latency: 905 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-correction
selected model: Qwen3.5-4B-4bit
complete request latency: 871 ms
outcome: Unsupported
failure reason: ExpectedResultMismatch
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-clean
selected model: Qwen3.5-4B-4bit
complete request latency: 860 ms
outcome: Unsupported
failure reason: ExpectedResultMismatch
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-correction
selected model: Qwen3.5-4B-4bit
complete request latency: 2067 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-clean
selected model: Qwen3.5-4B-4bit
complete request latency: 903 ms
outcome: Clean:ru-RU
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-fr-FR
selected model: Qwen3.5-4B-4bit
complete request latency: 910 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fixed-en-GB-German
selected model: Qwen3.5-4B-4bit
complete request latency: 859 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-unsupported-Japanese
selected model: Qwen3.5-4B-4bit
complete request latency: 856 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

success count: 0/15

## Historical failed attempt 6: Qwen3-14B-4bit

UTC time: 2026-10-06T20:13:48.820Z through 2026-10-06T20:14:49.736Z
gate or increment: historical failed complete production capture; preserved, not final qualification
constitution freeze ID: unavailable in the saved capture
constitution commit: unavailable in the saved capture
constitution tree: unavailable in the saved capture
critical requirement IDs: not recorded in the saved capture
tested implementation tree: unavailable in the saved capture
tested implementation commit: unavailable in the saved capture
commands or actions: retrospectively verified file SHA-256, canonical capture digest, metadata, 15 ordered case rows, structural match count, exact latencies, and failure reasons against the preserved summary; no new inference or individual case replacement
exact results: sealed capture state Incomplete; structural and exact-string matches 13/15; capture qualification-ready success count 0/15 with independent review pending; elapsed run time 60916 ms; file SHA-256 61cdc4d365f58fdc07d253ac2fc05940e69099b9b662a6153e8aa101bb726fca; canonical capture digest 56d34bac8f21f12013e02cc2bc7ec1e1f80854e764c7469edcfe54615c5293bd; summary SHA-256 3345408a8d5d171e7e286a341c9154097eac9f1f741c19433b35f733b7273967; summary-reported shared prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 is not bound into the sealed capture and exact used prompt identity is unavailable from the capture
evidence level: inspected
environment: historical local provider run; exact operating system, browser, device, and server identity not recorded in the saved capture
toolchain: unavailable in the saved capture
limitations or failures: the failed case facts below are preserved; all independent linguistic judgments remain pending within this capture; exact implementation, constitution, and prompt identity cannot be independently established from this artifact
next checkpoint: preserve this failed attempt separately; final qualification and any later independent review require their own evidence entries
provider: localOmlx
requested model: Qwen3-14B-4bit
server or provider build and verified policy: build unavailable; capture metadata affirms documented direct model selection and disabled model fallback, with strictly sequential execution and zero application retries
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: first case 15004 ms, contemporaneous summary reports cold; remaining 14 cases range 1343–10814 ms and were labelled warm in that summary; actual residency and memory observations are not recorded in the capture
startup preparation: unavailable in the saved capture
separate startup-test outcome, full-processing latency and actual cold/warm condition: no separate startup-test record is bound into this capture; actual cold/warm condition is unverified; measured complete request latencies are preserved per case below
semantic reviewers (agent/model identities): none recorded in this saved capture
reviewer profile/case coverage: 0/15 at capture sealing; no profiles covered; incomplete
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning

case: de-CH-correction
selected model: unavailable
complete request latency: 15004 ms
outcome: Failed
failure reason: DeadlineExceeded
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: de-CH-clean
selected model: Qwen3-14B-4bit
complete request latency: 2665 ms
outcome: Clean:de-CH
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-correction
selected model: Qwen3-14B-4bit
complete request latency: 4070 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-clean
selected model: Qwen3-14B-4bit
complete request latency: 1467 ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-correction
selected model: Qwen3-14B-4bit
complete request latency: 4088 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-clean
selected model: Qwen3-14B-4bit
complete request latency: 1401 ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-correction
selected model: Qwen3-14B-4bit
complete request latency: 4337 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-clean
selected model: Qwen3-14B-4bit
complete request latency: 1473 ms
outcome: Clean:fr-FR
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-correction
selected model: Qwen3-14B-4bit
complete request latency: 10814 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-clean
selected model: Qwen3-14B-4bit
complete request latency: 1569 ms
outcome: Clean:ka-GE
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-correction
selected model: Qwen3-14B-4bit
complete request latency: 4371 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-clean
selected model: Qwen3-14B-4bit
complete request latency: 1515 ms
outcome: Clean:ru-RU
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-fr-FR
selected model: Qwen3-14B-4bit
complete request latency: 5345 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fixed-en-GB-German
selected model: Qwen3-14B-4bit
complete request latency: 1448 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-unsupported-Japanese
selected model: Qwen3-14B-4bit
complete request latency: 1343 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

success count: 0/15

## Historical failed attempt 7: gemma-3-12b-it-4bit

UTC time: 2026-10-06T20:23:30.152Z through 2026-10-06T20:24:38.677Z
gate or increment: historical failed complete production capture; preserved, not final qualification
constitution freeze ID: unavailable in the saved capture
constitution commit: unavailable in the saved capture
constitution tree: unavailable in the saved capture
critical requirement IDs: not recorded in the saved capture
tested implementation tree: unavailable in the saved capture
tested implementation commit: unavailable in the saved capture
commands or actions: retrospectively verified file SHA-256, canonical capture digest, metadata, 15 ordered case rows, structural match count, exact latencies, and failure reasons against the preserved summary; no new inference or individual case replacement
exact results: sealed capture state Incomplete; structural and exact-string matches 9/15; capture qualification-ready success count 0/15 with independent review pending; elapsed run time 68525 ms; file SHA-256 e0f964648508455f1e586ceac36b803f7b5e4258f9be21d8430d9414d03f3a29; canonical capture digest 5764d09e1695a15aa72d41dc779fd1394b441c2fed7c62157333dfb05e17874b; summary SHA-256 3345408a8d5d171e7e286a341c9154097eac9f1f741c19433b35f733b7273967; summary-reported shared prompt SHA-256 28762f30af79e6ca2eef1ce0015ac8bbf3cb0036de725bb4fb5bfdc9dd719810 is not bound into the sealed capture and exact used prompt identity is unavailable from the capture
evidence level: inspected
environment: historical local provider run; exact operating system, browser, device, and server identity not recorded in the saved capture
toolchain: unavailable in the saved capture
limitations or failures: the failed case facts below are preserved; all independent linguistic judgments remain pending within this capture; exact implementation, constitution, and prompt identity cannot be independently established from this artifact
next checkpoint: preserve this failed attempt separately; final qualification and any later independent review require their own evidence entries
provider: localOmlx
requested model: gemma-3-12b-it-4bit
server or provider build and verified policy: build unavailable; capture metadata affirms documented direct model selection and disabled model fallback, with strictly sequential execution and zero application retries
enforced provider plugin policy: not applicable
cold/warm latency and memory observations: first case 15016 ms, contemporaneous summary reports cold; remaining 14 cases range 2073–5126 ms and were labelled warm in that summary; actual residency and memory observations are not recorded in the capture
startup preparation: unavailable in the saved capture
separate startup-test outcome, full-processing latency and actual cold/warm condition: no separate startup-test record is bound into this capture; actual cold/warm condition is unverified; measured complete request latencies are preserved per case below
semantic reviewers (agent/model identities): none recorded in this saved capture
reviewer profile/case coverage: 0/15 at capture sealing; no profiles covered; incomplete
semantic review method: after automated structural and exact-string checks, assess each required profile, correction or clean/unsupported decision, category, explanation, language, and preservation of meaning

case: de-CH-correction
selected model: unavailable
complete request latency: 15016 ms
outcome: Failed
failure reason: DeadlineExceeded
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: de-CH-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 5126 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 4353 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-GB-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 2167 ms
outcome: Clean:en-GB
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 4252 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: en-US-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 2194 ms
outcome: Clean:en-US
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 4674 ms
outcome: Correction:spelling
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fr-FR-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 4281 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 4513 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ka-GE-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 4971 ms
outcome: Correction:spelling
failure reason: ExpectedResultMismatch
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-correction
selected model: gemma-3-12b-it-4bit
complete request latency: 4116 ms
outcome: Correction:punctuation
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: ru-RU-clean
selected model: gemma-3-12b-it-4bit
complete request latency: 4399 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-fr-FR
selected model: gemma-3-12b-it-4bit
complete request latency: 4278 ms
outcome: Failed
failure reason: InvalidModelResult
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: fixed-en-GB-German
selected model: gemma-3-12b-it-4bit
complete request latency: 2107 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

case: auto-unsupported-Japanese
selected model: gemma-3-12b-it-4bit
complete request latency: 2073 ms
outcome: Unsupported
failure reason: none
linguistic correctness (independent agent judgment): pending in this saved capture; independent review not recorded there

success count: 0/15

### Failure retained: one installed Settings Test after server restart

```text
UTC time: 2026-10-07T10:08:30.913Z (one Test start; exact terminal completion time unavailable)
gate or increment: V0.1 Conformance; personal cold recovery remains incomplete
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PROV-001, EM-PROV-002, EM-AUTH-003, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Perform one explicit installed local Settings Test after the separately recorded server restart. No configuration, prompt, model, code or frozen policy change.
exact results: One Test began 2026-10-07T10:08:30.913Z and later showed the redacted server/model-unavailable result, corresponding to ProviderUnavailable. The selected model remained unloaded. Exact full-processing latency and HTTP response status were not captured. The fixed local synthetic 25,000 ms bound remained enforced. Artifact SHA-256 f6e38fc60c8871f9dd2b22f1ac546cfc85fa663cdea34a0e93aa9a16d6da5523. The prior official qualified 15-case capture remains unchanged.
evidence level: runtime
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; final verified native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. Other application/desktop memory load varied; no memory attribution to Emenda alone.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; exact frozen tested/installed source and production build unchanged.
limitations or failures: Do not claim an observed HTTP 507, exact timeout or measured completion latency. One failed cold test does not undo the earlier warm 15/15 semantic qualification, but leaves personal post-server recovery and dependable daily use unproved. No second Test, retry or new inference was performed for this draft.
next checkpoint: Freeze and preserve this failure. Any later recovery must be a separately authorized and separately recorded action.
```

### Read-only diagnosis: current dynamic admission ceiling is below the selected model estimate

```text
UTC time: 2026-10-07T10:15:32.771870Z (saved read-only diagnosis preparation)
gate or increment: V0.1 Conformance; cold-start resource limitation
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PROV-001, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Inspect existing server status and installed memory-guard source after the one failed test; compare recorded model estimate and dynamic admission ceiling. No inference, load/unload, model search, cleanup or settings mutation.
exact results: At the recorded 10:14:12.679049Z memory sample, model resident estimate was 8,430,109,010 bytes and final dynamic admission ceiling was 7,302,595,304 bytes: model-alone excess 1,127,513,706 bytes. Model was neither loaded nor loading. Balanced source formula is own physical footprint + free + inactive + half active memory, with final=min(static,dynamic,Metal) recomputed each call. Static ceiling was 12,884,901,888 bytes and Metal cap 12,713,115,648 bytes; the dynamic limit was binding. An independent nearby OS sample yielded 7,308,903,144 bytes, illustrating snapshot timing variance. Diagnosis artifact SHA-256 798eac34035f33a8516c91f4edafcd7e648c7e5ccbd2cd900328112308253d26.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; final verified native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. Other application/desktop memory load varied; no memory attribution to Emenda alone.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; exact frozen tested/installed source and production build unchanged.
limitations or failures: Insufficient current cold-admission headroom is inferred from present status and installed source; the failed native request HTTP status was not observed. A healthy server and whole-system free percentage do not guarantee admission. No proven task-owned orphan was found; that is not proof unrelated desktop activity is absent. No unrelated process was closed, and no server guard, pin, model, prompt, deadline or configuration was changed.
next checkpoint: Keep this measured limitation visible. The user elected to finish/freeze the process; do not perform further inference or mutate policy to force acceptance.
```

### Completed: hosted CI passed at the exact final implementation

```text
UTC time: 2026-10-07T09:59:55Z (fresh remote completed/success result timestamp)
gate or increment: Architecture and deterministic CI; final tested-state delivery
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PRIV-003, EM-PROV-001, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: Read the fresh hosted CI status for the exact already-published final implementation.
exact results: GitHub Actions run 37603804523 reported success at commit 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63, tree 154341857664f7da22560a6cd4980008b0f46609, with completed/success status, freshly API-verified again by the parent. Historical predecessor remote metadata describes 0df256caffbc0372182eb1127f707fd6a145b228 and is not used to prove this final 5a identity. This records the existing remote result; no new workflow/test execution was started during draft finalization.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; final verified native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. Other application/desktop memory load varied; no memory attribution to Emenda alone.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; exact frozen tested/installed source and production build unchanged.
limitations or failures: Hosted deterministic CI does not establish the unfinished personal cold-start/inference/Apply/Undo/navigation/worker checks or implement the newly requested resource policy.
next checkpoint: Retain the successful CI and exact source identity as part of the frozen checkpoint.
```

### Frozen checkpoint: verified gains and unfinished personal/resource gates

```text
UTC time: 2026-10-07T10:30:06.151961+00:00 (final draft checkpoint assembled from established facts)
gate or increment: V0.1 Conformance incomplete; user-directed end of this execution
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-AUTH-003, EM-PERM-001, EM-PERM-002, EM-PERM-003, EM-PRIV-001, EM-PRIV-002, EM-PRIV-003, EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-APPLY-001, EM-APPLY-002, EM-APPLY-003, EM-SEC-001, EM-SEC-002, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: The user requested that the process finish/freeze as soon as sensible and be summarized. Stop product work, audits, tests, inference and model/configuration changes. Finalize only this sanitized factual checkpoint for the current tested and installed state.
exact results: Frozen implementation identity is 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63/tree 154341857664f7da22560a6cd4980008b0f46609. Completed facts include 411 unit assertions, full clean-clone architecture/build/audit, exact current-binary 122 Brave tests, hosted CI success, byte-identical 14-file personal installed build, one official sequential 15/15 local qualification with both named independent complete semantic reviews, native browser restart, final exact-port enable/revoke observation, server restart/persisted-policy/auth recovery and fresh exact final remote implementation verification. Historical evidence now records 8 preflights, 7 failed production captures, 20 draft corpus diagnostics and 2 separate startup measurements; each retains its own facts and unknown identities explicitly. The qualified capture was not rerun or altered.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; final verified native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64. Other application/desktop memory load varied; no memory attribution to Emenda alone.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; exact frozen tested/installed source and production build unchanged.
limitations or failures: V0.1 Conformance and dependable personal daily-use proof are incomplete. The selected model failed the one post-server cold Settings Test under current memory headroom. Final actual native inference and insertion/deletion/replacement Apply, writer-approved one-step Undo, Dismiss, navigation invalidation, worker restart and installed extension recovery after server restart remain pending. Initial fresh permission grant/denial is historical prior 51 evidence, not a new final-state prompt proof. The newly requested resource-efficiency policy remains unimplemented; no new numeric budget, model default, fallback, background warm-up, guard relaxation or memory guarantee is invented or applied. Observed warm quality and bounded timings do not prove economical/reliable cold operation on every workload of this 16 GB Mac. Historical semantic sidecars remain privately preserved; no new inspection was performed to bind them into earlier pending-review captures. Private operational cleanup/recovery records are excluded.
next checkpoint: Publish only the authorized factual blueprint ledger checkpoint without moving frozen constitution refs or changing source/configuration. Any future resource-policy implementation and remaining native acceptance work require a new authorized continuation; no completion claim is made here.
```

### Final current checkpoint superseding all earlier execution next steps

```text
UTC time: 2026-10-07T10:32:00.922909+00:00 (final checkpoint record; parent’s last state verification was after 10:28Z)
gate or increment: Final frozen checkpoint; ledger-only delivery
constitution freeze ID: emenda-clean-room-v2.2.1-2026-10-06
constitution commit: cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d
constitution tree: 7e232bca9fe2a36c7c20ce5eeb4ba12126e48104
critical requirement IDs: EM-PRIV-003, EM-SEC-003
tested implementation tree: 154341857664f7da22560a6cd4980008b0f46609
tested implementation commit: 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63
commands or actions: At user request, freeze the current tested and installed implementation; inspect only established final delivery facts and prepare this factual ledger. Stop new product work, audits, tests and inference.
exact results: Parent freshly verified both tracked repositories clean and all 14 installed production files byte-identical to the exact 5a build. Fresh API refs confirm implementation 5a154ac6d7b19a4fc856a3c460dbae2ba3c31f63 and blueprint main/docs-v2.2.1-freeze at cd0cfe77356b6d188f6b71c2c6f9307ad2d5c38d; the preserved v2.2.0 freeze remains 11080a3c4be943f57ea35293b995d80453ffecad. Blueprint local main is aligned to the successor before the ledger-only append. Only the verified task-owned synthetic HTTP fixture was stopped; no user application, oMLX inference process or configuration was changed.
evidence level: inspected
environment: Apple M5; 16 GB RAM; macOS 26.6 build 25G72; final native Brave 154.1.96.61/Chromium 154.0.8037.98, arm64.
toolchain: Canonical pinned toolchain and installed 14-file production build at the exact final tested identity.
limitations or failures: This final checkpoint supersedes every earlier continue-testing or continue-inference next step in this ledger. The cold admission/resource-efficiency gate and personal daily-use/native inference/Apply/Undo/navigation/worker/post-server recovery proof remain incomplete. The newly requested resource policy remains unimplemented. No V0.1 completion or dependable daily-use claim is made; no further product audit, test, inference, model search or policy/configuration change is authorized by this frozen execution record.
next checkpoint: Only root may publish the sanitized ledger checkpoint on blueprint main, verify that ledger-only delivery and report where the process is frozen. Frozen constitution refs, implementation source/configuration and the qualified capture stay unchanged. Any future product/resource/native acceptance work requires a separately authorized continuation.
```


### Sprint 4 v2.3.1 Browser Integration: failure preserved and successor completed

This later human-authorized Sprint 4 continuation replaces active freeze governance
with a published versioned specification and exact source provenance. Historical
contracts, execution checkpoints, commits, refs and qualification evidence above
remain preserved. This ledger entry records an already-existing tested implementation;
it does not test or import its own later ledger commit. Section 8 remains pending.

```text
UTC time: 2026-10-07 23:44:36 UTC (ledger assembly after fresh completed/success CI read)
gate or increment: Sprint 4; Acceptance 7 Browser Integration, v2.3.1
constitution version: 2.3.1
constitution commit: f2116e27826642b43f032a70f21afac86bece422
constitution tree: 181b62ce68991df879feed0d8c4da0afd73741e0
original v2.3.0 commit/tree: 429c6e30905c748bafda9d211ec2a2dc6433596a / 9e4e0aed672379cc16a9217f8135924fae34ddfc
critical requirement IDs: EM-AUTH-001, EM-AUTH-002, EM-AUTH-003, EM-PERM-001, EM-PERM-002, EM-PERM-003, EM-PRIV-001, EM-PRIV-002, EM-PRIV-003, EM-PROV-001, EM-PROV-002, EM-PROV-003, EM-APPLY-001, EM-APPLY-002, EM-APPLY-003, EM-SEC-001, EM-SEC-002, EM-SEC-003, EM-RES-001, EM-OPS-001, EM-OPS-002, EM-OPS-003, EM-QUAL-001
tested implementation commit: 17df34772a9631acd0f8796c2ada1c85660d30f6
tested implementation tree: 9ef02c0ebefeab42242f0113f6aed8781339e623
Sprint 3 predecessor commit/tree: 822d1fcbf759e80a8b4251e8ca12e0200ab5fb9c / d8c4586bc3084c8d95d7da12012aaed1e4b46419
commands or actions: npm run audit -- --constitution-only; npm run audit -- --gate browser; npm run audit -- --final --gate browser; hosted npm run audit -- --clean-install --gate browser. Fresh no-hardlinks exact-commit clone ran npm ci, provisioned matching Chromium and completed the full audit. Publish branch build/v0.2, then read back remote commit/tree and successful hosted head identity. Independent complete read-only agent review of the exact final tree found no actionable findings or material blockers.
exact results: 532 deterministic tests across 39 files, 50 Chromium surface tests, 60 shell tests and 44 production extension integration tests passed locally, in the exact-commit clean clone and hosted Windows CI. Strict source record/imported raw tree, 13-document inventory/links, 23 critical-ID mappings, compiler/dependency/asset/manifest/bundle/privacy/CI checks and deterministic linguistic predecessor equivalence passed. Four required permissions: activeTab, scripting, storage, contextMenus. Hosted run 37703308122, attempt 1, completed/success at the tested implementation: https://github.com/duracell04/emenda-built/actions/runs/37703308122.
implemented interaction: stable native Proofread item and FIFO reconciliation; authenticated nontext candidate reservation/pinning; document-targeted, single-use source/generation-bound intent; explicit prefilled capture; unused-token retirement; guarded cold-document Apply; current-action presentation; exact source/caret focus handoff. Typing/lifecycle activity produces zero ambient provider traffic. Apply/Dismiss and insertion/deletion/supplementary replacement with one native Undo pass through the production extension.
IME rule and real fixture: trusted start plus strict native beforeinput/input pairs establish one source/generation; end is a notification regardless of trust. Exact [3,3] -> [3,5] -> [4,6] -> [6,6] fixture delivered trusted start/pairs and compositionend.isTrusted false, with one local committed settling change and zero ambient provider calls. Native pre-edit candidate range [3,6] required a composing-only exact-text/source/generation comparison; ordinary input/provider/mutation rules remain unchanged. A matching page-authored end can close an already native-fed eligible generation once, but cannot create a generation, add terminal text, mint intent, infer or authorize Apply. Duplicate/no-op/cancelled/invalid terminal events remain silent; duplicate end cannot cancel later work.
evidence level: compiled | deterministic | integration | inspected
environment: local macOS 26.6 build 25G72, darwin arm64, matching Chromium 151.0.7922.34 / Playwright revision 1234. Hosted Microsoft Windows Server 2025, 10.0.26100, image windows-2025-vs2026 20260925.250.1, win32 x64; browser Chromium 151.0.7922.34, Playwright revision 1234.
toolchain: Node 20.18.0; npm 11.10.0; TypeScript 5.9.2; Playwright 1.62.1; Vitest 3.2.7; esbuild 0.28.2; Zod 4.1.5; Chrome types 0.2.6; Node types 24.3.0. One browser worker, at most two Vitest workers. Chromium only provisioned; Windows PLAYWRIGHT_BROWSERS_PATH=0 binds rendering and tests to that browser.
failures and recovery: The first clean-clone audit at 89cd7f950d6549b11c722ed2c69d7fe642abc474 / fdf36c99733b3254e347f6c0920cbcf4101ab6e1 passed 532 deterministic, 50 surface and 60 shell tests but reported 43/44 integration passed: a delayed selection notification discarded the exact candidate after neutral element blur. The successor revalidates that retained candidate while retiring ordinary authority; the strengthened delayed-notification fixture passed ten repeats, then complete local and new exact-commit clean-clone audits passed at the successor above. This changed-implementation recovery preserves the earlier failed run. Failed run log SHA-256: c68004213bd5efd8d136bb0c4960ff3e619d279218f26874b55fed225eeb3a18. Successor local log SHA-256: c637dbda669f339178a03db765049f269e340c41eb3bfa993c1c243a29c9d6c4. Successor clean-clone log SHA-256: 23763e0306d02f06ef914d1e4859f79bd7e70b5b575bff4550ab8899eeda3c07. Earlier sandbox application-registration crashes provided no page/extension acceptance evidence; approved outside-sandbox matching-browser runs passed. Initial legacy protocol/ambient fixtures were corrected to explicit protocol 2; the copy fixture initially reused stale error authority. Commit ebf81adea81981ac2574105ff62a4d18a5120a09 incorrectly says that first browser copy fixture passed; actual correction and recovery is retained in 30e2c3c7253db2d84e41a77dad66c5f1fb17e709 and full audits. Independent revocation/duplicate-end findings were fixed and re-reviewed in separate commits.
evidence labels and limitations: Browser-delivered DOM behavior includes trusted ContextMenu-key and native CDP IME paths, production focus/selection/Apply/Undo and BFCache. Native menu-item selection uses an instrumented registered Chrome callback, not physical item selection. Delayed matching selection notification, window-blur invalidation and API/lifecycle failures use deterministic simulation. Historical personal Brave/native OS observations remain historical and receive no new Sprint 4 credit. Hosted Windows receives no personal Mac/Brave credit. Minimum-target Chrome 140 and every supported release were not exercised; current tests identify Chromium 151. Exact v2.3.0 conformance, native OS IME, actual native menu-item selection and real-model acceptance are not claimed. Existing sealed 15/15 local-model qualification and its independent reviews remain inherited linguistic evidence through deterministic equivalence, not a new model run.
resources: 0 real-model requests; 0 oMLX/model configuration changes; 0 added language services/processes. Synthetic intercepted provider routes fail on unexpected live traffic. Owned browser contexts/test fixtures close; temporary clean clone is removed; traces/video/screenshots off by default. Existing cache/dependency sizes are approximately 554/84 MB. One whole-system sample on 16 GiB reported 38% free; no Emenda-only memory or peak guarantee is inferred. No unrelated user process or installed extension path was changed.
next checkpoint: Sprint 5 Section 8 personal installed Conformance. Use the exact hashed production build, record actual Mac/Brave/Chromium/oMLX/model versions and installed identity, then verify toolbar grants/denials, physical native item selection, native spelling/caret/focus/IME, Dismiss/Apply/one Undo, navigation/revocation/restart/storage, zero ambient traffic and actual resources. One bounded explicit local inference smoke follows inexpensive checks. This Sprint ends with publication and sanitized report/build/SHA-256 inventory; it does not perform personal installation or real-model acceptance.
```
