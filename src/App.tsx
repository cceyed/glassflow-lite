import { RevampedPipelineUI } from "./components/RevampedPipelineUI";
import { ErrorBoundary } from "./components/ErrorBoundary";

function App() {
  return (
    <ErrorBoundary>
      <RevampedPipelineUI />
    </ErrorBoundary>
  );
}

export default App;
