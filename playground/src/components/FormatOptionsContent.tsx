import { DEFAULT_FORMAT_OPTIONS } from "../constants";
import type { FormatOptions } from "../types";

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
      <div className="mb-3 pb-3 border-b border-[var(--glass-border)]">
        <button
          type="button"
          onClick={handleReset}
          className="
            w-full px-3 py-2 text-sm font-medium
            bg-[var(--theme-toggle-bg)] text-[var(--theme-toggle-text)]
            border border-[var(--glass-border)]
            rounded-lg shadow-[var(--shadow-soft)]
            hover:bg-[var(--theme-toggle-hover-bg)] hover:shadow-[var(--shadow-medium)]
            active:scale-95
            disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none
            transition-all duration-200
            focus:outline-none focus:ring-2 focus:ring-[var(--tab-active-border)] focus:ring-offset-2
          "
        >
          🔄 Reset to Defaults
        </button>
      </div>

      <div className="flex flex-wrap gap-3 items-center">
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="lineLength" className="format-label">
            Line Length:
          </label>
          <div className="flex gap-1 flex-shrink-0">
            <select
              id="lineLength"
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
              className="format-input w-14"
            >
              <option value={40}>40</option>
              <option value={60}>60</option>
              <option value={80}>80</option>
              <option value={100}>100</option>
              <option value={120}>120</option>
              <option value="custom">Custom</option>
            </select>
            <input
              type="number"
              min="40"
              max="200"
              value={formatOptions.maxLineLength}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  maxLineLength: Number.parseInt(e.target.value),
                }))
              }
              className="format-input w-14"
            />
          </div>
        </div>{" "}
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="indentSize" className="format-label">
            Indent:
          </label>
          <div className="flex gap-1 flex-shrink-0">
            <select
              id="indentSize"
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
              className="format-input w-14"
            >
              <option value={2}>2</option>
              <option value={4}>4</option>
              <option value={8}>8</option>
              <option value="custom">Custom</option>
            </select>
            <input
              type="number"
              min="1"
              max="16"
              value={formatOptions.indentSize}
              onChange={(e) =>
                setFormatOptions((prev) => ({
                  ...prev,
                  indentSize: Number.parseInt(e.target.value),
                }))
              }
              className="format-input w-14"
            />
          </div>
        </div>{" "}
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="collapseMarkupSpaces" className="format-label">
            Collapse Markup Spaces:
          </label>
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
            className="format-checkbox"
          />
        </div>
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="reorderImportItems" className="format-label">
            Reorder Import Items:
          </label>
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
            className="format-checkbox"
          />
        </div>
        <div className="flex items-center justify-between w-full min-w-[200px] gap-2">
          <label htmlFor="wrapText" className="format-label">
            Wrap Text:
          </label>
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
            className="format-checkbox"
          />
        </div>
      </div>
    </div>
  );
}
