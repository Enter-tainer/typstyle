import { DEFAULT_FORMAT_OPTIONS } from "../../constants";
import type { FormatOptions } from "../../types";

interface FormatOptionsContentProps {
  formatOptions: FormatOptions;
  setFormatOptions: React.Dispatch<React.SetStateAction<FormatOptions>>;
}

export function FormatOptionsContent({
  formatOptions,
  setFormatOptions,
}: FormatOptionsContentProps) {
  const handleReset = () => {
    setFormatOptions(DEFAULT_FORMAT_OPTIONS);
  };
  return (
    <div className="p-2 overflow-y-auto flex-1">
      {/* Reset Button */}
      <div className="mb-3 pb-3 border-b border-[rgba(200, 230, 201, 0.9)] dark:border-[rgba(74, 63, 106, 0.9)]">
        <button type="button" onClick={handleReset} className="btn w-full">
          🔄 Reset to Defaults
        </button>
      </div>

      <div className="flex flex-wrap gap-3 items-center">
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="lineLengthSelect">Line Length:</label>
          <div className="flex gap-1 flex-shrink-0">
            <select
              id="lineLengthSelect"
              name="lineWidth"
              value={
                [40, 60, 80, 100, 120].includes(formatOptions.maxLineLength)
                  ? formatOptions.maxLineLength
                  : "custom"
              }
              onChange={(e) => {
                if (e.target.value !== "custom") {
                  setFormatOptions((prev) => ({
                    ...prev,
                    maxLineLength: Number.parseInt(e.target.value),
                  }));
                }
              }}
              className="w-14"
            >
              <option value={40}>40</option>
              <option value={60}>60</option>
              <option value={80}>80</option>
              <option value={100}>100</option>
              <option value={120}>120</option>
              <option value="custom">Custom</option>
            </select>
            <input
              id="lineLengthInput"
              type="number"
              min="40"
              max="200"
              aria-label="Custom Line Length"
              value={formatOptions.maxLineLength}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  maxLineLength: Number.parseInt(e.target.value),
                }))
              }
              className="w-14"
            />
          </div>
        </div>{" "}
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="indentSizeSelect">Indent:</label>
          <div className="flex gap-1 flex-shrink-0">
            <select
              id="indentSizeSelect"
              name="indentSize"
              value={
                [2, 4, 8].includes(formatOptions.indentSize)
                  ? formatOptions.indentSize
                  : "custom"
              }
              onChange={(e) => {
                if (e.target.value !== "custom") {
                  setFormatOptions((prev) => ({
                    ...prev,
                    indentSize: Number.parseInt(e.target.value),
                  }));
                }
              }}
              className="w-14"
            >
              <option value={2}>2</option>
              <option value={4}>4</option>
              <option value={8}>8</option>
              <option value="custom">Custom</option>
            </select>
            <input
              id="indentSizeInput"
              type="number"
              min="1"
              max="16"
              aria-label="Custom Indent Size"
              value={formatOptions.indentSize}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  indentSize: Number.parseInt(e.target.value),
                }))
              }
              className="w-14"
            />
          </div>
        </div>{" "}
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="collapseMarkupSpaces">Collapse Markup Spaces:</label>
          <input
            id="collapseMarkupSpaces"
            type="checkbox"
            checked={formatOptions.collapseMarkupSpaces}
            onChange={(e) =>
              setFormatOptions((prev) => ({
                ...prev,
                collapseMarkupSpaces: e.target.checked,
              }))
            }
          />
        </div>
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="reorderImportItems">Reorder Import Items:</label>
          <input
            id="reorderImportItems"
            type="checkbox"
            checked={formatOptions.reorderImportItems}
            onChange={(e) =>
              setFormatOptions((prev) => ({
                ...prev,
                reorderImportItems: e.target.checked,
              }))
            }
          />
        </div>
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="wrapText">Wrap Text:</label>
          <input
            id="wrapText"
            type="checkbox"
            checked={formatOptions.wrapText}
            onChange={(e) =>
              setFormatOptions((prev) => ({
                ...prev,
                wrapText: e.target.checked,
              }))
            }
          />
        </div>
      </div>
    </div>
  );
}
