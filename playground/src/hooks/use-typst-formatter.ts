import { useDeferredValue, useEffect, useState } from "react";
import * as typstyle from "typstyle-wasm";
import type { FormatOptions, OutputType } from "@/types";

export interface Formatter {
  formattedCode: string;
  astOutput: string;
  irOutput: string;
  error: string | null;
  update: () => void;
}

export function useTypstFormatter(
  sourceCode: string,
  formatOptions: FormatOptions,
  activeOutput: OutputType,
): Formatter {
  const deferredSourceCode = useDeferredValue(sourceCode);
  const [formattedCode, setFormattedCode] = useState("");
  const [astOutput, setAstOutput] = useState("");
  const [irOutput, setIrOutput] = useState("");
  const [error, setError] = useState<string | null>(null);

  const formatCode = async () => {
    const config: typstyle.Config = {
      max_width: formatOptions.maxLineLength,
      tab_spaces: formatOptions.indentSize,
      blank_lines_upper_bound: 2, // Default value, not exposed in UI
      collapse_markup_spaces: formatOptions.collapseMarkupSpaces,
      reorder_import_items: formatOptions.reorderImportItems,
      wrap_text: formatOptions.wrapText,
    };

    try {
      // Only call the WASM function for the currently active output
      switch (activeOutput) {
        case "formatted": {
          const formatted = typstyle.format(deferredSourceCode, config);
          setFormattedCode(formatted);

          // Check for convergence on formatted output
          const secondFormatted = typstyle.format(formatted, config);
          if (secondFormatted !== formatted) {
            setError(
              "Format doesn't converge! " +
                "This means formatting the output again will result in a different output. " +
                "This is a bug in the formatter. " +
                "Please report it to https://github.com/Enter-tainer/typstyle with the input code.",
            );
          } else {
            setError(null);
          }
          break;
        }
        case "ast": {
          const ast = typstyle.parse(deferredSourceCode);
          setAstOutput(ast);
          setError(null);
          break;
        }
        case "ir": {
          const formatIr = typstyle.format_ir(deferredSourceCode, config);
          setIrOutput(formatIr);
          setError(null);
          break;
        }
      }
    } catch (error) {
      setError(error instanceof Error ? error.message : String(error));
      // Keep previous outputs on error
    }
  };

  useEffect(() => {
    formatCode();
  }, [deferredSourceCode, formatOptions, activeOutput]);

  return {
    formattedCode,
    astOutput,
    irOutput,
    error,
    update: formatCode,
  };
}
