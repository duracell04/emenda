# Emenda Implementation Evidence

> **Mutable evidence-ledger template for constitution version 2.0.2**

This ledger is not constitutional authority and is excluded from the immutable checksums in `PACKAGE-MANIFEST.md`. It remains empty in the documentation-only v2.0.2 freeze.

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
commands:
exact results:
evidence level: inspected | compiled | deterministic | integration | live | runtime
environment:
limitations or failures:
next checkpoint:
```

Preserve failures and later recoveries as separate entries. Never record credentials, authorization headers, raw private text, page URLs, source identity, DOM structures, or raw provider bodies.

## Live provider evidence extension

For each official Provider Gate case, append only:

```text
case:
selected model: <model ID | unavailable>
latency:
outcome:
failure reason: <reason | none>
linguistic correctness:
```

After the 15 sequential cases, report `success count: x/15`. Do not retry or replace an official case. Record any diagnostic rerun separately without changing the original result.

## Browser evidence extension

For browser or device evidence, append only the relevant fields:

```text
browser and exact version:
operating system and version:
device:
extension commit:
tester:
timestamp:
checklist results:
failures or limitations:
```

## Evidence entries

No implementation evidence is recorded in the documentation-only v2.0.2 freeze.
