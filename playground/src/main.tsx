import { StrictMode, Suspense, lazy } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import "./styles/index.scss";
const App = lazy(() => import("./App"));
import { ThemeProvider } from "./contexts";

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
  </StrictMode>
);
