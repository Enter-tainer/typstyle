// Generic Monaco utilities

import loader from "@monaco-editor/loader";
import type { Monaco } from "@monaco-editor/loader";

export { wireTmGrammar } from "./textmate";
export { registerTheme, registerThemes } from "./theme-registry";

export const initMonacoLoader = async (): Promise<Monaco> => {
  return await loader.init();
};
