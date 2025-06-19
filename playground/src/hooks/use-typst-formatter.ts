import { useEffect, useState } from "react";
import * as typstyle from "typstyle-wasm";
import type { FormatOptions } from "@/types";

export function useTypstFormatter(
  sourceCode: string,
  formatOptions: FormatOptions,
) {
  const [formattedCode, setFormattedCode] = useState("");
  const [astOutput, setAstOutput] = useState("");
  const [irOutput, setIrOutput] = useState("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
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
        const ast = typstyle.parse(sourceCode);
        const formatIr = typstyle.format_ir(sourceCode, config);
        const formatted = typstyle.format(sourceCode, config);

        setAstOutput(ast);
        setIrOutput(formatIr);
        setFormattedCode(formatted);

        if (typstyle.format(formatted, config) !== formatted) {
          setError(
            "Format doesn't converge! " +
              "This means formatting the output again will result in a different output. " +
              "This is a bug in the formatter. " +
              "Please report it to https://github.com/Enter-tainer/typstyle with the input code.",
          );
        } else {
          setError(null); // Clear error on success
        }
      } catch (error) {
        console.error("Formatting error:", error);
        setError(error instanceof Error ? error.message : String(error));
        // Keep previous outputs on error
      }
    };

    formatCode();
  }, [sourceCode, formatOptions]);

  return {
    formattedCode,
    astOutput,
    irOutput,
    error,
  };
}
