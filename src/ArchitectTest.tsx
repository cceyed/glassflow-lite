import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

type AgentState = 'idle' | 'analyzing' | 'questioning' | 'designing' | 'complete' | 'error';

const stateEmojis: Record<AgentState, string> = {
  idle: '○',
  analyzing: '◐',
  questioning: '◑',
  designing: '●',
  complete: '✓',
  error: '✗'
};

export function ArchitectTest() {
  const [spec, setSpec] = useState('');
  const [state, setState] = useState<AgentState>('idle');
  const [response, setResponse] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleAnalyze = async () => {
    if (!spec.trim()) {
      setError('Please enter a specification');
      return;
    }

    setLoading(true);
    setError('');
    setResponse('');
    setState('analyzing');

    try {
      const result = await invoke<string>('architect_analyze', { spec });
      setResponse(result);
      setState('complete');
    } catch (err) {
      setError(String(err));
      setState('error');
    } finally {
      setLoading(false);
    }
  };

  const handleGetState = async () => {
    try {
      const currentState = await invoke<string>('architect_get_state');
      setState(currentState as AgentState);
    } catch (err) {
      setError(String(err));
    }
  };

  const handleCancel = async () => {
    try {
      await invoke('architect_cancel');
      setState('idle');
      setResponse('');
      setError('');
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <div className="min-h-screen bg-black text-white p-8">
      <div className="max-w-4xl mx-auto space-y-6">
        {/* Header */}
        <div className="text-center space-y-2">
          <h1 className="text-4xl font-bold">Architect Agent Test</h1>
          <p className="text-gray-400">Test the Architect Agent with OpenRouter (Grok-4-Fast)</p>
        </div>

        {/* State Display */}
        <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
          <div className="flex items-center gap-3">
            <span className="text-2xl">{stateEmojis[state]}</span>
            <div>
              <h3 className="text-lg font-semibold">Architect Agent</h3>
              <p className="text-sm text-gray-400">State: {state}</p>
            </div>
          </div>
        </div>

        {/* Input */}
        <div className="bg-gray-900 p-6 rounded-lg border border-gray-700 space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Project Specification
            </label>
            <textarea
              value={spec}
              onChange={(e) => setSpec(e.target.value)}
              placeholder="e.g., Build a React todo app with TypeScript and Tailwind CSS"
              className="w-full h-32 bg-gray-900 border border-gray-700 rounded-lg p-3 text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors"
              disabled={loading}
            />
          </div>

          <div className="flex gap-3">
            <button
              onClick={handleAnalyze}
              disabled={loading || !spec.trim()}
              className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed rounded-lg font-medium transition-colors"
            >
              {loading ? 'Analyzing...' : 'Analyze Specification'}
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

        {/* Response */}
        {response && (
          <div className="bg-blue-900/20 p-6 rounded-lg border border-blue-500">
            <h3 className="text-lg font-semibold text-white mb-3">LLM Response:</h3>
            <div className="text-gray-300 whitespace-pre-wrap">{response}</div>
          </div>
        )}

        {/* Error */}
        {error && (
          <div className="bg-red-900/20 p-6 rounded-lg border border-red-500">
            <h3 className="text-lg font-semibold text-red-400 mb-3">Error:</h3>
            <div className="text-red-300">{error}</div>
          </div>
        )}

        {/* Debug Info */}
        <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
          <div className="text-xs text-gray-500 space-y-1">
            <div>Current State: <span className="text-blue-400">{state}</span></div>
            <div>API: OpenRouter (x-ai/grok-4-fast:free)</div>
            <div>Status: {loading ? 'Processing...' : 'Ready'}</div>
          </div>
        </div>
      </div>
    </div>
  );
}
