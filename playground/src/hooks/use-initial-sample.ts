import type { SampleDocumentKey } from "@/constants";
import {
  getFallbackContent,
  getSampleFileContent,
} from "@/utils/sample-loader";
import { useEffect } from "react";

interface UseInitialSampleProps {
  setSourceCode: (code: string) => void;
  sampleName?: SampleDocumentKey;
}

export function useInitialSample({
  setSourceCode,
  sampleName = "basic",
}: UseInitialSampleProps): void {
  useEffect(() => {
    const loadDefaultSample = async () => {
      try {
        const content = await getSampleFileContent(sampleName);
        setSourceCode(content);
      } catch (error) {
        console.error(`Failed to load ${sampleName} sample:`, error);
        setSourceCode(getFallbackContent(sampleName, error as Error));
      }
    };

    loadDefaultSample();
  }, [sampleName, setSourceCode]);
}
