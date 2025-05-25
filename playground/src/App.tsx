import { useState, useEffect } from "react";
import MonacoEditor, { type Monaco } from "@monaco-editor/react";
import { registerTypstLanguage } from "./typst-language";

// Sample typst code for testing
const SAMPLE_CODE = `= Introduction

This is a *sample* document to showcase Typst formatting.

== Math Example
$ x = (-b plus.minus sqrt(b^2 - 4 a c)) / (2 a) $

== Code Example
\`\`\`python
def hello_world():
    print("Hello, World!")
\`\`\`

#let custom_function(content) = {
  box(
    fill: rgb("#e8f4fd"),
    inset: 8pt,
    radius: 4pt,
    content
  )
}

#custom_function[This is a custom styled box!]
`;

// Compact format style options
const FORMAT_OPTIONS = {
  indentSize: 2,
  maxLineLength: 80,
  insertFinalNewline: true,
  trimTrailingWhitespace: true,
};

type TabType = "formatted" | "ast" | "ir";
type ThemeType = "light" | "dark";

function App() {
  const [sourceCode, setSourceCode] = useState(SAMPLE_CODE);
  const [activeTab, setActiveTab] = useState<TabType>("formatted");
  const [formatOptions, setFormatOptions] = useState(FORMAT_OPTIONS);
  const [formattedCode, setFormattedCode] = useState("");
  const [astOutput, setAstOutput] = useState("");
  const [irOutput, setIrOutput] = useState("");
  const [theme, setTheme] = useState<ThemeType>("light");
  const [monacoInstance, setMonacoInstance] = useState<Monaco | null>(null);

  // Update Monaco theme when theme changes
  useEffect(() => {
    if (monacoInstance) {
      monacoInstance.editor.setTheme(
        theme === "light" ? "typst-theme" : "vs-dark"
      );
    }
  }, [theme, monacoInstance]);

  // Reactive formatting - format code whenever source or options change
  useEffect(() => {
    const formatCode = async () => {
      try {
        // TODO: Replace with actual typstyle WASM/API call
        // For now, just simulate formatting by adding some basic formatting
        const mockFormatted = sourceCode
          .split("\n")
          .map((line) => line.trim())
          .join("\n");

        setFormattedCode(mockFormatted);

        // Mock AST output
        setAstOutput(
          JSON.stringify(
            {
              type: "document",
              children: [
                {
                  type: "heading",
                  level: 1,
                  content: "Introduction",
                },
                {
                  type: "paragraph",
                  content:
                    "This is a sample document to showcase Typst formatting.",
                },
              ],
            },
            null,
            2
          )
        );

        // Mock IR output
        setIrOutput(`Document(
  Heading(level: 1, "Introduction"),
  Paragraph("This is a ", Emphasis("sample"), " document..."),
  Heading(level: 2, "Math Example"),
  Math("x = (-b \\pm \\sqrt{b^2 - 4ac}) / (2a)"),
  // ... more IR nodes
)`);
      } catch (error) {
        console.error("Formatting error:", error);
      }
    };

    formatCode();
  }, [sourceCode, formatOptions]);

  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setSourceCode(value);
    }
  };

  // Handle Monaco Editor mounting with theme support
  const handleEditorDidMount = (_editor: unknown, monaco: Monaco) => {
    registerTypstLanguage(monaco);
    setMonacoInstance(monaco);
    // Set theme based on current theme state
    monaco.editor.setTheme(theme === "light" ? "typst-theme" : "vs-dark");
  };

  const toggleTheme = () => {
    setTheme((prev) => (prev === "light" ? "dark" : "light"));
  };

  const renderRightPanel = () => {
    const editorTheme = theme === "light" ? "light" : "vs-dark";

    switch (activeTab) {
      case "formatted":
        return (
          <MonacoEditor
            height="100%"
            language="typst"
            value={formattedCode}
            theme={editorTheme}
            options={{
              readOnly: true,
              minimap: { enabled: false },
              scrollBeyondLastLine: false,
              fontSize: 14,
              fontFamily: "Monaco, Menlo, Ubuntu Mono, monospace",
              automaticLayout: true,
              padding: { top: 8, bottom: 8 },
              scrollbar: {
                vertical: "hidden",
                horizontal: "hidden",
              },
            }}
          />
        );
      case "ast":
        return (
          <MonacoEditor
            height="100%"
            language="json"
            value={astOutput}
            theme={editorTheme}
            options={{
              readOnly: true,
              minimap: { enabled: false },
              scrollBeyondLastLine: false,
              fontSize: 14,
              fontFamily: "Monaco, Menlo, Ubuntu Mono, monospace",
              automaticLayout: true,
              padding: { top: 8, bottom: 8 },
              scrollbar: {
                vertical: "hidden",
                horizontal: "hidden",
              },
            }}
          />
        );
      case "ir":
        return (
          <MonacoEditor
            height="100%"
            language="text"
            value={irOutput}
            theme={editorTheme}
            options={{
              readOnly: true,
              minimap: { enabled: false },
              scrollBeyondLastLine: false,
              fontSize: 14,
              fontFamily: "Monaco, Menlo, Ubuntu Mono, monospace",
              automaticLayout: true,
              padding: { top: 8, bottom: 8 },
              scrollbar: {
                vertical: "hidden",
                horizontal: "hidden",
              },
            }}
          />
        );
      default:
        return null;
    }
  };

  return (
    <div className="h-screen flex flex-col app-bg" data-theme={theme}>
      {/* Header */}
      <div className="app-header">
        <div>
          <h1 className="header-title">Typstyle Playground</h1>
        </div>

        <div className="header-actions">
          {/* GitHub Repo Link */}
          <a
            href="https://github.com/enter-tainer/typstyle"
            target="_blank"
            rel="noopener noreferrer"
            className="github-link"
            title="View Typstyle on GitHub"
          >
            <svg
              width="20"
              height="20"
              fill="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
            </svg>
          </a>

          {/* Theme Toggle */}
          <button
            onClick={toggleTheme}
            className="theme-toggle"
            title={`Switch to ${theme === "light" ? "dark" : "light"} mode`}
          >
            {theme === "light" ? (
              <svg
                width="20"
                height="20"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                />
              </svg>
            ) : (
              <svg
                width="20"
                height="20"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                />
              </svg>
            )}
          </button>
        </div>
      </div>

      {/* Main Content */}
      <div className="main-content">
        {/* Left Panel - Source Code Editor */}
        <div className="editor-panel">
          <div className="panel-header">Source Code</div>
          <div className="editor-container">
            <MonacoEditor
              height="100%"
              language="typst"
              value={sourceCode}
              onChange={handleEditorChange}
              onMount={handleEditorDidMount}
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
                scrollbar: {
                  vertical: "hidden",
                  horizontal: "hidden",
                },
              }}
            />
          </div>
        </div>

        {/* Right Panel - Output */}
        <div className="output-panel">
          {/* Tabs */}
          <div className="glass-panel tabs-container">
            <div className="flex">
              {(["formatted", "ast", "ir"] as TabType[]).map((tab) => (
                <button
                  key={tab}
                  onClick={() => setActiveTab(tab)}
                  className={`tab-button ${activeTab === tab ? "active" : ""}`}
                >
                  {tab === "formatted" && "Formatted"}
                  {tab === "ast" && "AST"}
                  {tab === "ir" && "Pretty IR"}
                </button>
              ))}
            </div>
          </div>

          {/* Tab Content */}
          <div className="editor-container tabs-content">
            {renderRightPanel()}
          </div>
        </div>
      </div>

      {/* Format Options Panel - Moved to Bottom */}
      <div className="glass-panel format-options">
        <div className="options-grid">
          <div className="option-group">
            <label className="format-label">Indent:</label>
            <input
              type="number"
              min="1"
              max="8"
              value={formatOptions.indentSize}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  indentSize: parseInt(e.target.value),
                }))
              }
              className="format-input w-16"
            />
          </div>

          <div className="option-group">
            <label className="format-label">Line Length:</label>
            <input
              type="number"
              min="40"
              max="200"
              value={formatOptions.maxLineLength}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  maxLineLength: parseInt(e.target.value),
                }))
              }
              className="format-input w-20"
            />
          </div>

          <label className="checkbox-group">
            <input
              type="checkbox"
              checked={formatOptions.insertFinalNewline}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  insertFinalNewline: e.target.checked,
                }))
              }
            />
            <span className="format-label">Final Newline</span>
          </label>

          <label className="checkbox-group">
            <input
              type="checkbox"
              checked={formatOptions.trimTrailingWhitespace}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  trimTrailingWhitespace: e.target.checked,
                }))
              }
            />
            <span className="format-label">Trim Whitespace</span>
          </label>
        </div>
      </div>
    </div>
  );
}

export default App;
