import { spawn } from "node:child_process";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repository = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const manifest = join(repository, "src-tauri", "Cargo.toml");
const envFile = join(repository, ".env");
const MAX_ENV_BYTES = 64 * 1024;
const MAX_OUTPUT_BYTES = 8 * 1024 * 1024;

const TESTS = Object.freeze({
  openrouter: {
    label: "strict live OpenRouter provider test",
    cargoArguments: [
      "test",
      "--locked",
      "--manifest-path",
      manifest,
      "--lib",
      "inference::openrouter::tests::live_openrouter_flow",
      "--",
      "--ignored",
      "--exact",
      "--nocapture",
      "--test-threads=1",
    ],
  },
  notepad: {
    label: "strict native Notepad smoke test",
    cargoArguments: [
      "test",
      "--locked",
      "--manifest-path",
      manifest,
      "--test",
      "windows_desktop_smoke",
      "corrects_selected_text_in_notepad",
      "--",
      "--ignored",
      "--exact",
      "--nocapture",
      "--test-threads=1",
    ],
  },
  vscode: {
    label: "strict native VS Code smoke test",
    cargoArguments: [
      "test",
      "--locked",
      "--manifest-path",
      manifest,
      "--test",
      "windows_desktop_smoke",
      "corrects_selected_text_in_vscode",
      "--",
      "--ignored",
      "--exact",
      "--nocapture",
      "--test-threads=1",
    ],
  },
});

const SAFE_ENVIRONMENT_KEYS = [
  "SystemRoot",
  "WINDIR",
  "COMSPEC",
  "PATH",
  "PATHEXT",
  "TEMP",
  "TMP",
  "USERPROFILE",
  "HOMEDRIVE",
  "HOMEPATH",
  "LOCALAPPDATA",
  "APPDATA",
  "ProgramData",
  "ProgramFiles",
  "ProgramFiles(x86)",
  "ProgramW6432",
  "CommonProgramFiles",
  "CommonProgramFiles(x86)",
  "CommonProgramW6432",
  "OS",
  "PROCESSOR_ARCHITECTURE",
  "NUMBER_OF_PROCESSORS",
  "CARGO_HOME",
  "RUSTUP_HOME",
];

function environmentValue(name) {
  const actualName = Object.keys(process.env).find(
    (candidate) => candidate.toLowerCase() === name.toLowerCase(),
  );
  return actualName === undefined ? undefined : process.env[actualName];
}

