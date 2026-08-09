import { spawn } from "node:child_process";
import { createServer } from "node:net";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repository = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const selected = process.argv[2];
const editors = selected === "all" ? ["notepad", "vscode"] : [selected];
if (editors.some((editor) => editor !== "notepad" && editor !== "vscode")) {
  throw new Error("usage: node e2e/run-desktop-e2e.mjs <notepad|vscode|all>");
}

const MAX_ENV_FILE_BYTES = 64 * 1024;
const MIN_SECRET_BYTES = 8;
const MAX_SECRET_BYTES = 4 * 1024;

function parseEnvironmentValue(rawValue) {
  const value = rawValue.trimStart();
  if (!value.startsWith('"') && !value.startsWith("'")) {
    let commentIndex = -1;
    for (let index = 0; index < value.length; index += 1) {
      if (value[index] === "#" && (index === 0 || /\s/.test(value[index - 1]))) {
        commentIndex = index;
        break;
      }
    }
    return (commentIndex < 0 ? value : value.slice(0, commentIndex)).trim();
  }

  const quote = value[0];
  let parsed = "";
  let escaped = false;
  let closingIndex = -1;
  for (let index = 1; index < value.length; index += 1) {
    const character = value[index];
    if (quote === '"' && escaped) {
      const escapeValues = { n: "\n", r: "\r", t: "\t", '"': '"', "\\": "\\" };
      if (!(character in escapeValues)) {
        throw new Error("OPENROUTER_API_KEY contains an unsupported quoted escape");
      }
      parsed += escapeValues[character];
      escaped = false;
    } else if (quote === '"' && character === "\\") {
      escaped = true;
    } else if (character === quote) {
      closingIndex = index;
      break;
    } else {
      parsed += character;
    }
  }
  if (escaped || closingIndex < 0) {
    throw new Error("OPENROUTER_API_KEY has an unterminated quoted value");
  }
  const trailing = value.slice(closingIndex + 1).trim();
  if (trailing && !trailing.startsWith("#")) {
    throw new Error("OPENROUTER_API_KEY has content after its quoted value");
  }
  return parsed;
}

function readOpenRouterKey() {
  const envFile = join(repository, ".env");
  if (!existsSync(envFile)) {
    throw new Error(".env is required and must contain OPENROUTER_API_KEY");
  }
  const metadata = lstatSync(envFile);
  if (metadata.isSymbolicLink() || !metadata.isFile()) {
    throw new Error(".env must be a regular, non-symlink file");
  }
  if (metadata.size <= 0 || metadata.size > MAX_ENV_FILE_BYTES) {
    throw new Error(`.env must be between 1 and ${MAX_ENV_FILE_BYTES} bytes`);
  }
  const encoded = readFileSync(envFile);
  if (encoded.length > MAX_ENV_FILE_BYTES) {
    throw new Error(`.env exceeds the ${MAX_ENV_FILE_BYTES}-byte limit`);
  }
  let contents;
  try {
    contents = new TextDecoder("utf-8", { fatal: true }).decode(encoded);
  } catch {
    throw new Error(".env must contain valid UTF-8");
  }
  const matches = [];
  for (const line of contents.split(/\r?\n/)) {
    const assignment = line.match(/^\s*(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=(.*)$/);
    if (assignment?.[1] === "OPENROUTER_API_KEY") {
      matches.push(parseEnvironmentValue(assignment[2]));
    }
  }
  if (matches.length !== 1) {
    throw new Error(".env must define OPENROUTER_API_KEY exactly once");
  }
  const value = matches[0];
  const byteLength = Buffer.byteLength(value, "utf8");
  if (
    byteLength < MIN_SECRET_BYTES ||
    byteLength > MAX_SECRET_BYTES ||
    /[\u0000-\u001f\u007f-\u009f]/.test(value) ||
    /\s/.test(value)
  ) {
    throw new Error("OPENROUTER_API_KEY in .env has an invalid length or characters");
  }
  return value;
}

function freePort() {
  return new Promise((resolvePort, reject) => {
    const server = createServer();
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      if (!address || typeof address === "string") {
        server.close();
        reject(new Error("could not allocate an embedded WebDriver port"));
        return;
      }
      server.close((error) => (error ? reject(error) : resolvePort(address.port)));
    });
  });
}

