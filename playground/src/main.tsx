import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import "./styles/index.scss";
import App from "./App.tsx";
import { ThemeProvider } from "./contexts";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ThemeProvider>
      <App />
    </ThemeProvider>
  </StrictMode>,
);
