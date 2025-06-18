import { useTheme } from "@/hooks";
import { SampleDocumentSelector } from "./forms/SampleDocumentSelector";
import { DarkModeIcon, GitHubIcon, LightModeIcon } from "./ui/Icons";

interface HeaderProps {
  onSampleSelect: (content: string) => void;
}

export function Header({ onSampleSelect }: HeaderProps) {
  const { theme, toggleTheme } = useTheme();
  return (
    <div className="navbar bg-base-100 shadow-sm">
      <div className="flex-none">
        <h1 className="text-2xl font-bold text-primary m-2 drop-shadow-sm">
          Typstyle Playground
        </h1>
      </div>

      <div className="flex-1 ml-4">
        <SampleDocumentSelector
          onSampleSelect={onSampleSelect}
          className="min-w-[200px] max-w-[300px]"
        />
      </div>

      <div className="flex-none">
        {/* GitHub Repo Link */}
        <a
          href="https://github.com/enter-tainer/typstyle"
          target="_blank"
          rel="noopener noreferrer"
          className="btn btn-ghost btn-circle btn-sm"
          title="View Typstyle on GitHub"
        >
          <GitHubIcon />
        </a>

        {/* Theme Toggle */}
        <button
          type="button"
          onClick={toggleTheme}
          className="btn btn-ghost btn-circle btn-sm"
          title={`Switch to ${theme === "light" ? "dark" : "light"} mode`}
        >
          {theme === "light" ? <LightModeIcon /> : <DarkModeIcon />}
        </button>
      </div>
    </div>
  );
}
