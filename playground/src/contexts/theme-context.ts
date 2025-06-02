import { createContext } from "react";
import type { ThemeType } from "../types";

export interface ThemeContextType {
  theme: ThemeType;
  toggleTheme: () => void;
}

export const ThemeContext = createContext<ThemeContextType | undefined>(
  undefined,
);
