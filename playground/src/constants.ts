import type { FormatOptions } from "./types";

// Sample Typst documents for testing
export const SAMPLE_DOCUMENTS: {
  [key: string]: { name: string; filePath: URL };
} = {} as const;

// Type for sample document keys
export type SampleDocumentKey = keyof typeof SAMPLE_DOCUMENTS;

// Default format style options
export const DEFAULT_FORMAT_OPTIONS: FormatOptions = {
  maxLineLength: 80,
  indentSize: 2,
  collapseMarkupSpaces: false,
  reorderImportItems: true,
  wrapText: false,
};
