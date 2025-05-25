import { useEffect, useState } from "react";
import {
  FormatOptionsContent,
  Header,
  OutputEditor,
  Panel,
  SourceEditor,
  Tab,
  Tabs,
} from "./components";
import { DEFAULT_FORMAT_OPTIONS } from "./constants";
import { useTheme } from "./contexts";
import { useMonacoEditor, useTypstFormatter } from "./hooks";
import { ScreenSize } from "./types";
import type { ScreenSizeType } from "./types";
import { getFallbackContent, getSampleFileContent } from "./utils/sampleLoader";

function App() {
  const [sourceCode, setSourceCode] = useState("");
  const [formatOptions, setFormatOptions] = useState(DEFAULT_FORMAT_OPTIONS);
  const [screenSize, setScreenSize] = useState<ScreenSizeType>(ScreenSize.Wide);
  const { theme } = useTheme();

  // Custom hooks
  const { formattedCode, astOutput, irOutput } = useTypstFormatter(
    sourceCode,
    formatOptions,
  );
  const { handleEditorDidMount } = useMonacoEditor(theme);

  // Load default sample document on app start
  useEffect(() => {
    const loadDefaultSample = async () => {
      try {
        const content = await getSampleFileContent("basic");
        setSourceCode(content);
      } catch (error) {
        console.error("Failed to load default sample:", error);
        setSourceCode(getFallbackContent("basic", error as Error));
      }
    };

    loadDefaultSample();
  }, []);

  // Detect screen size for 3 responsive layouts
  useEffect(() => {
    const updateScreenSize = () => {
      const width = window.innerWidth;
      if (width >= 1200) {
        setScreenSize(ScreenSize.Wide);
      } else if (width >= 768) {
        setScreenSize(ScreenSize.Medium);
      } else {
        setScreenSize(ScreenSize.Thin);
      }
    };

    updateScreenSize();
    window.addEventListener("resize", updateScreenSize);

    return () => window.removeEventListener("resize", updateScreenSize);
  }, []);

  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setSourceCode(value);
    }
  };

  const handleSampleSelect = (content: string) => {
    setSourceCode(content);
  };

  const optionsPanel = (
    <FormatOptionsContent
      formatOptions={formatOptions}
      setFormatOptions={setFormatOptions}
    />
  );
  const sourcePanel = (
    <SourceEditor
      key="persistent-source-editor"
      sourceCode={sourceCode}
      onChange={handleEditorChange}
      onMount={handleEditorDidMount}
      formatOptions={formatOptions}
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
    <div className="h-screen flex flex-col bg-gradient-to-br from-[var(--bg-gradient-from)] via-[var(--bg-gradient-via)] to-[var(--bg-gradient-to)]">
      <Header onSampleSelect={handleSampleSelect} />

      {/* Content Container - 3 Responsive Layouts */}
      <div className="flex overflow-hidden min-h-0 h-full p-4 gap-2">
        {/* Wide Layout: 3 Columns */}
        {screenSize === ScreenSize.Wide && (
          <>
            <Panel header="Format Options" className="w-[240px] flex-none">
              {optionsPanel}
            </Panel>
            <Panel header="Source Code" className="flex-1">
              {sourcePanel}
            </Panel>
            <Tabs defaultActiveTab="formatted" className="flex-1">
              <Tab id="formatted" label="Formatted">
                {formattedPanel}
              </Tab>
              <Tab id="ast" label="AST">
                {astPanel}
              </Tab>
              <Tab id="ir" label="Pretty IR">
                {irPanel}
              </Tab>
            </Tabs>
          </>
        )}

        {/* Medium Layout: 2 Columns (Equal 1:1) */}
        {screenSize === ScreenSize.Medium && (
          <>
            <Panel className="flex-1">
              <Tabs defaultActiveTab="source">
                <Tab id="options" label="Options">
                  {optionsPanel}
                </Tab>
                <Tab id="source" label="Source">
                  {sourcePanel}
                </Tab>
              </Tabs>
            </Panel>
            <Tabs defaultActiveTab="formatted" className="flex-1">
              <Tab id="formatted" label="Formatted">
                {formattedPanel}
              </Tab>
              <Tab id="ast" label="AST">
                {astPanel}
              </Tab>
              <Tab id="ir" label="Pretty IR">
                {irPanel}
              </Tab>
            </Tabs>
          </>
        )}

        {/* Thin Layout: 1 Column (Full Width) */}
        {screenSize === ScreenSize.Thin && (
          <Tabs defaultActiveTab="source" className="flex-1">
            <Tab id="options" label="Options">
              {optionsPanel}
            </Tab>
            <Tab id="source" label="Source">
              {sourcePanel}
            </Tab>
            <Tab id="formatted" label="Formatted">
              {formattedPanel}
            </Tab>
            <Tab id="ast" label="AST">
              {astPanel}
            </Tab>
            <Tab id="ir" label="Pretty IR">
              {irPanel}
            </Tab>
          </Tabs>
        )}
      </div>
    </div>
  );
}

export default App;
