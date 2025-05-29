import type { ReactNode } from "react";

export interface PanelProps {
  children: ReactNode;
  header?: string;
  className?: string;
}

export function Panel({ children, header, className = "" }: PanelProps) {
  return (
    <div className={`panel ${className}`}>
      {header && <div className="panel-header">{header}</div>}
      <div className="panel-content">{children}</div>
    </div>
  );
}
