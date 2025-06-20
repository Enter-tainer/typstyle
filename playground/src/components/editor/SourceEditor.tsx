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
      language="typst"
      indentSize={0}
      readOnly={false}
      options={{
        wordWrap: "on",
        minimap: { enabled: true },
        rulers: lineLengthGuide ? [lineLengthGuide] : [],
      }}
      onChange={onChange}
    />
  );
}
