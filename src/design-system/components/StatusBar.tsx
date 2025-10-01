import React from 'react';
import { BaseComponentProps } from '@/types/design-system';

interface StatusBarProps extends BaseComponentProps {
  projectPath: string;
  currentPhase: string;
  timeElapsed: string;
}

export const StatusBar: React.FC<StatusBarProps> = ({
  projectPath,
  currentPhase,
  timeElapsed,
  className = '',
  testId = 'status-bar',
}) => {
  return (
    <div
      data-testid={testId}
      className={`bg-background border-t border-border flex items-center justify-between px-4 ${className}`}
      style={{ height: '32px' }}
    >
      {/* Project Path - Left */}
      <div data-testid="status-section" className="flex-1 text-left">
        <span className="text-xs text-text-tertiary">{projectPath}</span>
      </div>

      {/* Current Phase - Center */}
      <div data-testid="status-section" className="flex-1 text-center">
        <span className="text-xs text-text-secondary font-medium">{currentPhase}</span>
      </div>

      {/* Time Elapsed - Right */}
      <div data-testid="status-section" className="flex-1 text-right">
        <span className="text-xs text-text-tertiary font-mono">{timeElapsed}</span>
      </div>
    </div>
  );
};
