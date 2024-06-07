import { resolve } from 'path';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { viteSingleFile } from "vite-plugin-singlefile"

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    viteSingleFile()
  ]
});
