import { useArchitect } from '../hooks/useArchitect';
import { QuestionDisplay } from './QuestionDisplay';

const stateEmojis: Record<string, string> = {
  idle: '○',
  analyzing: '◐',
  questioning: '◑',
  designing: '●',
  complete: '✓',
  error: '✗'
};

export function ArchitectTest() {
  const {
    agentState,
    reasoningEntries,
    currentQuestion,
    plan,
    confidence,
    progress,
    elapsed,
    analyze,
    answer,
    cancel,
    getState,
  } = useArchitect();

  const handleAnalyze = async () => {
    const spec = (document.getElementById('spec-input') as HTMLTextAreaElement)?.value;
    if (!spec?.trim()) {
      alert('Please enter a specification');
      return;
    }

    try {
      await analyze(spec);
    } catch (err) {
      console.error('Analysis failed:', err);
    }
  };

  const handleGetState = async () => {
    try {
      const state = await getState();
      console.log('Current state:', state);
    } catch (err) {
      console.error('Get state failed:', err);
    }
  };

  const handleCancel = async () => {
    try {
      await cancel();
    } catch (err) {
      console.error('Cancel failed:', err);
    }
  };

  const handleAnswer = async (answerText: string) => {
    try {
      await answer(answerText);
    } catch (err) {
      console.error('Answer failed:', err);
    }
  };

  return (
    <div className="min-h-screen bg-black text-white p-8">
      <div className="max-w-4xl mx-auto space-y-6">
        {/* Header */}
        <div className="text-center space-y-2">
          <h1 className="text-4xl font-bold">Architect Agent Test (With Events)</h1>
          <p className="text-gray-400">Testing Tauri Events + Zustand Store</p>
        </div>

        {/* State Display */}
        <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
          <div className="flex items-center gap-3">
            <span className="text-2xl">{stateEmojis[agentState] || '?'}</span>
            <div>
              <h3 className="text-lg font-semibold">Architect Agent</h3>
              <p className="text-sm text-gray-400">State: {agentState}</p>
            </div>
          </div>
          {progress > 0 && (
            <div className="mt-3">
              <div className="w-full bg-gray-700 rounded-full h-2">
                <div 
                  className="bg-blue-500 h-2 rounded-full transition-all"
                  style={{ width: `${progress}%` }}
                />
              </div>
              <p className="text-xs text-gray-400 mt-1">Progress: {progress}%</p>
            </div>
          )}
          {elapsed > 0 && (
            <p className="text-xs text-gray-400 mt-2">Elapsed: {elapsed}s</p>
          )}
        </div>

        {/* Input */}
        <div className="bg-gray-900 p-6 rounded-lg border border-gray-700 space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Project Specification
            </label>
            <textarea
              id="spec-input"
              placeholder="e.g., Build a React todo app with TypeScript and Tailwind CSS"
              className="w-full h-32 bg-gray-900 border border-gray-700 rounded-lg p-3 text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors"
              disabled={agentState === 'analyzing'}
            />
          </div>

          <div className="flex gap-3">
            <button
              onClick={handleAnalyze}
              disabled={agentState === 'analyzing'}
              className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed rounded-lg font-medium transition-colors"
            >
              {agentState === 'analyzing' ? 'Analyzing...' : 'Analyze Specification'}
            </button>
            <button
              onClick={handleGetState}
              className="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg font-medium transition-colors"
            >
              Get State
            </button>
            <button
              onClick={handleCancel}
              className="px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg font-medium transition-colors"
            >
              Cancel
            </button>
          </div>
        </div>

        {/* Reasoning Stream */}
        {reasoningEntries.length > 0 && (
          <div className="bg-gray-900 p-6 rounded-lg border border-gray-700">
            <h3 className="text-lg font-semibold text-white mb-3">Reasoning Stream:</h3>
            <div className="space-y-2 max-h-64 overflow-y-auto">
              {reasoningEntries.map((entry, idx) => (
                <div key={idx} className="text-sm">
                  <span className="text-gray-500">[{entry.type}]</span>{' '}
                  <span className="text-gray-300">{entry.content}</span>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Question Display */}
        {currentQuestion && agentState === 'questioning' && (
          <QuestionDisplay
            question={currentQuestion}
            questionNumber={1} // TODO: Get actual question number from state
            totalQuestions={5} // TODO: Get total from state
            onAnswer={handleAnswer}
          />
        )}

        {/* Plan Display */}
        {plan && (
          <div className="bg-green-900/20 p-6 rounded-lg border border-green-500">
            <h3 className="text-lg font-semibold text-green-400 mb-3">Architecture Plan:</h3>
            <div className="text-gray-300 space-y-2">
              <p><strong>Project:</strong> {plan.projectName || 'N/A'}</p>
              <p><strong>Components:</strong> {plan.components?.length || 0}</p>
              <p><strong>Decisions:</strong> {plan.decisions?.length || 0}</p>
              {confidence > 0 && (
                <p><strong>Confidence:</strong> {confidence.toFixed(1)}%</p>
              )}
            </div>
          </div>
        )}

        {/* Debug Info */}
        <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
          <div className="text-xs text-gray-500 space-y-1">
            <div>Current State: <span className="text-blue-400">{agentState}</span></div>
            <div>Reasoning Entries: <span className="text-blue-400">{reasoningEntries.length}</span></div>
            <div>Has Question: <span className="text-blue-400">{currentQuestion ? 'Yes' : 'No'}</span></div>
            <div>Has Plan: <span className="text-blue-400">{plan ? 'Yes' : 'No'}</span></div>
            <div>API: OpenRouter (x-ai/grok-4-fast:free)</div>
          </div>
        </div>
      </div>
    </div>
  );
}
