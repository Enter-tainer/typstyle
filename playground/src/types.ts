// Types for the Typstyle Playground application

export type ThemeType = "light" | "dark";

export type ScreenSizeType = "wide" | "medium" | "thin";

export interface FormatOptions {
  maxLineLength: number;
  indentSize: number;
  collapseMarkupSpaces: boolean;
  reorderImportItems: boolean;
  wrapText: boolean;
}
