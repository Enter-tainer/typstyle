import { SAMPLE_DOCUMENTS } from "@/constants";

interface LoadSampleOptions {
  onSuccess: (content: string) => void;
  onError?: (error: string) => void;
}

const fetchSampleDocument = async (filePath: URL | string): Promise<string> => {
  const response = await fetch(filePath);
  if (!response.ok) {
    throw new Error(
      `Failed to load sample: ${response.status} ${response.statusText} from ${filePath}`,
    );
  }
  return response.text();
};

const getSampleFileContent = async (sampleKey: string): Promise<string> => {
  const sample = SAMPLE_DOCUMENTS[sampleKey];
  if (!sample) {
    throw new Error(`Sample with key "${sampleKey}" not found.`);
  }
  return fetchSampleDocument(sample.filePath);
};

const getFallbackContent = (
  sampleKey: string,
  error: Error | string,
): string => {
  const errorMessage = error instanceof Error ? error.message : error;
  const sampleName =
    SAMPLE_DOCUMENTS[sampleKey]?.name ?? "the requested sample";

  return `= Sample Document Loading Error

Failed to load: ${sampleName}
Error: ${errorMessage}

== Available Sample Documents
You can try selecting a different sample document from the dropdown.

${Object.values(SAMPLE_DOCUMENTS)
  .map((doc) => `- ${doc.name}: ${doc.filePath}`)
  .join("\n")}

Try refreshing the page or check your internet connection.`;
};

export const loadSample = async (
  sampleKey: string,
  { onSuccess, onError }: LoadSampleOptions,
): Promise<void> => {
  try {
    const content = await getSampleFileContent(sampleKey);
    onSuccess(content);
  } catch (error) {
    const errorMessage =
      error instanceof Error ? error.message : "Unknown error";
    console.error(`Failed to load ${sampleKey} sample:`, error);

    onError?.(errorMessage);

    // Always provide fallback content
    const fallback = getFallbackContent(sampleKey, error as Error);
    onSuccess(fallback);
  }
};
