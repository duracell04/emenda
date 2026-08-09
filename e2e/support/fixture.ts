import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process";

interface FixtureResponse<T> {
  id: number | null;
  ok: boolean;
  result?: T;
  error?: {
    code: string;
    message: string;
  };
}

type PendingRequest = {
  id: number;
  resolve: (result: unknown) => void;
  reject: (error: Error) => void;
  timer: NodeJS.Timeout;
};

const FIXTURE_TIMEOUT_MS = 45_000;
const MAX_STDERR_BYTES = 64 * 1024;
const MAX_RESPONSE_LINE_BYTES = 64 * 1024;

export class FixtureClient {
  readonly #child: ChildProcessWithoutNullStreams;
  #buffer = "";
  #stderr = "";
  #nextId = 1;
  #pending: PendingRequest | undefined;

  private constructor(child: ChildProcessWithoutNullStreams) {
    this.#child = child;
    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk: string) => this.#consume(chunk));
    child.stderr.on("data", (chunk: string) => {
      this.#stderr = (this.#stderr + chunk).slice(-MAX_STDERR_BYTES);
    });
    child.on("error", (error) => this.#rejectPending(error));
    child.on("exit", (code, signal) => {
      if (this.#pending) {
        this.#rejectPending(
          new Error(
            `desktop fixture exited before responding (code ${String(code)}, signal ${String(signal)}): ${this.#stderr.trim()}`,
          ),
        );
      }
    });
  }

  static start(): FixtureClient {
    const executable = process.env.EMENDA_E2E_FIXTURE_BINARY;
    if (!executable) {
      throw new Error("EMENDA_E2E_FIXTURE_BINARY is required");
    }
    const fixtureEnvironment = { ...process.env };
    for (const name of Object.keys(fixtureEnvironment)) {
      const normalizedName = name.toUpperCase();
      if (
        normalizedName === "OPENROUTER_API_KEY" ||
        normalizedName.startsWith("EMENDA_E2E_") ||
        normalizedName.startsWith("TAURI_") ||
        normalizedName.startsWith("WDIO")
      ) {
        delete fixtureEnvironment[name];
      }
    }
    const child = spawn(executable, [], {
      cwd: process.cwd(),
      env: fixtureEnvironment,
      stdio: ["pipe", "pipe", "pipe"],
      windowsHide: true,
    });
    return new FixtureClient(child);
  }

  async request<T>(command: string, fields: Record<string, unknown> = {}): Promise<T> {
    if (this.#pending) {
      throw new Error("the fixture protocol permits only one in-flight request");
    }
    const id = this.#nextId++;
    return await new Promise<T>((resolve, reject) => {
      const timer = setTimeout(() => {
        this.#pending = undefined;
        reject(new Error(`desktop fixture timed out handling ${command}`));
      }, FIXTURE_TIMEOUT_MS);
      this.#pending = {
        id,
        resolve: resolve as (result: unknown) => void,
        reject,
        timer,
      };
      this.#child.stdin.write(`${JSON.stringify({ id, version: 1, command, ...fields })}\n`);
    });
  }

  async shutdown(): Promise<void> {
    if (this.#child.exitCode !== null || this.#child.signalCode !== null) {
      return;
    }
    try {
      await this.request<{ stopped: boolean }>("shutdown");
    } finally {
      this.#child.stdin.end();
      await new Promise<void>((resolve) => {
        if (this.#child.exitCode !== null || this.#child.signalCode !== null) {
          resolve();
          return;
        }
        const timer = setTimeout(() => {
          this.#child.kill();
          resolve();
        }, 5_000);
        this.#child.once("exit", () => {
          clearTimeout(timer);
          resolve();
        });
      });
    }
  }

  #consume(chunk: string): void {
    this.#buffer += chunk;
    for (;;) {
      const newline = this.#buffer.indexOf("\n");
      if (newline < 0) {
        if (Buffer.byteLength(this.#buffer, "utf8") > MAX_RESPONSE_LINE_BYTES) {
          this.#abortProtocol("desktop fixture response exceeded the 64 KiB JSONL limit");
        }
        return;
      }
      const line = this.#buffer.slice(0, newline).replace(/\r$/, "");
      this.#buffer = this.#buffer.slice(newline + 1);
      if (!line) {
        continue;
      }
      if (Buffer.byteLength(line, "utf8") > MAX_RESPONSE_LINE_BYTES) {
        this.#abortProtocol("desktop fixture response exceeded the 64 KiB JSONL limit");
        return;
      }
      let response: FixtureResponse<unknown>;
      try {
        response = JSON.parse(line) as FixtureResponse<unknown>;
      } catch (error) {
        this.#rejectPending(new Error(`desktop fixture returned invalid JSON: ${String(error)}`));
        continue;
      }
      const pending = this.#pending;
      if (!pending || response.id !== pending.id) {
        this.#rejectPending(new Error(`desktop fixture returned an unexpected response id`));
        continue;
      }
      clearTimeout(pending.timer);
      this.#pending = undefined;
      if (!response.ok) {
        pending.reject(
          new Error(
            `desktop fixture ${response.error?.code ?? "unknown_error"}: ${response.error?.message ?? "no message"}`,
          ),
        );
      } else {
        pending.resolve(response.result);
      }
    }
  }

  #rejectPending(error: Error): void {
    const pending = this.#pending;
    if (!pending) {
      return;
    }
    clearTimeout(pending.timer);
    this.#pending = undefined;
    pending.reject(error);
  }

  #abortProtocol(message: string): void {
    this.#buffer = "";
    this.#rejectPending(new Error(message));
    this.#child.kill();
  }
}
