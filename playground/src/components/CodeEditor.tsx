import MonacoEditor from "@monaco-editor/react";
import type { Monaco } from "@monaco-editor/react";
import type { editor } from "monaco-editor";
import { useCallback, useEffect, useMemo, useRef } from "react";
import { useTheme } from "../contexts";
import { getEditorTheme } from "../utils/monaco-setup";

/**
 * CodeEditor - A configurable Monaco Editor wrapper for the Typstyle Playground
 *
 * This component serves as the base editor implementation across the application.
 * The `indentSize` prop now controls indentation: a positive value sets a fixed indent
 * size (using spaces), while 0 or a negative value enables auto-detection of indentation.
 */

export interface CodeEditorProps {
  value: string;
  onChange?: (value: string | undefined) => void;
  onMount?: (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => void;
  indentSize: number; // Positive for fixed indent, 0 or negative for auto-detect
  language?: string;
  readOnly?: boolean;
  showLineNumbers?: boolean;
  enableFolding?: boolean;
  enableWordWrap?: boolean;
  enableMinimap?: boolean;
  rulers?: number[];
}

export function CodeEditor({
  value,
  onChange,
  onMount,
  indentSize,
  language = "typst",
  readOnly = false,
  showLineNumbers = true,
  enableFolding = true,
  enableWordWrap = true,
  enableMinimap = false,
  rulers,
}: CodeEditorProps) {
  const { theme } = useTheme();
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);

  const editorTheme = useMemo(() => getEditorTheme(theme), [theme]);

  const applyIndentationSettings = useCallback(() => {
    if (editorRef.current) {
      const editor = editorRef.current;
      const model = editor.getModel();

      if (model) {
        if (indentSize > 0) {
          // Positive indentSize: use fixed indentation
          editor.updateOptions({ detectIndentation: false });
          model.updateOptions({
            tabSize: indentSize,
            insertSpaces: true, // Typically use spaces for fixed indentation
          });
        } else {
          // indentSize is 0 or negative: use auto-detection
          editor.updateOptions({ detectIndentation: true });
          // When detectIndentation is true, Monaco handles tabSize and insertSpaces.
        }
      }
    }
  }, [indentSize]); // Dependency is now only indentSize

  const handleEditorDidMount = useCallback(
    (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => {
      editorRef.current = editor;
      applyIndentationSettings(); // Apply initial settings
      onMount?.(editor, monaco);
    },
    [onMount, applyIndentationSettings],
  );

  useEffect(() => applyIndentationSettings(), [applyIndentationSettings]);

  const editorOptions: editor.IStandaloneEditorConstructionOptions = {
    readOnly,
    minimap: { enabled: enableMinimap },
    scrollBeyondLastLine: false,
    fontSize: 14,
    fontFamily: "Monaco, Menlo, Ubuntu Mono, monospace",
    automaticLayout: true,
    padding: { top: 8, bottom: 8 },
    // tabSize, detectIndentation, and insertSpaces are now handled by applyIndentationSettings
    wordWrap: enableWordWrap ? "on" : "off",
    lineNumbers: showLineNumbers ? "on" : "off",
    folding: enableFolding,
    renderLineHighlight: readOnly ? "none" : "gutter",
    smoothScrolling: true,
    autoIndent: readOnly ? "none" : "full",
    scrollbar: {
      vertical: "auto",
      horizontal: "auto",
    },
    ...(rulers && rulers.length > 0 && { rulers }),
  };
  return (
    <div
      className={`
        h-full flex-1 overflow-hidden flex flex-col relative
        bg-[rgba(232,245,232,0.6)] dark:bg-[rgba(42,31,74,0.6)]
        transition-all duration-300 ease-in-out
    `}
    >
      <MonacoEditor
        language={language}
        value={value}
        theme={editorTheme}
        onChange={onChange}
        onMount={handleEditorDidMount}
        options={editorOptions}
      />
    </div>
  );
}
