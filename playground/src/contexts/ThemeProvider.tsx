import { useCallback, useEffect, useState } from "react";
import type { ThemeType } from "@/types";
import { ThemeContext } from "./theme-context";

interface ThemeProviderProps {
  children: React.ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  // Initialize theme with saved preference, defaulting to light
  const [theme, setTheme] = useState<ThemeType>(() => {
    const savedTheme = localStorage.getItem("theme") as ThemeType | null;
    return savedTheme && (savedTheme === "light" || savedTheme === "dark")
      ? savedTheme
      : "light";
  });

  const toggleTheme = useCallback(() => {
    setTheme((prev) => (prev === "light" ? "dark" : "light"));
  }, []);

  // Apply theme to document root and save to localStorage
  useEffect(() => {
    const root = document.documentElement;
    root.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}
