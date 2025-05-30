import type { Monaco } from "@monaco-editor/react";

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
      ["[", "]"],
      ["{", "}"],
      ["(", ")"],
    ],
    autoClosingPairs: [
      { open: "[", close: "]" },
      { open: "{", close: "}" },
      { open: "(", close: ")" },
      { open: '"', close: '"', notIn: ["string"] },
      { open: "$", close: "$", notIn: ["string"] },
    ],
    autoCloseBefore: ";:.,=}])>$ \n\t",
    surroundingPairs: [
      { open: "[", close: "]" },
      { open: "{", close: "}" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
      { open: "*", close: "*" },
      { open: "_", close: "_" },
      { open: "`", close: "`" },
      { open: "$", close: "$" },
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
  });
};
