// Types for the Typstyle Playground application

export type ThemeType = "light" | "dark";

export type ScreenSizeType = "wide" | "thin";

export type OutputType = "formatted" | "ast" | "ir";

export interface FormatOptions {
  maxLineLength: number;
  indentSize: number;
  collapseMarkupSpaces: boolean;
  reorderImportItems: boolean;
  wrapText: boolean;
}
