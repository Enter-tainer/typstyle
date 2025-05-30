import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import react from "@vitejs/plugin-react";
import toplevelAwait from "vite-plugin-top-level-await";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
  base: "/typstyle/playground/",

  plugins: [react(), tailwindcss(), wasm(), toplevelAwait()],

  build: {
    rollupOptions: {
      output: {
        manualChunks: (id): string | undefined => {
          // Large packages get their own chunks
          if (id.includes("monaco-editor")) {
            return "monaco-editor";
          }
          if (id.includes("monaco-themes")) {
            return "monaco-themes";
          }
          if (id.includes("react")) {
            return "react";
          }

          // Group all application source code and public resources together
          if (id.includes("/src/") || id.includes("?url")) {
            return "app";
          }

          // Default chunk for everything else
          return "vendor";
        },
      },
    },
  },
});
