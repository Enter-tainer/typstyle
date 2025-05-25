import type { ReactNode } from "react";

export interface PanelProps {
  children: ReactNode;
  header?: string;
  className?: string;
}

export function Panel({ children, header, className = "" }: PanelProps) {
  return (
    <div
      className={`
      bg-[var(--glass-bg)] backdrop-blur-md border border-[var(--glass-border)]
      rounded-2xl shadow-[var(--shadow-medium)] relative overflow-hidden
      transition-all duration-300 ease-in-out
      hover:shadow-[var(--shadow-strong)]
      flex flex-col h-full min-h-0
      ${className}
    `}
    >
      {header && (
        <div
          className="
          px-5 py-2 bg-[var(--panel-header-bg)] text-[var(--panel-header-text)]
          text-sm font-semibold backdrop-blur-md border border-[var(--glass-border)]
          border-b-0 rounded-t-2xl flex-shrink-0 relative overflow-hidden
        "
        >
          {header}
        </div>
      )}
      <div className="flex-1 overflow-hidden flex flex-col">{children}</div>
    </div>
  );
}
