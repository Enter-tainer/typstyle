import { loader, type Monaco } from "@monaco-editor/react";
import type { editor } from "monaco-editor";
import type { ThemeType } from "../types";
import { registerTypstLanguage } from "../typst-language";

const DEFAULT_LIGHT_THEME = "typstyle-light";
const DEFAULT_DARK_THEME = "typstyle-dark";

const fetchTheme = async (url: string) =>
  (await (await fetch(url)).json()).data as editor.IStandaloneThemeData;

const registerTheme = async (monaco: Monaco, name: string, url: string) => {
  monaco.editor.defineTheme(name, await fetchTheme(url));
};

export const initMonaco = async () => {
  const monaco = await loader.init();
  // here we don't need to sync the following loading
  registerTypstLanguage(monaco);
  registerTheme(
    monaco,
    DEFAULT_LIGHT_THEME,
    "https://cdn.jsdelivr.net/npm/@react-monaco/assets/assets/themes/atom-one-light.json"
  );
  registerTheme(
    monaco,
    DEFAULT_DARK_THEME,
    "https://cdn.jsdelivr.net/npm/@react-monaco/assets/assets/themes/csb-default.json"
  );
};

export const getEditorTheme = (theme: ThemeType): string =>
  theme === "light" ? DEFAULT_LIGHT_THEME : DEFAULT_DARK_THEME;
