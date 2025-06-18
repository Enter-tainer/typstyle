// Generic theme registration utilities for Monaco Editor

import type { editor, Monaco } from "./types";

const fetchTheme = async (url: string): Promise<editor.IStandaloneThemeData> =>
  (await (await fetch(url)).json()).data;

export const registerTheme = async (
  monaco: Monaco,
  name: string,
  url: string,
): Promise<void> => {
  try {
    monaco.editor.defineTheme(name, await fetchTheme(url));
  } catch (err) {
    console.error(`Failed to register monaco theme ${name} from ${url}:`, err);
  }
};

export const registerThemes = async (
  monaco: Monaco,
  themes: Array<{ name: string; url: string }>,
): Promise<void> => {
  await Promise.all(
    themes.map(({ name, url }) => registerTheme(monaco, name, url)),
  );
};
