import { useTheme } from "../contexts";
import { SampleDocumentSelector } from "./SampleDocumentSelector";

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
          className="
            flex items-center justify-center p-3
            bg-[var(--theme-toggle-bg)] text-[var(--theme-toggle-text)]
            border border-[var(--glass-border)] rounded-xl backdrop-blur-md
            transition-all duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] cursor-pointer
            relative overflow-hidden no-underline
            hover:bg-[var(--theme-toggle-hover-bg)] hover:shadow-[var(--shadow-medium)]
            hover:no-underline active:transition-all active:duration-100 active:ease-in
          "
          title="View Typstyle on GitHub"
        >
          <svg
            width="20"
            height="20"
            fill="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
          </svg>
        </a>

        {/* Theme Toggle */}
        <button
          type="button"
          onClick={toggleTheme}
          className="
            flex items-center justify-center p-3
            bg-[var(--theme-toggle-bg)] text-[var(--theme-toggle-text)]
            border border-[var(--glass-border)] rounded-xl backdrop-blur-md
            transition-all duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] cursor-pointer
            relative
            hover:bg-[var(--theme-toggle-hover-bg)] hover:shadow-[var(--shadow-medium)]
            active:transition-all active:duration-100 active:ease-in
          "
          title={`Switch to ${theme === "light" ? "dark" : "light"} mode`}
        >
          {theme === "light" ? (
            <svg
              width="20"
              height="20"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
              />
            </svg>
          ) : (
            <svg
              width="20"
              height="20"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
              />
            </svg>
          )}
        </button>
      </div>
    </div>
  );
}