function parseAssignmentValue(rawValue) {
  const value = rawValue.trim();
  if (value.startsWith("\"") || value.startsWith("'")) {
    const quote = value[0];
    const closingQuote = value.indexOf(quote, 1);
    if (closingQuote === -1 || !/^\s*(?:#.*)?$/.test(value.slice(closingQuote + 1))) {
      throw new Error("OPENROUTER_API_KEY in .env has malformed quoting");
    }
    return value.slice(1, closingQuote);
  }
  return value.replace(/\s+#.*$/, "").trim();
}

function readOpenRouterKey() {
  if (!existsSync(envFile)) {
    throw new Error(".env is required and must contain OPENROUTER_API_KEY");
  }
  const metadata = lstatSync(envFile);
  if (!metadata.isFile() || metadata.isSymbolicLink()) {
    throw new Error(".env must be a regular, non-symbolic-link file");
  }
  if (metadata.size > MAX_ENV_BYTES) {
    throw new Error(".env exceeds the protected 64 KiB input limit");
  }

  const assignments = [];
  const contents = readFileSync(envFile, "utf8").replace(/^\uFEFF/, "");
  for (const line of contents.split(/\r?\n/)) {
    const match = line.match(/^\s*(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=([\s\S]*)$/);
    if (match?.[1] === "OPENROUTER_API_KEY") {
      assignments.push(parseAssignmentValue(match[2]));
    }
  }

  if (assignments.length !== 1) {
    throw new Error(".env must define OPENROUTER_API_KEY exactly once");
  }
  const secret = assignments[0];
  if (secret.length < 16 || secret.length > 1024 || /[\s\u0000-\u001f\u007f]/.test(secret)) {
    throw new Error("OPENROUTER_API_KEY in .env is empty or malformed");
  }
  return secret;
}

function cargoExecutable() {
  const userProfile = environmentValue("USERPROFILE");
  if (!userProfile || !isAbsolute(userProfile)) {
    throw new Error("USERPROFILE must be an absolute path to resolve Cargo safely");
  }
  const executable = join(userProfile, ".cargo", "bin", "cargo.exe");
  if (!existsSync(executable) || !lstatSync(executable).isFile()) {
    throw new Error("Cargo was not found at the standard per-user Rustup location");
  }
  return executable;
}

function cargoEnvironment(secret) {
  const environment = {};
  for (const name of SAFE_ENVIRONMENT_KEYS) {
    const value = environmentValue(name);
    if (value !== undefined && value !== "") environment[name] = value;
  }
  return {
    ...environment,
    CARGO_TERM_COLOR: "never",
    RUST_BACKTRACE: "0",
    OPENROUTER_API_KEY: secret,
  };
}

function redact(output, secret) {
  return output.split(secret).join("[REDACTED]");
}

function runCargoOnce(executable, cargoArguments, environment, secret) {
  return new Promise((resolveRun, reject) => {
    const child = spawn(executable, cargoArguments, {
      cwd: repository,
      env: environment,
      windowsHide: true,
      stdio: ["ignore", "pipe", "pipe"],
    });
    const stdout = [];
    const stderr = [];
    let byteLength = 0;
    let exceededLimit = false;

    const collect = (destination) => (chunk) => {
      if (exceededLimit) return;
      byteLength += chunk.length;
      if (byteLength > MAX_OUTPUT_BYTES) {
        // Stop retaining output, but let the test unwind so its owned editor
        // session can perform deterministic cleanup before this run fails.
        exceededLimit = true;
        return;
      }
      destination.push(chunk);
    };
    child.stdout.on("data", collect(stdout));
    child.stderr.on("data", collect(stderr));

    child.once("error", () => {
      // Keep the diagnostic constant: spawn arguments and environment values
      // (including the credential) must never be interpolated into output.
      reject(new Error("could not start Cargo"));
    });
    child.once("close", (code, signal) => {
      const rawStdout = Buffer.concat(stdout).toString("utf8");
      const rawStderr = Buffer.concat(stderr).toString("utf8");
      const leakedSecret = rawStdout.includes(secret) || rawStderr.includes(secret);
      const safeStdout = redact(rawStdout, secret);
      const safeStderr = redact(rawStderr, secret);
      if (safeStdout) process.stdout.write(safeStdout);
      if (safeStderr) process.stderr.write(safeStderr);

      if (exceededLimit) {
        reject(new Error("Cargo output exceeded the protected 8 MiB buffer"));
      } else if (leakedSecret) {
        reject(
          new Error(
            "Cargo emitted the exact OpenRouter credential; only redacted output was forwarded",
          ),
        );
      } else if (code !== 0) {
        reject(
          new Error(
            `Cargo exited with code ${String(code)} and signal ${String(signal)}`,
          ),
        );
      } else {
        resolveRun();
      }
    });
  });
}

if (process.platform !== "win32") {
  throw new Error("strict live Emenda certification runners require Windows");
}

const [selected, option, ...unexpected] = process.argv.slice(2);
const test = TESTS[selected];
if (!test || unexpected.length > 0 || (option !== undefined && option !== "--preflight")) {
  throw new Error(
    "usage: node scripts/run-strict-live-test.mjs <openrouter|notepad|vscode> [--preflight]",
  );
}
if (!existsSync(manifest)) throw new Error("src-tauri/Cargo.toml is missing");

const executable = cargoExecutable();
const secret = readOpenRouterKey();
const environment = cargoEnvironment(secret);

if (option === "--preflight") {
  process.stdout.write(`Preflight passed for ${test.label}; no test process was started.\n`);
} else {
  process.stdout.write(`Running ${test.label} as one exact, ignored Cargo test...\n`);
  await runCargoOnce(executable, test.cargoArguments, environment, secret);
}
