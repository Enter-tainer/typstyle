// Generic Monaco utilities

import { loader } from "@monaco-editor/react";
import type { Monaco } from "./types";

export { setupLanguage } from "./language-registry";
export { wireTmGrammar } from "./textmate";
export { registerTheme, registerThemes } from "./theme-registry";

export const initMonacoLoader = async (): Promise<Monaco> => {
  return await loader.init();
};
