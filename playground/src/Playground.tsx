import { useState } from "react";
import { OutputEditor, SourceEditor } from "./components/editor";
import { FloatingErrorCard } from "./components/FloatingErrorCard";
import { SettingsPanel } from "./components/forms/SettingsPanel";
import { Header } from "./components/Header";
import { MainLayout } from "./components/MainLayout";
import { DEFAULT_FORMAT_OPTIONS } from "./constants";
import { useInitialSample, useScreenSize, useTypstFormatter } from "./hooks";

function Playground() {
  const [sourceCode, setSourceCode] = useState("");
  // Load initial sample document
  useInitialSample({ setSourceCode });
  const [formatOptions, setFormatOptions] = useState(DEFAULT_FORMAT_OPTIONS);

  // Custom hooks
  const screenSize = useScreenSize();
  const { formattedCode, astOutput, irOutput, error } = useTypstFormatter(
    sourceCode,
    formatOptions,
  );
  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setSourceCode(value);
    }
  };

  const handleSampleSelect = (content: string) => {
    setSourceCode(content);
  };

  const optionsPanel = (
    <SettingsPanel
      formatOptions={formatOptions}
      setFormatOptions={setFormatOptions}
    />
  );
  const sourcePanel = (
    <SourceEditor
      key="source-editor"
      value={sourceCode}
      onChange={handleEditorChange}
      lineLengthGuide={formatOptions.maxLineLength}
    />
  );
  const formattedPanel = (
    <OutputEditor
      key="output-formatted"
      content={formattedCode}
      language="typst"
      indentSize={formatOptions.indentSize}
      lineLengthGuide={formatOptions.maxLineLength}
    />
  );
  const astPanel = (
    <OutputEditor
      key="output-ast"
      content={astOutput}
      language="json"
      indentSize={4}
    />
  );
  const irPanel = (
    <OutputEditor
      key="output-ir"
      content={irOutput}
      language="python"
      indentSize={4}
    />
  );

  return (
    <div
      className="
        h-screen flex flex-col
        bg-gradient-to-br from-koishi-green-50 via-koishi-green-100 to-koishi-green-200
        dark:from-koishi-purple-900 dark:via-koishi-purple-800 dark:to-koishi-purple-700
      "
    >
      <Header onSampleSelect={handleSampleSelect} />

      <MainLayout
        screenSize={screenSize}
        optionsPanel={optionsPanel}
        sourcePanel={sourcePanel}
        formattedPanel={formattedPanel}
        astPanel={astPanel}
        irPanel={irPanel}
      />

      {/* Global floating error card */}
      <FloatingErrorCard error={error} />
    </div>
  );
}

export default Playground;
