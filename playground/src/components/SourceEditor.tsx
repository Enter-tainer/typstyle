import { CodeEditor } from "./CodeEditor";

export interface SourceEditorProps {
  value: string;
  onChange: (value: string | undefined) => void;
  lineLengthGuide?: number;
}

export function SourceEditor({
  value,
  onChange,
  lineLengthGuide,
}: SourceEditorProps) {
  return (
    <CodeEditor
      value={value}
      onChange={onChange}
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
