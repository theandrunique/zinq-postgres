import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import tailwindcss from "@tailwindcss/vite";
import adapter from "@sveltejs/adapter-static";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

function readAppVersion() {
  const cargoToml = readFileSync(new URL("../Cargo.toml", import.meta.url), "utf8");
  const version = cargoToml.match(/^version = "([^"]+)"/m)?.[1] ?? "0.0.0";

  let commit = process.env.GIT_COMMIT?.slice(0, 7) ?? "";

  if (!commit) {
    try {
      commit = execSync("git rev-parse --short HEAD", { stdio: ["ignore", "pipe", "ignore"] })
        .toString()
        .trim();
    } catch {
      commit = "";
    }
  }

  return { version, commit };
}

export default defineConfig(({ command }) => {
  const { version, commit } = readAppVersion();

  return {
    define: {
      __APP_VERSION__: JSON.stringify({ version, commit, dev: command === "serve" }),
    },
    plugins: [
      tailwindcss(),
      sveltekit({
        compilerOptions: {
          // Force runes mode for the project, except for libraries. Can be removed in svelte 6.
          runes: ({ filename }) => (filename.split(/[/\\]/).includes("node_modules") ? undefined : true),
        },
        adapter: adapter({ fallback: "index.html" }),
        version: {
          name: version,
          pollInterval: 60_000,
        },
      }),
    ],
    server: {
      proxy: {
        "/api": "http://localhost:8000",
        "/socket.io": {
          target: "http://localhost:8000",
          ws: true,
        },
      },
    },
  };
});
