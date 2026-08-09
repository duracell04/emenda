# Windows local test signing

Emenda can produce Authenticode-signed V0.1 artifacts with a machine-local,
self-signed test certificate. This workflow proves artifact integrity and
possession of the local signing key. It does **not** establish a publicly
verified publisher identity, build Microsoft SmartScreen reputation, or make
the artifacts trusted on another computer.

The certificate stays in the current user's Personal certificate store and has
a non-exportable private key. No private-key file or machine-specific
thumbprint is configured in tracked repository files. Tauri receives the
thumbprint through a temporary configuration file that is deleted after every
build.

## Prerequisites

- A working MSVC Rust target and Tauri Windows build.
- A complete Windows SDK containing Microsoft-signed `signtool.exe`.
- Network access to DigiCert's RFC 3161 timestamp service.
- An ordinary, non-elevated PowerShell session. The workflow never modifies a
  `LocalMachine` certificate store.
- No copy of the Emenda test certificate in the current user's Root or
  TrustedPublisher store. Certification refuses pre-existing trust so a prior
  interrupted run cannot be mistaken for temporary trust that was cleaned up.

## Provision the local certificate

Run the provisioning script explicitly and keep its thumbprint only in the
current PowerShell process:

```powershell
$env:EMENDA_WINDOWS_SIGNING_THUMBPRINT = npm run --silent tauri:signing:certificate:test
```

The script creates `CN=Emenda V0.1 Local Test Signing` in
`Cert:\CurrentUser\My` with a non-exportable RSA-2048 key, Code Signing EKU,
SHA-256 certificate signature, and one-year lifetime. A later invocation reuses
the one matching valid certificate. It fails instead of guessing when duplicate
or incompatible certificates exist.

The certificate is retained in `CurrentUser\My` so later V0.1 builds use the
same local signer. Delete it and its private key only as a deliberate
certificate-management action when local test signing is no longer needed.

## Build and verify

```powershell
npm run tauri:build:signed:test
```

The npm command applies `ExecutionPolicy Bypass` only to its child PowerShell
process so the checked-in local script can run on machines whose default policy
blocks unsigned development scripts. It does not change the user's persisted
execution policy.

For a read-only prerequisite check after the certificate and SDK exist:

```powershell
npm run tauri:build:signed:test -- -PreflightOnly
```

The signed build:

1. Validates the exact current-user certificate and the newest Microsoft-signed
   x64 SignTool in the installed Windows SDK. An inherited
   `TAURI_WINDOWS_SIGNTOOL_PATH` cannot select a different tool.
2. Requires a clean Git worktree, excluding normal ignored build outputs, so
   the manifest corresponds to committed source.
3. Merges a temporary Tauri config selecting that thumbprint, SHA-256 artifact
   digests, DigiCert RFC 3161 timestamping, and the checked-in fail-closed
   SignTool helper.
4. Builds the release executable, NSIS installer, and MSI installer. Tauri
   restores its unsigned base executable after each installer variant, so the
   helper captures the signed NSIS-marked executable immediately before NSIS
   packages it. The build republishes that exact byte stream as the release
   EXE; this is what makes the later installed-payload SHA-256 comparison
   meaningful.
5. Requires the expected signer and a DigiCert timestamp certificate carrying
   the Time Stamping EKU on every artifact.
6. Temporarily imports only the public certificate into
   `Cert:\CurrentUser\Root` and `Cert:\CurrentUser\TrustedPublisher`.
7. Requires `Get-AuthenticodeSignature` and `signtool verify /pa /all /v /tw`
   to validate every artifact, then proves a modified copy fails verification.
8. Removes only the trust entries it added and deletes every temporary file in
   a `finally` block.

Timestamping is mandatory. A timestamp outage, missing timestamp, signing
warning, signer mismatch, or cleanup failure fails the command. There is no
fallback to an accepted untimestamped build.

Successful artifacts are under `src-tauri/target/release/` and its `bundle/`
subdirectories. The ignored
`bundle/emenda-test-signing-manifest.json` records their post-signing SHA-256
hashes and public signature metadata, including signer and timestamp
thumbprints.

## Install and certify the current-user NSIS package

The build step does not install anything. First run the read-only install
preflight:

```powershell
npm run tauri:install:signed:test -- -PreflightOnly
```

The preflight validates the manifest schema, all three artifact paths, lengths,
SHA-256 hashes, exact signer, and embedded timestamp before any trust or install
mutation. It also requires a non-elevated session and refuses to continue if an
Emenda process, install directory, Start-menu entry, Desktop shortcut, uninstall
registration, or exact Root/TrustedPublisher trust entry already exists.

After the preflight passes, install and certify the NSIS package:

```powershell
npm run tauri:install:signed:test
```

This command:

1. Revalidates all signed release artifacts and temporarily trusts only the
   exact public test certificate in the two current-user stores.
2. Runs only the manifest's NSIS installer in silent current-user mode. It
   never invokes the MSI and rejects any configuration that selects a
   per-machine install mode.
3. Requires `%LOCALAPPDATA%\Emenda\emenda.exe` and `uninstall.exe`, one exact
   current-user NSIS uninstall registration, and no machine/MSI registration.
   Its product, version, install path, and uninstall command metadata must match
   V0.1. The install directory must contain exactly those two production files.
4. Verifies both installed files with Authenticode and
   `signtool verify /pa /all /v /tw`, including the expected signer and
   timestamp.
5. Requires the current user's expected Start-menu and Desktop shortcuts to
   resolve to the installed Emenda executable without launching it.
6. Requires the installed executable's SHA-256 to equal the signed release EXE
   recorded in the manifest.
7. Removes the exact temporary Root and TrustedPublisher entries in `finally`,
   then rechecks the installed signatures and payload hash after cleanup.

The certification script never uninstalls, replaces, or deletes an existing
installation. If the installer succeeds but a later verification fails, it
leaves that installation intact as failure evidence and the next run fails on
the conflict. Resolve such a partial installation deliberately before starting
a new certification attempt.

## Trust semantics after verification

While the two temporary current-user trust entries exist, Windows must report
the signatures as `Valid`. After cleanup, Windows may correctly report the
self-signed chain as untrusted. That does not remove the signature: the build
script rechecks that each artifact still has the exact signer, intact hash, and
timestamp produced by the RFC 3161-configured signing path after cleanup.

The Personal-store certificate remains available for future signing but does
not make the publisher publicly trusted. Public distribution requires a
CA-backed code-signing identity or a managed service such as Azure Artifact
Signing.

References:

- [Tauri Windows code signing](https://v2.tauri.app/distribute/sign/windows/)
- [Microsoft SignTool](https://learn.microsoft.com/windows/win32/seccrypto/signtool)
- [DigiCert RFC 3161 timestamp service](https://knowledge.digicert.com/general-information/rfc3161-compliant-time-stamp-authority-server)
