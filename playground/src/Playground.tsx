import { useState } from "react";
import { OutputEditor, SourceEditor } from "./components/editor";
import { FloatingErrorCard } from "./components/FloatingErrorCard";
import { SettingsPanel } from "./components/forms/SettingsPanel";
import { Header } from "./components/Header";
import { MainLayout } from "./components/MainLayout";
import { DEFAULT_FORMAT_OPTIONS } from "./constants";
import { useScreenSize, useTypstFormatter } from "./hooks";
import type { OutputType } from "./types";

function Playground() {
  const [sourceCode, setSourceCode] = useState("");
  const [formatOptions, setFormatOptions] = useState(DEFAULT_FORMAT_OPTIONS);
  const [activeOutput, setActiveOutput] = useState<OutputType>("formatted");

  // Custom hooks
  const screenSize = useScreenSize();
  const formatter = useTypstFormatter(sourceCode, formatOptions, activeOutput);
  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setSourceCode(value);
    }
  };

  const handleSampleSelect = (content: string) => {
    setSourceCode(content);
  };

  const handleActiveTabChange = (tabId: string) => {
    // Only update activeOutput if the tab is an output type
    if (tabId === "formatted" || tabId === "ast" || tabId === "ir") {
      setActiveOutput(tabId);
    }
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
      content={formatter.formattedCode}
      language="typst"
      indentSize={formatOptions.indentSize}
      lineLengthGuide={formatOptions.maxLineLength}
    />
  );
  const astPanel = (
    <OutputEditor
      key="output-ast"
      content={formatter.astOutput}
      language="json"
      indentSize={4}
    />
  );
  const irPanel = (
    <OutputEditor
      key="output-ir"
      content={formatter.irOutput}
      language="python"
      indentSize={4}
    />
  );

  return (
    <div className="h-screen flex flex-col">
      <Header onSampleSelect={handleSampleSelect} />

      <MainLayout
        screenSize={screenSize}
        optionsPanel={optionsPanel}
        sourcePanel={sourcePanel}
        formattedPanel={formattedPanel}
        astPanel={astPanel}
        irPanel={irPanel}
        onActiveTabChange={handleActiveTabChange}
      />

      {/* Global floating error card */}
      <FloatingErrorCard error={formatter.error} />
    </div>
  );
}

export default Playground;
