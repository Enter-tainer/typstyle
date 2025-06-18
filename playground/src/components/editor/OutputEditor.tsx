import { CodeEditor } from "./CodeEditor";

export interface OutputEditorProps {
  content: string;
  language: string;
  indentSize: number;
  lineLengthGuide?: number;
}

export function OutputEditor({
  content,
  language,
  indentSize,
  lineLengthGuide,
}: OutputEditorProps) {
  return (
    <div className="h-full">
      <CodeEditor
        value={content}
        indentSize={indentSize}
        language={language}
        readOnly={true}
        options={{
          rulers: lineLengthGuide ? [lineLengthGuide] : [],
        }}
      />
    </div>
  );
}
