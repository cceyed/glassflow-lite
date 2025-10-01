import React from 'react';

type AgentState = 'idle' | 'analyzing' | 'questioning' | 'designing' | 'complete' | 'error';

interface LEDIndicatorProps {
  state: AgentState;
  className?: string;
}

const stateConfig = {
  idle: { symbol: '○', color: 'text-gray-400', glow: false },
  analyzing: { symbol: '◐', color: 'text-blue-400', glow: true },
  questioning: { symbol: '◑', color: 'text-blue-400', glow: true },
  designing: { symbol: '●', color: 'text-blue-500', glow: true },
  complete: { symbol: '✓', color: 'text-white', glow: false },
  error: { symbol: '✗', color: 'text-red-400', glow: false },
};

export const LEDIndicator: React.FC<LEDIndicatorProps> = ({ state, className = '' }) => {
  const config = stateConfig[state];
  const glowClass = config.glow ? 'glass-blue-glow animate-pulse-slow' : '';
  
  return (
    <span className={`text-2xl ${config.color} ${glowClass} ${className}`}>
      {config.symbol}
    </span>
  );
};
