# Emenda Implementation Evidence

> **Mutable evidence-ledger template for constitution version 2.0.3**

This ledger is not constitutional authority and is excluded from the immutable checksums in `PACKAGE-MANIFEST.md`. It remains empty in the documentation-only v2.0.3 freeze.

Implementation evidence may be added only under a separately authorized future implementation objective. Each entry identifies an already-existing implementation commit that was actually tested. The later evidence commit records that fact; it does not claim to have tested itself.

## Baseline template

```text
constitution version:
freeze ID:
constitution commit:
implementation objective:
UTC time:
environment:
limitations:
```

## Evidence entry template

```text
UTC time:
gate or increment:
tested implementation tree:
tested implementation commit:
commands or actions:
exact results:
evidence level: inspected | compiled | deterministic | integration | live | runtime
environment:
limitations or failures:
next checkpoint:
```

Preserve failures and later recoveries as separate entries. Never record credentials, authorization headers, raw private text, page URLs, tab/frame/document metadata, source identity, DOM structures, or raw provider bodies.

## Live provider evidence extension

For each complete Provider Gate run, append once:

```text
requested model:
enforced provider plugin policy: none
```

For each official case in that run, append only:

```text
case:
selected model: <model ID | unavailable>
complete request latency:
outcome:
failure reason: <reason | none>
linguistic correctness:
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

No implementation evidence is recorded in the documentation-only v2.0.3 freeze.
