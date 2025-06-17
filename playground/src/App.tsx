import Playground from "./Playground";
import { ThemeProvider } from "./contexts";

function App() {
  return (
    <ThemeProvider>
      <Playground />
    </ThemeProvider>
  );
}

export default App;
