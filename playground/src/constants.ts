import type { FormatOptions } from "./types";

// Sample Typst documents for testing
export const SAMPLE_DOCUMENTS = {
  basic: {
    name: "Basic Document",
    description: "Simple document with headings, text, and math",
    filePath: import("/samples/basic.typ?url"),
  },

  academic: {
    name: "Academic Paper",
    description: "Research paper with citations, figures, and tables",
    filePath: import("/samples/academic.typ?url"),
  },

  presentation: {
    name: "Presentation Slides",
    description: "Slide deck with multiple layouts and visual elements",
    filePath: import("/samples/presentation.typ?url"),
  },

  letter: {
    name: "Business Letter",
    description: "Formal business correspondence template",
    filePath: import("/samples/letter.typ?url"),
  },

  cookbook: {
    name: "Recipe Collection",
    description: "Cookbook with recipes, ingredients, and cooking instructions",
    filePath: import("/samples/cookbook.typ?url"),
  },

  technical: {
    name: "Technical Documentation",
    description: "API documentation with code examples and specifications",
    filePath: import("/samples/technical.typ?url"),
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
