import React from 'react';
import { BaseComponentProps } from '@/types/design-system';

interface TitleBarProps extends BaseComponentProps {
  projectName: string;
  onMinimize?: () => void;
  onMaximize?: () => void;
  onClose?: () => void;
}

export const TitleBar: React.FC<TitleBarProps> = ({
  projectName,
  onMinimize,
  onMaximize,
  onClose,
  className = '',
  testId = 'titlebar',
}) => {
  return (
    <div
      data-testid={testId}
      className={`flex items-center justify-between bg-background border-b border-border px-4 ${className}`}
      style={{ height: '40px' }}
    >
      {/* Logo - Left */}
      <div data-testid="titlebar-logo" className="flex items-center space-x-2">
        <div className="w-6 h-6 glass-brand rounded flex items-center justify-center">
          <span className="text-glass-blue-500 text-xs font-bold">G</span>
        </div>
        <span className="text-sm font-semibold text-text-primary">Glassflow</span>
      </div>

      {/* Project Name - Center */}
      <div className="flex-1 text-center">
        <span className="text-sm text-text-secondary">{projectName}</span>
      </div>

      {/* Window Controls - Right */}
      <div data-testid="window-controls" className="flex items-center space-x-2">
        {onMinimize && (
          <button
            onClick={onMinimize}
            aria-label="Minimize"
            className="w-8 h-8 flex items-center justify-center hover:bg-surface rounded transition-colors"
          >
            <span className="text-text-secondary">−</span>
          </button>
        )}
        {onMaximize && (
          <button
            onClick={onMaximize}
            aria-label="Maximize"
            className="w-8 h-8 flex items-center justify-center hover:bg-surface rounded transition-colors"
          >
            <span className="text-text-secondary">□</span>
          </button>
        )}
        {onClose && (
          <button
            onClick={onClose}
            aria-label="Close"
            className="w-8 h-8 flex items-center justify-center hover:bg-red-500/20 rounded transition-colors"
          >
            <span className="text-text-secondary hover:text-red-400">×</span>
          </button>
        )}
      </div>
    </div>
  );
};
