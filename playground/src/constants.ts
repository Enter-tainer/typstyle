import type { FormatOptions } from "./types";

// Sample Typst documents for testing
export const SAMPLE_DOCUMENTS = {
  basic: {
    name: "Basic Document",
    description: "Simple document with headings, text, and math",
    filePath: new URL("/samples/basic.typ", import.meta.url),
  },

  academic: {
    name: "Academic Paper",
    description: "Research paper with citations, figures, and tables",
    filePath: new URL("/samples/academic.typ", import.meta.url),
  },

  presentation: {
    name: "Presentation Slides",
    description: "Slide deck with multiple layouts and visual elements",
    filePath: new URL("/samples/presentation.typ", import.meta.url),
  },

  letter: {
    name: "Business Letter",
    description: "Formal business correspondence template",
    filePath: new URL("/samples/letter.typ", import.meta.url),
  },

  cookbook: {
    name: "Recipe Collection",
    description: "Cookbook with recipes, ingredients, and cooking instructions",
    filePath: new URL("/samples/cookbook.typ", import.meta.url),
  },

  technical: {
    name: "Technical Documentation",
    description: "API documentation with code examples and specifications",
    filePath: new URL("/samples/technical.typ", import.meta.url),
  },
} as const;

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
