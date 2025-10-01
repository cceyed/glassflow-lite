import React from 'react';
import { GlassSurface } from '../design-system/GlassSurface';
import { LEDIndicator } from '../design-system/LEDIndicator';

type AgentState = 'idle' | 'analyzing' | 'questioning' | 'designing' | 'complete' | 'error';

interface AgentCardProps {
  state: AgentState;
  confidence: number;
  onClick?: () => void;
}

const stateLabels: Record<AgentState, string> = {
  idle: 'Ready',
  analyzing: 'Analyzing Specification',
  questioning: 'Asking Questions',
  designing: 'Designing Architecture',
  complete: 'Complete',
  error: 'Error',
};

export const AgentCard: React.FC<AgentCardProps> = ({ state, confidence, onClick }) => {
  const isActive = ['analyzing', 'questioning', 'designing'].includes(state);
  
  return (
    <GlassSurface
      glow={isActive}
      glowColor="blue"
      className="p-4 cursor-pointer transition-all duration-300 hover:scale-105"
      onClick={onClick}
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <LEDIndicator state={state} />
          <div>
            <h3 className="text-lg font-semibold text-white">Architect Agent</h3>
            <p className="text-sm text-gray-400">{stateLabels[state]}</p>
          </div>
        </div>
        
        {confidence > 0 && (
          <div className="text-right">
            <div className="text-2xl font-bold text-white">{confidence}%</div>
            <div className="text-xs text-gray-400">Confidence</div>
          </div>
        )}
      </div>
    </GlassSurface>
  );
};
