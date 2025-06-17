// Generic theme registration utilities for Monaco Editor

import type { Monaco } from "@monaco-editor/loader";
import type { editor } from "monaco-editor";

const fetchTheme = async (url: string): Promise<editor.IStandaloneThemeData> =>
  (await (await fetch(url)).json()).data;

export const registerTheme = async (
  monaco: Monaco,
  name: string,
  url: string,
): Promise<void> => {
  monaco.editor.defineTheme(name, await fetchTheme(url));
};

export const registerThemes = async (
  monaco: Monaco,
  themes: Array<{ name: string; url: string }>,
): Promise<void> => {
  await Promise.all(
    themes.map(({ name, url }) => registerTheme(monaco, name, url)),
  );
};
