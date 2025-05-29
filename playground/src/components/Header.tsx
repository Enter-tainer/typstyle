import { useTheme } from "../contexts";
import { SampleDocumentSelector } from "./SampleDocumentSelector";
import { GitHub, DarkMode, LightMode } from "@mui/icons-material";

interface HeaderProps {
  onSampleSelect: (content: string) => void;
}

export function Header({ onSampleSelect }: HeaderProps) {
  const { theme, toggleTheme } = useTheme();
  return (
    <div
      className="
      px-4 py-2 backdrop-blur-md flex items-center justify-between flex-shrink-0
      border-b border-[var(--glass-border)] shadow-[var(--shadow-soft)] relative
    "
    >
      <div className="flex items-center gap-4">
        <h1 className="text-2xl font-bold text-[var(--header-text)] m-0 drop-shadow-sm">
          Typstyle Playground
        </h1>
        <SampleDocumentSelector
          onSampleSelect={onSampleSelect}
          className="min-w-[200px] max-w-[300px]"
        />
      </div>

      <div className="flex items-center gap-3">
        {/* GitHub Repo Link */}
        <a
          href="https://github.com/enter-tainer/typstyle"
          target="_blank"
          rel="noopener noreferrer"
          className="btn-icon no-underline hover:no-underline"
          title="View Typstyle on GitHub"
        >
          <GitHub />
        </a>

        {/* Theme Toggle */}
        <button
          type="button"
          onClick={toggleTheme}
          className="btn-icon"
          title={`Switch to ${theme === "light" ? "dark" : "light"} mode`}
        >
          {theme === "light" ? <LightMode /> : <DarkMode />}
        </button>
      </div>
    </div>
  );
}
