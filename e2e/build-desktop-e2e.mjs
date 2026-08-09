import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repository = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const targetDirectory = join(repository, "src-tauri", "target", "desktop-e2e");
const environment = {
  ...process.env,
  CARGO_TARGET_DIR: targetDirectory,
};

function run(command, args) {
  return new Promise((resolveRun, reject) => {
    const child = spawn(command, args, {
      cwd: repository,
      env: environment,
      stdio: "inherit",
      windowsHide: true,
    });
    child.once("error", reject);
    child.once("exit", (code, signal) => {
      if (code === 0) {
        resolveRun();
      } else {
        reject(
          new Error(
            `${command} exited with code ${String(code)} and signal ${String(signal)}`,
          ),
        );
      }
    });
  });
}

const tauriCli = join(repository, "node_modules", "@tauri-apps", "cli", "tauri.js");
if (!existsSync(tauriCli)) {
  throw new Error(`the local Tauri CLI is missing: ${tauriCli}`);
}

await run(process.execPath, [
  tauriCli,
  "build",
  "--debug",
  "--no-bundle",
  "--features",
  "desktop-e2e",
  "--config",
  "src-tauri/tauri.desktop-e2e.conf.json",
]);
await run("cargo", [
  "build",
  "--locked",
  "--manifest-path",
  "src-tauri/Cargo.toml",
  "--features",
  "desktop-e2e",
  "--bin",
  "emenda-desktop-e2e-fixture",
]);
