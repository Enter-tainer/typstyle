// Types for the Typstyle Playground application

export type ThemeType = "light" | "dark";

export const ScreenSize = {
  Wide: "wide",
  Medium: "medium",
  Thin: "thin",
} as const;

export type ScreenSizeType = (typeof ScreenSize)[keyof typeof ScreenSize];

export interface FormatOptions {
  maxLineLength: number;
  indentSize: number;
  collapseMarkupSpaces: boolean;
  reorderImportItems: boolean;
  wrapText: boolean;
}
