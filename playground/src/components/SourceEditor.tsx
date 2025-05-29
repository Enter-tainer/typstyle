import type { Monaco } from "@monaco-editor/react";
import type { editor } from "monaco-editor";
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
  return (
    <CodeEditor
      value={sourceCode}
      onChange={onChange}
      onMount={onMount}
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
