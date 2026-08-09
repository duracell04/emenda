import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const desktopE2eBridge = (enabled: boolean) => ({
  name: "emenda-desktop-e2e-bridge",
  resolveId(id: string) {
    return id === "virtual:emenda-desktop-e2e-bridge" ? `\0${id}` : undefined;
  },
  load(id: string) {
    if (id !== "\0virtual:emenda-desktop-e2e-bridge") {
      return undefined;
    }
    return enabled ? 'import "@wdio/tauri-plugin";' : "export {};";
  },
});

export default defineConfig(({ mode }) => ({
  plugins: [react(), desktopE2eBridge(mode === "desktop-e2e")],
  clearScreen: false,
  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    outDir: mode === "desktop-e2e" ? "dist-desktop-e2e" : "dist",
    target: process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify: process.env.TAURI_ENV_DEBUG ? false : "esbuild",
    sourcemap: Boolean(process.env.TAURI_ENV_DEBUG),
  },
}));
