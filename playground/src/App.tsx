import { useState } from "react";
import {
  FormatOptionsContent,
  Header,
  MainLayout,
  OutputEditor,
  SourceEditor,
} from "./components";
import { DEFAULT_FORMAT_OPTIONS } from "./constants";
import { useTheme } from "./contexts";
import {
  useInitialSample,
  useMonacoEditor,
  useScreenSize,
  useTypstFormatter,
} from "./hooks";

function App() {
  const [sourceCode, setSourceCode] = useState("");
  // Load initial sample document
  useInitialSample({ setSourceCode });
  const [formatOptions, setFormatOptions] = useState(DEFAULT_FORMAT_OPTIONS);
  const { theme } = useTheme();

  // Custom hooks
  const screenSize = useScreenSize();
  const { formattedCode, astOutput, irOutput } = useTypstFormatter(
    sourceCode,
    formatOptions,
  );
  const { handleEditorDidMount } = useMonacoEditor(theme);

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

      <MainLayout
        screenSize={screenSize}
        optionsPanel={optionsPanel}
        sourcePanel={sourcePanel}
        formattedPanel={formattedPanel}
        astPanel={astPanel}
        irPanel={irPanel}
      />
    </div>
  );
}

export default App;
