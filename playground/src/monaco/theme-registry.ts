// Generic theme registration utilities for Monaco Editor

import type { editor, Monaco } from "./types";

// Fetches a converted theme from CDN
// NOTE: The actual Monaco theme data is nested under the "data" property
// This structure may change if we use different theme sources or formats
const fetchTheme = async (
  url: string,
): Promise<editor.IStandaloneThemeData> => {
  const response = await fetch(url);
  const themeData = await response.json();
  return themeData.data; // Extract Monaco theme from converted format
};

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
