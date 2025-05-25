import { type Monaco } from "@monaco-editor/react";

// Typst language definition for Monaco Editor
export const registerTypstLanguage = (monaco: Monaco) => {
  // Register the Typst language
  monaco.languages.register({ id: "typst" });

  // Define the language configuration
  monaco.languages.setLanguageConfiguration("typst", {
    comments: {
      lineComment: "//",
      blockComment: ["/*", "*/"],
    },
    brackets: [
      ["{", "}"],
      ["[", "]"],
      ["(", ")"],
    ],
    autoClosingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
      { open: "`", close: "`" },
    ],
    surroundingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
      { open: "`", close: "`" },
    ],
  });

  // Define the token provider for syntax highlighting
  monaco.languages.setMonarchTokensProvider("typst", {
    tokenizer: {
      root: [
        // Headings
        [/^=+\s.*$/, "keyword.heading"],

        // Comments
        [/\/\/.*$/, "comment"],
        [/\/\*/, "comment", "@comment"],

        // Functions and variables
        [/#[a-zA-Z_][a-zA-Z0-9_]*/, "variable.function"],
        [/@[a-zA-Z_][a-zA-Z0-9_]*/, "variable.parameter"],

        // Math mode
        [/\$/, "string.math", "@math"],

        // String literals
        [/"/, "string", "@string"],
        [/'/, "string", "@singlestring"],

        // Code blocks
        [/```/, "string.code", "@codeblock"],
        [/`/, "string.code", "@inlinecode"],

        // Markup
        [/\*([^*]|\*[^*])*\*/, "markup.bold"],
        [/_([^_]|_[^_])*_/, "markup.italic"],
        [/~([^~]|~[^~])*~/, "markup.strikethrough"],

        // Links
        [/https?:\/\/[^\s]+/, "string.link"],

        // Numbers
        [/\b\d+(\.\d+)?(pt|em|mm|cm|in|%)\b/, "number"],
        [/\b\d+(\.\d+)?\b/, "number"],

        // Keywords
        [
          /\b(let|set|show|import|include|if|else|for|while|break|continue|return)\b/,
          "keyword",
        ],

        // Built-in functions
        [
          /\b(box|text|par|page|document|grid|table|figure|image|rect|circle|ellipse|line|path)\b/,
          "keyword.function",
        ],

        // Colors
        [
          /\b(red|green|blue|yellow|black|white|gray|orange|purple|pink|brown)\b/,
          "constant.color",
        ],
        [/#[0-9a-fA-F]{3,8}\b/, "constant.color"],
        [/rgb\([^)]+\)/, "constant.color"],

        // Operators
        [/[+\-*=<>!]+/, "operator"],

        // Delimiters
        [/[{}()[\]]/, "delimiter"],
        [/[;,.]/, "delimiter"],
      ],

      comment: [
        [/[^/*]+/, "comment"],
        [/\*\//, "comment", "@pop"],
        [/[/*]/, "comment"],
      ],

      string: [
        [/[^\\"]+/, "string"],
        [/\\./, "string.escape"],
        [/"/, "string", "@pop"],
      ],

      singlestring: [
        [/[^\\']+/, "string"],
        [/\\./, "string.escape"],
        [/'/, "string", "@pop"],
      ],

      math: [
        [/[^$]+/, "string.math"],
        [/\$/, "string.math", "@pop"],
      ],

      codeblock: [
        [/[^`]+/, "string.code"],
        [/```/, "string.code", "@pop"],
        [/`/, "string.code"],
      ],

      inlinecode: [
        [/[^`]+/, "string.code"],
        [/`/, "string.code", "@pop"],
      ],
    },
  }); // Define the Komeiji Koishi themed color scheme
  monaco.editor.defineTheme("typst-theme", {
    base: "vs",
    inherit: true,
    rules: [
      // Headings - Koishi's hat green
      { token: "keyword.heading", foreground: "2E7D32", fontStyle: "bold" },
      // Comments - soft green like her dress
      { token: "comment", foreground: "7CB342", fontStyle: "italic" },
      // Functions - third eye purple
      { token: "variable.function", foreground: "8E24AA" },
      // Parameters - lighter purple
      { token: "variable.parameter", foreground: "AB47BC" },
      // Math - golden hair color with soft background
      { token: "string.math", foreground: "FFB300", background: "FFF8E1" },
      // Strings - rose/heart color
      { token: "string", foreground: "E91E63" },
      // Code blocks - soft green with light background
      { token: "string.code", foreground: "2E7D32", background: "F1F8E9" },
      // Links - deeper green with underline
      { token: "string.link", foreground: "1B5E20", fontStyle: "underline" },
      // Bold markup - darker green
      { token: "markup.bold", foreground: "2E7D32", fontStyle: "bold" },
      // Italic markup - soft purple
      { token: "markup.italic", foreground: "9C27B0", fontStyle: "italic" },
      // Strikethrough - muted color
      {
        token: "markup.strikethrough",
        foreground: "757575",
        fontStyle: "strikethrough",
      },
      // Numbers - golden yellow
      { token: "number", foreground: "FFA000" },
      // Keywords - Koishi's main green
      { token: "keyword", foreground: "388E3C", fontStyle: "bold" },
      // Keyword functions - bright green
      { token: "keyword.function", foreground: "4CAF50" },
      // Colors - heart pink
      { token: "constant.color", foreground: "E91E63" },
      // Operators - warm brown
      { token: "operator", foreground: "6D4C41" },
      // Delimiters - soft brown
      { token: "delimiter", foreground: "8D6E63" },
      // String escapes - bright gold
      { token: "string.escape", foreground: "FF8F00", fontStyle: "bold" },
    ],
    colors: {
      // Main editor background - very soft green-tinted cream
      "editor.background": "#FAFAFA",
      // Text color - soft dark green
      "editor.foreground": "#1B5E20",
      // Current line highlight - very soft green
      "editor.lineHighlightBackground": "#F1F8E9",
      // Selection background - soft pink
      "editor.selectionBackground": "#FCE4EC",
      // Cursor color - Koishi's green
      "editorCursor.foreground": "#2E7D32",
      // Line numbers - muted green
      "editorLineNumber.foreground": "#81C784",
      "editorLineNumber.activeForeground": "#2E7D32",
      // Bracket matching
      "editorBracketMatch.background": "#E8F5E8",
      "editorBracketMatch.border": "#4CAF50",
      // Find matches
      "editor.findMatchBackground": "#FFE082",
      "editor.findMatchHighlightBackground": "#FFF3C0",
      // Scrollbar
      "scrollbarSlider.background": "#C8E6C9",
      "scrollbarSlider.hoverBackground": "#A5D6A7",
      "scrollbarSlider.activeBackground": "#81C784",
    },
  });
};
