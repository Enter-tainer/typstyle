import type { Monaco } from "@monaco-editor/react";
import type { editor } from "monaco-editor";
import { useTheme } from "../contexts";
import type { FormatOptions } from "../types";
import { CodeEditor } from "./CodeEditor";

export interface SourceEditorProps {
  sourceCode: string;
  onChange: (value: string | undefined) => void;
  onMount: (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => void;
  formatOptions: FormatOptions;
  lineLengthGuide?: number;
}

export function SourceEditor({
  sourceCode,
  onChange,
  onMount,
  lineLengthGuide,
}: SourceEditorProps) {
  const { theme } = useTheme();
  return (
    <CodeEditor
      value={sourceCode}
      onChange={onChange}
      onMount={onMount}
      theme={theme}
      indentSize={0}
      language="typst"
      readOnly={false}
      showLineNumbers={true}
      enableFolding={true}
      enableWordWrap={true}
      enableMinimap={false}
      rulers={lineLengthGuide ? [lineLengthGuide] : []}
    />
  );
}
