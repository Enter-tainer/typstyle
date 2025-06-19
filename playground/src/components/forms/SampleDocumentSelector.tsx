import { useState } from "react";
import { SAMPLE_DOCUMENTS, type SampleDocumentKey } from "../../constants";
import {
  getFallbackContent,
  getSampleFileContent,
} from "../../utils/sample-loader";

interface SampleDocumentSelectorProps {
  onSampleSelect: (content: string) => void;
  className?: string;
}

export function SampleDocumentSelector({
  onSampleSelect,
  className = "",
}: SampleDocumentSelectorProps) {
  const [selectedSample, setSelectedSample] = useState<SampleDocumentKey | "">(
    "",
  );
  const [error, setError] = useState<string | null>(null);

  const loadSampleDocument = async (sampleKey: SampleDocumentKey) => {
    setError(null);
    try {
      const content = await getSampleFileContent(sampleKey);
      onSampleSelect(content);
      setSelectedSample(sampleKey);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Unknown error";
      console.error("Error loading sample document:", err);
      setError(errorMessage);
      const fallback = getFallbackContent(sampleKey, errorMessage);
      onSampleSelect(fallback);
    }
  };

  const handleSampleChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const value = e.target.value as SampleDocumentKey | "";
    if (value && value in SAMPLE_DOCUMENTS) {
      loadSampleDocument(value);
    } else {
      setSelectedSample("");
      setError(null);
    }
  };

  return (
    <div className={className}>
      <div className="flex items-center gap-2">
        <select
          value={selectedSample}
          onChange={handleSampleChange}
          className="select select-sm select-primary w-64"
          title={
            selectedSample && selectedSample in SAMPLE_DOCUMENTS
              ? SAMPLE_DOCUMENTS[selectedSample].description
              : "📄 Choose a sample document to load"
          }
        >
          <option value="" disabled>
            Select a sample...
          </option>
          {Object.entries(SAMPLE_DOCUMENTS).map(([key, sample]) => (
            <option key={key} value={key} title={sample.description}>
              {sample.name}
            </option>
          ))}
        </select>

        <button
          type="button"
          onClick={() => {
            setSelectedSample("");
            setError(null);
            onSampleSelect("");
          }}
          className="btn btn-square btn-sm"
          title="Clear document and start fresh"
        >
          🗑️
        </button>

        {/* Error message moved here, to the right of the button */}
        {error && (
          <div className="alert alert-error alert-outline text-xs p-2">
            ⚠️ {error}
          </div>
        )}
      </div>
    </div>
  );
}
