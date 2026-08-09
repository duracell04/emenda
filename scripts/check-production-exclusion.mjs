import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repository = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const forbidden = [
  "wdioTauri",
  "__wdio_mocks__",
  "plugin:wdio",
  "tauri-plugin-wdio",
  "wdio-webdriver",
  "TAURI_WEBDRIVER_PORT",
  "emenda-desktop-e2e-fixture",
  "desktop-e2e-main-capability",
  "@wdio/",
  "tauri_plugin_wdio",
  "wdio_webdriver",
  "desktop-e2e",
  "desktop_e2e",
  "EMENDA_E2E_CONFIG_DIR",
  "dist-desktop-e2e",
];

function fail(message) {
  throw new Error(`production exclusion check failed: ${message}`);
}

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repository,
    encoding: "utf8",
    windowsHide: true,
    maxBuffer: 20 * 1024 * 1024,
  });
  if (result.status !== 0) {
    fail(`${command} ${args.join(" ")} returned ${String(result.status)}\n${result.stderr}`);
  }
  return result.stdout;
}

function assertNoForbidden(label, value) {
  for (const token of forbidden) {
    if (value.includes(token)) fail(`${label} contains ${token}`);
  }
}

const packageJson = JSON.parse(readFileSync(join(repository, "package.json"), "utf8"));
for (const dependency of Object.keys(packageJson.dependencies ?? {})) {
  if (dependency.startsWith("@wdio/")) {
    fail(`${dependency} is a production JavaScript dependency`);
  }
}

const cargoManifest = readFileSync(join(repository, "src-tauri", "Cargo.toml"), "utf8");
const featuresSection = cargoManifest.match(/^\[features\]\s*$([\s\S]*?)(?=^\[)/m)?.[1];
if (!featuresSection || !/^default\s*=\s*\[\s*\]\s*$/m.test(featuresSection)) {
  fail("Cargo default features must be explicitly empty");
}
for (const crate of ["tauri-plugin-wdio", "tauri-plugin-wdio-webdriver", "tempfile"]) {
  const escaped = crate.replaceAll("-", "\\-");
  const declaration = cargoManifest.match(new RegExp(`^${escaped}\\s*=\\s*\\{([^}]*)\\}`, "m"));
  if (!declaration || !/optional\s*=\s*true/.test(declaration[1])) {
    fail(`${crate} is not declared as an optional Rust dependency`);
  }
}

const baseConfig = JSON.parse(
  readFileSync(join(repository, "src-tauri", "tauri.conf.json"), "utf8"),
);
if (baseConfig.app?.withGlobalTauri === true) {
  fail("the production Tauri configuration enables withGlobalTauri");
}
if (
  JSON.stringify(baseConfig.app?.security?.capabilities) !==
  JSON.stringify(["main-capability"])
) {
  fail("the production Tauri configuration must select only main-capability");
}
const capabilityDirectory = join(repository, "src-tauri", "capabilities");
for (const file of readdirSync(capabilityDirectory)) {
  if (file.endsWith(".json")) {
    assertNoForbidden(
      `production capability ${file}`,
      readFileSync(join(capabilityDirectory, file), "utf8"),
    );
  }
}

const cargoTree = run("cargo", [
  "tree",
  "--locked",
  "--manifest-path",
  "src-tauri/Cargo.toml",
  "--edges",
  "normal",
]);
assertNoForbidden("default Cargo dependency graph", cargoTree);

const desktopE2eCargoTree = run("cargo", [
  "tree",
  "--locked",
  "--manifest-path",
  "src-tauri/Cargo.toml",
  "--features",
  "desktop-e2e",
  "--edges",
  "normal",
]);
for (const crate of ["tauri-plugin-wdio", "tauri-plugin-wdio-webdriver"]) {
  const escaped = crate.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const packageLine = new RegExp(`${escaped}\\s+v\\d`);
  if (!desktopE2eCargoTree.split(/\r?\n/).some((line) => packageLine.test(line))) {
    fail(`desktop-e2e Cargo dependency graph does not include ${crate}`);
  }
}

const npmCli =
  process.env.npm_execpath ??
  join(dirname(process.execPath), "node_modules", "npm", "bin", "npm-cli.js");
const productionNpmTree = run(process.execPath, [
  npmCli,
  "ls",
  "--omit=dev",
  "--all",
  "--json",
]);
assertNoForbidden("production npm dependency graph", productionNpmTree);

const artifactRoots = [
  join(repository, "dist"),
  join(repository, "src-tauri", "target", "release", "emenda.exe"),
  join(repository, "src-tauri", "target", "release", "bundle"),
];
const forbiddenReleaseFixture = join(
  repository,
  "src-tauri",
  "target",
  "release",
  "emenda-desktop-e2e-fixture.exe",
);
if (existsSync(forbiddenReleaseFixture)) {
  fail(`test fixture exists in the production target: ${forbiddenReleaseFixture}`);
}
for (const artifact of artifactRoots) {
  if (!existsSync(artifact)) {
    fail(`required production artifact is missing: ${artifact}; run a production bundle build first`);
  }
}

function artifactFiles(path) {
  if (statSync(path).isFile()) return [path];
  return readdirSync(path, { withFileTypes: true }).flatMap((entry) =>
    artifactFiles(join(path, entry.name)),
  );
}

for (const root of artifactRoots) {
  for (const file of artifactFiles(root)) {
    const normalizedPath = file.replaceAll("\\", "/").toLowerCase();
    for (const token of forbidden) {
      if (normalizedPath.includes(token.toLowerCase())) {
        fail(`production artifact path ${file} contains ${token}`);
      }
    }
    const bytes = readFileSync(file);
    for (const token of forbidden) {
      if (bytes.includes(Buffer.from(token, "utf8")) || bytes.includes(Buffer.from(token, "utf16le"))) {
        fail(`${file} embeds ${token}`);
      }
    }
  }
}

process.stdout.write("Production graphs, frontend output, executable, and bundles exclude desktop E2E code.\n");
