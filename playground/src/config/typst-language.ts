import languageConfiguration from "@/assets/language-configuration.json";
import { setupLanguage } from "@/monaco";
import type { LanguageConfiguration, Monaco } from "@/monaco/types";

// Typst language configuration constants
const TYPST_LANGUAGE = {
  ID: "typst",
  SCOPE: "source.typst",
  GRAMMAR_URL: new URL("/typst.tmLanguage.json", import.meta.url),
} as const;

// Typst language definition for Monaco Editor
export const registerTypstLanguage = async (monaco: Monaco) => {
  await setupLanguage(
    monaco,
    TYPST_LANGUAGE.ID,
    TYPST_LANGUAGE.SCOPE,
    languageConfiguration as LanguageConfiguration,
    TYPST_LANGUAGE.GRAMMAR_URL,
  );
};
