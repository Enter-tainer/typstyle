// Playground-specific Monaco setup

import { initMonacoLoader, registerThemes } from "@/monaco";
import type { ThemeType } from "@/types";
import { registerTypstLanguage } from "./typst-language";

const DEFAULT_LIGHT_THEME = "play-light";
const DEFAULT_DARK_THEME = "play-dark";

export const initMonaco = async () => {
  const monaco = await initMonacoLoader();

  // Register Typst language
  registerTypstLanguage(monaco);

  // Register themes for playground
  registerThemes(monaco, [
    {
      name: DEFAULT_LIGHT_THEME,
      url: "https://cdn.jsdelivr.net/npm/@react-monaco/assets/assets/themes/atom-one-light.json",
    },
    {
      name: DEFAULT_DARK_THEME,
      url: "https://cdn.jsdelivr.net/npm/@react-monaco/assets/assets/themes/csb-default.json",
    },
  ]);
};

export const getEditorTheme = (theme: ThemeType): string => {
  return theme === "light" ? DEFAULT_LIGHT_THEME : DEFAULT_DARK_THEME;
};
