import { ThemeProvider } from "./contexts";
import Playground from "./Playground";

function App() {
  return (
    <ThemeProvider>
      <Playground />
    </ThemeProvider>
  );
}

export default App;
