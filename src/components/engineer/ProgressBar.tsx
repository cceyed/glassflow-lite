// T055: ProgressBar component
import React from 'react';

interface ProgressBarProps {
  completed: number;
  total: number;
  currentFile?: string;
}

export const ProgressBar: React.FC<ProgressBarProps> = ({ completed, total, currentFile }) => {
  const percentage = total > 0 ? (completed / total) * 100 : 0;

  return (
    <div className="progress-bar-container">
      <div className="progress-info">
        <span className="progress-label">
          {completed} / {total} files
        </span>
        <span className="progress-percentage">{percentage.toFixed(1)}%</span>
      </div>
      <div className="progress-track">
        <div
          className="progress-fill"
          style={{
            width: `${percentage}%`,
            transition: 'width 0.3s ease',
          }}
        />
      </div>
      {currentFile && (
        <div className="current-file-info">
          <span className="file-icon">📄</span>
          <span className="file-name">{currentFile}</span>
        </div>
      )}
    </div>
  );
};
