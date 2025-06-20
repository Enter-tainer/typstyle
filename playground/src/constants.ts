import type { FormatOptions } from "./types";

// Sample Typst documents for testing
export const SAMPLE_DOCUMENTS: {
  [key: string]: { name: string; filePath: URL };
} = {
  // TODO: add samples
} as const;

// Default format style options
export const DEFAULT_FORMAT_OPTIONS: FormatOptions = {
  maxLineLength: 80,
  indentSize: 2,
  collapseMarkupSpaces: false,
  reorderImportItems: true,
  wrapText: false,
};
