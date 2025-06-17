import type { Monaco } from "@monaco-editor/loader";
import type { languages } from "monaco-editor";
import * as oniguruma from "vscode-oniguruma";
import * as vsctm from "vscode-textmate";
import languageConfiguration from "./assets/language-configuration.json";
import { wireTmGrammar } from "./monaco";

const vscodeOnigurumaLib = (async () => {
  const response = await fetch(
    new URL("https://cdn.jsdelivr.net/npm/vscode-oniguruma/release/onig.wasm"),
  );
  await oniguruma.loadWASM(await response.arrayBuffer());
  return {
    createOnigScanner(patterns: string[]) {
      return new oniguruma.OnigScanner(patterns);
    },
    createOnigString(s: string) {
      return new oniguruma.OnigString(s);
    },
  };
})();

// Create a registry that can create a grammar from a scope name.
const registry = new vsctm.Registry({
  onigLib: vscodeOnigurumaLib,
  loadGrammar: async (scopeName) => {
    if (scopeName === "source.typst") {
      const url = new URL("/typst.tmLanguage.json", import.meta.url);
      const data = await fetch(url);
      const text = await data.text();
      return vsctm.parseRawGrammar(text, url.toString());
    }
    console.log(`Unknown scope name: ${scopeName}`);
  },
});

// Typst language definition for Monaco Editor
export const registerTypstLanguage = async (monaco: Monaco) => {
  // Register the Typst language
  monaco.languages.register({ id: "typst" });

  // Define the language configuration
  monaco.languages.setLanguageConfiguration(
    "typst",
    languageConfiguration as languages.LanguageConfiguration,
  );

  await wireTmGrammar(monaco, registry, "typst", "source.typst");
};
