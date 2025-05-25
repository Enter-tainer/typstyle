import type { Monaco } from "@monaco-editor/react";
import { useEffect, useState } from "react";
import type { ThemeType } from "../types";
import { registerTypstLanguage } from "../typst-language";

export function useMonacoEditor(theme: ThemeType) {
  const [monacoInstance, setMonacoInstance] = useState<Monaco | null>(null);

  // Update Monaco theme when theme changes
  useEffect(() => {
    if (monacoInstance) {
      monacoInstance.editor.setTheme(
        theme === "light" ? "typst-theme" : "vs-dark",
      );
    }
  }, [theme, monacoInstance]);

  // Handle Monaco Editor mounting with theme support
  const handleEditorDidMount = (_editor: unknown, monaco: Monaco) => {
    registerTypstLanguage(monaco);
    setMonacoInstance(monaco);
    // Set theme based on current theme state
    monaco.editor.setTheme(theme === "light" ? "typst-theme" : "vs-dark");
  };

  return {
    monacoInstance,
    handleEditorDidMount,
  };
}
