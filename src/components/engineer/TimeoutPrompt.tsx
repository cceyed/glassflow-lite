// T057: TimeoutPrompt component
import React from 'react';
import { invoke } from '@tauri-apps/api/core';

interface TimeoutPromptProps {
  filePath: string;
  elapsedSeconds: number;
  onChoice: (choice: 'continue' | 'cancel' | 'skip') => void;
}

export const TimeoutPrompt: React.FC<TimeoutPromptProps> = ({
  filePath,
  elapsedSeconds,
  onChoice,
}) => {
  const handleChoice = async (choice: 'continue' | 'cancel' | 'skip') => {
    try {
      await invoke('handle_timeout_prompt', {
        filePath,
        choice,
      });
      onChoice(choice);
    } catch (err) {
      console.error('Failed to handle timeout:', err);
    }
  };

  return (
    <div className="timeout-prompt-overlay">
      <div className="timeout-prompt-modal">
        <div className="timeout-icon">⏱️</div>
        <h3>Generation Timeout</h3>
        <p className="timeout-message">
          File generation is taking longer than expected
        </p>
        <div className="timeout-details">
          <div className="detail-item">
            <span className="detail-label">File:</span>
            <span className="detail-value">{filePath}</span>
          </div>
          <div className="detail-item">
            <span className="detail-label">Elapsed:</span>
            <span className="detail-value">{elapsedSeconds}s</span>
          </div>
        </div>
        <div className="timeout-actions">
          <button
            onClick={() => handleChoice('continue')}
            className="btn-primary"
          >
            Continue Waiting
          </button>
          <button
            onClick={() => handleChoice('skip')}
            className="btn-secondary"
          >
            Skip This File
          </button>
          <button
            onClick={() => handleChoice('cancel')}
            className="btn-danger"
          >
            Cancel Generation
          </button>
        </div>
      </div>
    </div>
  );
};
