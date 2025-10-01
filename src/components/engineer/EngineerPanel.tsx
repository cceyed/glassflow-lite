// T054: EngineerPanel component
import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface EngineerPanelProps {
  architecturePlan: any;
  onComplete?: (output: any) => void;
}

export const EngineerPanel: React.FC<EngineerPanelProps> = ({ architecturePlan, onComplete }) => {
  const [state, setState] = useState<string>('idle');
  const [progress, setProgress] = useState<number>(0);
  const [currentFile, setCurrentFile] = useState<string | null>(null);
  const [reasoning, setReasoning] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Listen for state changes
    const unsubscribeState = listen('engineer:state_changed', (event: any) => {
      setState(event.payload.newState);
    });

    // Listen for progress updates
    const unsubscribeProgress = listen('engineer:progress', (event: any) => {
      setProgress(event.payload.percentage);
      setCurrentFile(event.payload.currentFile);
    });

    // Listen for reasoning
    const unsubscribeReasoning = listen('engineer:reasoning', (event: any) => {
      setReasoning((prev) => [...prev, event.payload.content]);
    });

    // Listen for errors
    const unsubscribeError = listen('engineer:error', (event: any) => {
      setError(event.payload.message);
    });

    // Listen for completion
    const unsubscribeComplete = listen('engineer:complete', (event: any) => {
      if (onComplete) {
        onComplete(event.payload);
      }
    });

    return () => {
      unsubscribeState.then((fn) => fn());
      unsubscribeProgress.then((fn) => fn());
      unsubscribeReasoning.then((fn) => fn());
      unsubscribeError.then((fn) => fn());
      unsubscribeComplete.then((fn) => fn());
    };
  }, [onComplete]);

  const startGeneration = async () => {
    try {
      const config = {
        outputDirectory: './output',
        maxConcurrent: 3,
        timeoutSeconds: 120,
      };
      
      await invoke('start_code_generation', {
        plan: architecturePlan,
        config,
      });
    } catch (err) {
      setError(String(err));
    }
  };

  const cancelGeneration = async () => {
    try {
      await invoke('cancel_generation');
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <div className="engineer-panel">
      <div className="engineer-header">
        <h2>Engineer Agent</h2>
        <div className="engineer-state">
          State: <span className={`state-${state}`}>{state}</span>
        </div>
      </div>

      {state === 'idle' && (
        <button onClick={startGeneration} className="btn-primary">
          Start Code Generation
        </button>
      )}

      {state !== 'idle' && state !== 'complete' && state !== 'error' && (
        <div className="generation-progress">
          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${progress}%` }} />
          </div>
          <div className="progress-text">{progress}%</div>
          {currentFile && <div className="current-file">Generating: {currentFile}</div>}
          <button onClick={cancelGeneration} className="btn-secondary">
            Cancel
          </button>
        </div>
      )}

      {reasoning.length > 0 && (
        <div className="reasoning-stream">
          <h3>Reasoning</h3>
          <div className="reasoning-list">
            {reasoning.slice(-10).map((entry, idx) => (
              <div key={idx} className="reasoning-entry">
                {entry}
              </div>
            ))}
          </div>
        </div>
      )}

      {error && (
        <div className="error-message">
          <strong>Error:</strong> {error}
        </div>
      )}

      {state === 'complete' && (
        <div className="completion-message">
          <h3>✓ Code Generation Complete</h3>
          <button onClick={() => invoke('export_generated_code', { directory: './export' })}>
            Export Code
          </button>
        </div>
      )}
    </div>
  );
};
