import { CodeEditor } from "./CodeEditor";

export interface OutputEditorProps {
  content: string;
  language?: string;
  indentSize?: number;
  lineLengthGuide?: number;
}

export function OutputEditor({
  content,
  language = "typst",
  indentSize = 2,
  lineLengthGuide,
}: OutputEditorProps) {
  return (
    <div className="h-full">
      <CodeEditor
        value={content}
        indentSize={indentSize}
        language={language}
        readOnly={true}
        showLineNumbers={false}
        enableFolding={language === "json"}
        enableWordWrap={false}
        enableMinimap={false}
        rulers={lineLengthGuide ? [lineLengthGuide] : []}
      />
    </div>
  );
}
