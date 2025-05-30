import { loader } from "@monaco-editor/react";
import type { editor } from "monaco-editor";
import type { ThemeType } from "../types";
import { registerTypstLanguage } from "../typst-language";

const DEFAULT_LIGHT_THEME = "xcode-default";
const DEFAULT_DARK_THEME = "dracula";

export const initMonaco = async () => {
  const [monaco, xcodeTheme, draculaTheme] = await Promise.all([
    loader.init(),
    import("monaco-themes/themes/Xcode_default.json"),
    import("monaco-themes/themes/Dracula.json"),
  ]);
  registerTypstLanguage(monaco);
  monaco.editor.defineTheme(
    DEFAULT_LIGHT_THEME,
    xcodeTheme.default as editor.IStandaloneThemeData,
  );
  monaco.editor.defineTheme(
    DEFAULT_DARK_THEME,
    draculaTheme.default as editor.IStandaloneThemeData,
  );
};

export const getEditorTheme = (theme: ThemeType): string =>
  theme === "light" ? DEFAULT_LIGHT_THEME : DEFAULT_DARK_THEME;
