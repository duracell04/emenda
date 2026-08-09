import type { TauriCapabilities, TauriServiceOptions } from "@wdio/tauri-service";

function requiredEnvironment(name: string): string {
  const value = process.env[name];
  if (!value) {
    throw new Error(`${name} is required by the desktop E2E runner`);
  }
  return value;
}

const application = requiredEnvironment("EMENDA_E2E_APP_BINARY");
const embeddedPort = Number.parseInt(requiredEnvironment("TAURI_WEBDRIVER_PORT"), 10);
if (!Number.isInteger(embeddedPort) || embeddedPort <= 0 || embeddedPort > 65_535) {
  throw new Error("TAURI_WEBDRIVER_PORT must be a valid non-zero TCP port");
}

const appEnvironment = {
  OPENROUTER_API_KEY: requiredEnvironment("OPENROUTER_API_KEY"),
  EMENDA_E2E_CONFIG_DIR: requiredEnvironment("EMENDA_E2E_CONFIG_DIR"),
  TAURI_WEBDRIVER_PORT: String(embeddedPort),
};

const serviceOptions: TauriServiceOptions = {
  appBinaryPath: application,
  driverProvider: "embedded",
  embeddedPort,
  env: appEnvironment,
  windowLabel: "main",
  commandTimeout: 30_000,
  startTimeout: 90_000,
  statusPollTimeout: 5_000,
  captureBackendLogs: false,
  captureFrontendLogs: false,
  logLevel: "warn",
};

const capability: TauriCapabilities = {
  browserName: "tauri",
  "tauri:options": {
    application,
  },
  "wdio:tauriServiceOptions": serviceOptions,
};

export const config: WebdriverIO.Config = {
  runner: "local",
  specs: ["./specs/desktop-correction.spec.ts"],
  maxInstances: 1,
  capabilities: [capability],
  services: [["@wdio/tauri-service", serviceOptions]],
  framework: "mocha",
  reporters: ["spec"],
  logLevel: "warn",
  bail: 1,
  waitforTimeout: 120_000,
  connectionRetryTimeout: 120_000,
  connectionRetryCount: 0,
  specFileRetries: 0,
  mochaOpts: {
    ui: "bdd",
    timeout: 180_000,
    retries: 0,
  },
};
