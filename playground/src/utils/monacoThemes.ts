import { loader } from "@monaco-editor/react";
import type { editor } from "monaco-editor";
import type { ThemeType } from "../types";
import { registerTypstLanguage } from "../typst-language";

const DEFAULT_LIGHT_THEME = "typstyle-light";
const DEFAULT_DARK_THEME = "typstyle-dark";

const fetchTheme = async (url: string) =>
  (await (await fetch(url)).json()).data as editor.IStandaloneThemeData;

export const initMonaco = async () => {
  const [monaco, lightTheme, darkTheme] = await Promise.all([
    loader.init(),
    fetchTheme(
      "https://cdn.jsdelivr.net/npm/@react-monaco/assets/assets/themes/atom-one-light.json",
    ),
    fetchTheme(
      "https://cdn.jsdelivr.net/npm/@react-monaco/assets/assets/themes/csb-default.json",
    ),
  ]);
  await registerTypstLanguage(monaco);
  monaco.editor.defineTheme(DEFAULT_LIGHT_THEME, lightTheme);
  monaco.editor.defineTheme(DEFAULT_DARK_THEME, darkTheme);
};

export const getEditorTheme = (theme: ThemeType): string =>
  theme === "light" ? DEFAULT_LIGHT_THEME : DEFAULT_DARK_THEME;
