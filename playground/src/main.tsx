import { StrictMode, Suspense, lazy } from "react";
import { createRoot } from "react-dom/client";
import { ThemeProvider } from "./contexts";
import "./styles/index.css";
import "./styles/_monaco.scss";

const App = lazy(() => import("./App"));

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Could not find root element");
}

createRoot(rootElement).render(
  <StrictMode>
    <Suspense fallback={<div>Loading...</div>}>
      <ThemeProvider>
        <App />
      </ThemeProvider>
    </Suspense>
  </StrictMode>,
);
