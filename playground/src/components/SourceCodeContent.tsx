import MonacoEditor from "@monaco-editor/react";
import type { Monaco } from "@monaco-editor/react";
import type { FormatOptions, ThemeType } from "../types";

interface SourceCodeContentProps {
  sourceCode: string;
  onChange: (value: string | undefined) => void;
  onMount: (editor: unknown, monaco: Monaco) => void;
  theme: ThemeType;
  formatOptions: FormatOptions;
}

export function SourceCodeContent({
  sourceCode,
  onChange,
  onMount,
  theme,
  formatOptions,
}: SourceCodeContentProps) {
  return (
    <MonacoEditor
      height="100%"
      language="typst"
      value={sourceCode}
      onChange={onChange}
      onMount={onMount}
      theme={theme === "light" ? "light" : "vs-dark"}
      options={{
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        fontSize: 14,
        fontFamily: "Monaco, Menlo, Ubuntu Mono, monospace",
        wordWrap: "on",
        lineNumbers: "on",
        folding: true,
        automaticLayout: true,
        padding: { top: 8, bottom: 8 },
        renderLineHighlight: "gutter",
        smoothScrolling: true,
        tabSize: formatOptions.indentSize,
        insertSpaces: true,
        scrollbar: {
          vertical: "hidden",
          horizontal: "hidden",
        },
      }}
    />
  );
}
