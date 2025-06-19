import { useEffect, useState } from "react";
import { ErrorIcon } from "./ui/Icons";

export interface FloatingErrorCardProps {
  error: string | null;
  onDismiss?: () => void;
}

export function FloatingErrorCard({ error }: FloatingErrorCardProps) {
  const [isVisible, setIsVisible] = useState(false);

  // Reset dismissed state and show animation when error changes
  useEffect(() => {
    if (error) {
      // Small delay to ensure animation triggers
      setTimeout(() => setIsVisible(true), 10);
    } else {
      setIsVisible(false);
    }
  }, [error]);

  if (!error) return null;

  return (
    <div
      className={`
        fixed bottom-6 left-6 right-6 z-50 flex justify-center
        transition-all duration-300 ease-out
        ${
          isVisible
            ? "transform translate-y-0 opacity-100"
            : "transform translate-y-4 opacity-0"
        }
      `}
    >
      <div
        className="
        bg-error/10 backdrop-blur-sm
        border border-error/20
        rounded-lg shadow-lg
        max-w-2xl w-full
      "
      >
        <div className="p-3">
          <div className="flex items-start gap-2">
            {/* Warning/Error Icon */}
            <div className="flex-shrink-0 mt-0.5">
              <ErrorIcon className="w-5 h-5 text-error" />
            </div>
            {/* Content */}
            <div className="flex-1 min-w-0">
              <h3 className="text-sm font-semibold text-error mb-1">
                Formatting Error
              </h3>
              <div className="text-sm text-error/80 font-mono whitespace-pre-wrap break-words">
                {error}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
