// take from: https://github.com/janpoem/react-monaco/blob/main/packages/plugin-textmate/src/monaco-editor-textmate/tm-to-monaco-token.ts

import type * as monaco from "monaco-editor";

// as described in issue: https://github.com/NeekSandhu/monaco-textmate/issues/5
export const TMToMonacoToken = (
  editor: monaco.editor.ICodeEditor,
  scopes: string[],
) => {
  let scopeName = "";
  if (!scopes[0]) return "";

  // get the scope name. Example: cpp , java, haskell
  for (let i = scopes[0].length - 1; i >= 0; i -= 1) {
    const char = scopes[0][i];
    if (char === ".") {
      break;
    }
    scopeName = char + scopeName;
  }

  // iterate through all scopes from last to first
  for (let i = scopes.length - 1; i >= 0; i -= 1) {
    const scope = scopes[i];
    if (scope == null) continue;
    /**
     * Try all possible tokens from high specific token to low specific token
     *
     * Example:
     * 0 meta.function.definition.parameters.cpp
     * 1 meta.function.definition.parameters
     *
     * 2 meta.function.definition.cpp
     * 3 meta.function.definition
     *
     * 4 meta.function.cpp
     * 5 meta.function
     *
     * 6 meta.cpp
     * 7 meta
     */
    for (let i = scope.length - 1; i >= 0; i -= 1) {
      const char = scope[i];
      if (char === ".") {
        const token = scope.slice(0, i);
        if (
          // @ts-ignore
          // biome-ignore lint/complexity/useLiteralKeys: <explanation>
          editor["_themeService"]._theme._tokenTheme._match(
            `${token}.${scopeName}`,
          )._foreground > 1
        ) {
          return `${token}.${scopeName}`;
        }
        if (
          // @ts-ignore
          // biome-ignore lint/complexity/useLiteralKeys: <explanation>
          editor["_themeService"]._theme._tokenTheme._match(token)._foreground >
          1
        ) {
          return token;
        }
      }
    }
  }

  return "";
};
