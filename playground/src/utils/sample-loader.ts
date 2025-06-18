import { SAMPLE_DOCUMENTS, type SampleDocumentKey } from "@/constants";

export const fetchSampleDocument = async (
  filePath: string,
): Promise<string> => {
  const response = await fetch(filePath);
  if (!response.ok) {
    throw new Error(
      `Failed to load sample: ${response.status} ${response.statusText} from ${filePath}`,
    );
  }
  return response.text();
};

export const getSampleFileContent = async (
  sampleKey: SampleDocumentKey,
): Promise<string> => {
  const sample = SAMPLE_DOCUMENTS[sampleKey];
  if (!sample) {
    throw new Error(`Sample with key "${sampleKey}" not found.`);
  }
  return fetchSampleDocument((await sample.filePath).default);
};

export const getFallbackContent = (
  sampleKey: SampleDocumentKey | null,
  error: Error | string,
): string => {
  const errorMessage = error instanceof Error ? error.message : error;
  const sampleName =
    sampleKey && sampleKey in SAMPLE_DOCUMENTS
      ? SAMPLE_DOCUMENTS[sampleKey].name
      : "the requested sample";

  return `= Sample Document Loading Error

Failed to load: ${sampleName}
Error: ${errorMessage}

== Available Sample Documents
You can try selecting a different sample document from the dropdown.

${Object.values(SAMPLE_DOCUMENTS)
  .map((doc) => `- ${doc.name}: ${doc.description}`)
  .join("\n")}

Try refreshing the page or check your internet connection.`;
};