const SAFE_ENVIRONMENT_KEYS = [
  "SystemRoot",
  "WINDIR",
  "COMSPEC",
  "PATH",
  "PATHEXT",
  "TEMP",
  "TMP",
  "USERPROFILE",
  "LOCALAPPDATA",
  "APPDATA",
  "ProgramData",
  "ProgramFiles",
  "ProgramFiles(x86)",
  "CommonProgramFiles",
  "CommonProgramFiles(x86)",
  "HOMEDRIVE",
  "HOMEPATH",
  "OS",
  "PROCESSOR_ARCHITECTURE",
  "NUMBER_OF_PROCESSORS",
  "CI",
];

function isolatedEnvironment(overrides) {
  const environment = {};
  for (const name of SAFE_ENVIRONMENT_KEYS) {
    if (process.env[name]) environment[name] = process.env[name];
  }
  return { ...environment, ...overrides };
}

function runWdio(environment, secret) {
  const cli = join(repository, "node_modules", "@wdio", "cli", "bin", "wdio.js");
  return new Promise((resolveRun, reject) => {
    const child = spawn(process.execPath, [cli, "run", "e2e/wdio.conf.ts"], {
      cwd: repository,
      env: environment,
      windowsHide: true,
      stdio: ["ignore", "pipe", "pipe"],
    });
    const chunks = [];
    let byteLength = 0;
    const collect = (chunk) => {
      byteLength += chunk.length;
      if (byteLength > 20 * 1024 * 1024) {
        child.kill();
        reject(new Error("WDIO output exceeded the protected 20 MiB buffer"));
        return;
      }
      chunks.push(chunk);
    };
    child.stdout.on("data", collect);
    child.stderr.on("data", collect);
    child.once("error", reject);
    child.once("exit", (code, signal) => {
      const rawOutput = Buffer.concat(chunks).toString("utf8");
      const leakedSecret = rawOutput.includes(secret);
      const output = rawOutput.split(secret).join("[REDACTED]");
      if (output) process.stdout.write(output);
      if (leakedSecret) {
        reject(new Error("WDIO emitted the exact OpenRouter credential; redacted output was forwarded"));
      } else if (code === 0) {
        resolveRun();
      } else {
        reject(new Error(`WDIO exited with code ${String(code)} and signal ${String(signal)}`));
      }
    });
  });
}

const secret = readOpenRouterKey();
const e2eTarget = join(repository, "src-tauri", "target", "desktop-e2e", "debug");
const application = join(e2eTarget, "emenda.exe");
const fixture = join(
  e2eTarget,
  "emenda-desktop-e2e-fixture.exe",
);
for (const binary of [application, fixture]) {
  if (!existsSync(binary)) throw new Error(`desktop E2E binary is missing: ${binary}`);
}

const failures = [];
for (const editor of editors) {
  let configDirectory;
  try {
    configDirectory = await mkdtemp(join(tmpdir(), `emenda-e2e-${editor}-`));
    const port = await freePort();
    const environment = isolatedEnvironment({
      OPENROUTER_API_KEY: secret,
      EMENDA_E2E_CONFIG_DIR: configDirectory,
      EMENDA_E2E_APP_BINARY: application,
      EMENDA_E2E_FIXTURE_BINARY: fixture,
      EMENDA_E2E_EDITOR: editor,
      TAURI_WEBDRIVER_PORT: String(port),
    });
    process.stdout.write(`Running isolated ${editor} desktop E2E acceptance...\n`);
    await runWdio(environment, secret);
  } catch (error) {
    const failure = error instanceof Error ? error : new Error(String(error));
    failures.push(new Error(`${editor}: ${failure.message}`));
    process.stderr.write(`${editor} desktop E2E failed; continuing to the next one-shot case.\n`);
  } finally {
    if (configDirectory) {
      try {
        await rm(configDirectory, { recursive: true, force: true });
      } catch (error) {
        const failure = error instanceof Error ? error : new Error(String(error));
        failures.push(new Error(`${editor} config cleanup: ${failure.message}`));
        process.stderr.write(
          `${editor} config cleanup failed; continuing to the next one-shot case.\n`,
        );
      }
    }
  }
}

if (failures.length > 0) {
  throw new AggregateError(
    failures,
    `${failures.length} serialized desktop E2E acceptance case(s) failed`,
  );
}
