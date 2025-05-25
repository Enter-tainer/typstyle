import { useTheme } from "../contexts";
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
  const { theme } = useTheme();
  return (
    <div className="h-full">
      <CodeEditor
        value={content}
        theme={theme}
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
