// Generic language registry utilities for Monaco Editor with TextMate grammar support

import * as oniguruma from "vscode-oniguruma";
import * as vsctm from "vscode-textmate";
import { CDN_URLS } from "./constants";
import { wireTmGrammar } from "./textmate";
import type { LanguageConfiguration, Monaco } from "./types";

export type GrammarLoader = (
  scopeName: string,
) => Promise<vsctm.IRawGrammar | null>;

// Cached oniguruma lib instance to avoid multiple initializations
let onigurumaLibPromise: Promise<{
  createOnigScanner(patterns: string[]): oniguruma.OnigScanner;
  createOnigString(s: string): oniguruma.OnigString;
}> | null = null;

/**
 * Creates and initializes the vscode-oniguruma library for TextMate grammar support
 */
const createOnigurumaLib = async () => {
  if (onigurumaLibPromise) {
    return onigurumaLibPromise;
  }

  onigurumaLibPromise = (async () => {
    try {
      const response = await fetch(new URL(CDN_URLS.ONIGURUMA_WASM));

      if (!response.ok) {
        throw new Error(`Failed to fetch oniguruma WASM: ${response.status}`);
      }

      await oniguruma.loadWASM(await response.arrayBuffer());
      return {
        createOnigScanner(patterns: string[]) {
          return new oniguruma.OnigScanner(patterns);
        },
        createOnigString(s: string) {
          return new oniguruma.OnigString(s);
        },
      };
    } catch (error) {
      onigurumaLibPromise = null; // Reset cache on error
      throw new Error(`Failed to initialize oniguruma library: ${error}`);
    }
  })();

  return onigurumaLibPromise;
};

/**
 * Creates a TextMate grammar registry with custom grammar loaders
 */
export const createLanguageRegistry = async (grammarLoader: GrammarLoader) => {
  const onigLib = createOnigurumaLib();

  return new vsctm.Registry({
    onigLib,
    loadGrammar: grammarLoader,
  });
};

/**
 * Creates a grammar loader function that handles multiple language scopes
 * @param scopeHandlers Record mapping scope names to functions that return grammar promises
 * @returns A grammar loader function compatible with TextMate registry
 */
export const createGrammarLoader = (
  scopeHandlers: Record<string, () => Promise<vsctm.IRawGrammar | null>>,
): GrammarLoader => {
  return async (scopeName: string) => {
    const handler = scopeHandlers[scopeName];
    if (handler) {
      try {
        return await handler();
      } catch (error) {
        console.error(`Error loading grammar for scope ${scopeName}:`, error);
        return null;
      }
    }
    console.warn(`Unknown scope name: ${scopeName}`);
    return null;
  };
};

/**
 * Creates a grammar from a URL
 */
const createGrammarFromUrl = async (
  grammarUrl: URL,
): Promise<vsctm.IRawGrammar | null> => {
  try {
    const data = await fetch(grammarUrl);

    if (!data.ok) {
      console.error(
        `Failed to fetch grammar from ${grammarUrl.href}: ${data.status}`,
      );
      return null;
    }

    const text = await data.text();
    return vsctm.parseRawGrammar(text, grammarUrl.href);
  } catch (error) {
    console.error(`Error loading grammar from ${grammarUrl.href}:`, error);
    return null;
  }
};

/**
 * Complete language setup: register language and wire TextMate grammar from URL
 */
export const setupLanguage = async (
  monaco: Monaco,
  languageId: string,
  scopeName: string,
  languageConfiguration: LanguageConfiguration,
  grammarUrl: URL,
) => {
  try {
    // Register the language with Monaco
    monaco.languages.register({ id: languageId });
    monaco.languages.setLanguageConfiguration(
      languageId,
      languageConfiguration,
    );

    // Create grammar loader for this specific language
    const grammarLoader: GrammarLoader = async (requestedScopeName) => {
      if (requestedScopeName === scopeName) {
        return await createGrammarFromUrl(grammarUrl);
      }
      console.warn(`Unknown scope name: ${requestedScopeName}`);
      return null;
    };

    // Create the registry and wire the grammar
    const registry = await createLanguageRegistry(grammarLoader);
    await wireTmGrammar(monaco, registry, languageId, scopeName);

    console.log(`Successfully set up language: ${languageId}`);
  } catch (error) {
    console.error(`Failed to set up language ${languageId}:`, error);
    throw error;
  }
};
