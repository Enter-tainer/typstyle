import type { FormatOptions } from "@/types";
import { useEffect, useState } from "react";
import * as typstyle from "typstyle-wasm";

export function useTypstFormatter(
  sourceCode: string,
  formatOptions: FormatOptions,
) {
  const [formattedCode, setFormattedCode] = useState("");
  const [astOutput, setAstOutput] = useState("");
  const [irOutput, setIrOutput] = useState("");

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
      } catch (error) {
        console.error("Formatting error:", error);
      }
    };

    formatCode();
  }, [sourceCode, formatOptions]);

  return {
    formattedCode,
    astOutput,
    irOutput,
  };
}
