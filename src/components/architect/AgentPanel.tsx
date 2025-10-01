import React, { useState } from 'react';
import { GlassSurface } from '../design-system/GlassSurface';
import { AgentCard } from './AgentCard';
import { ReasoningStream } from './ReasoningStream';

type AgentState = 'idle' | 'analyzing' | 'questioning' | 'designing' | 'complete' | 'error';

interface ReasoningEntry {
  timestamp: string;
  type: string;
  content: string;
}

interface AgentPanelProps {
  state: AgentState;
  confidence: number;
  reasoningEntries: ReasoningEntry[];
  progress?: number;
  elapsed?: number;
}

export const AgentPanel: React.FC<AgentPanelProps> = ({
  state,
  confidence,
  reasoningEntries,
  progress,
  elapsed,
}) => {
  const [expanded, setExpanded] = useState(false);
  
  return (
    <div className="space-y-4">
      <AgentCard
        state={state}
        confidence={confidence}
        onClick={() => setExpanded(!expanded)}
      />
      
      {expanded && (
        <GlassSurface className="p-6 space-y-4 animate-fade-in">
          {/* Progress Bar */}
          {state === 'designing' && progress !== undefined && (
            <div className="space-y-2">
              <div className="flex justify-between text-sm text-gray-400">
                <span>Designing Architecture</span>
                <span>{Math.round(progress * 100)}%</span>
              </div>
              <div className="h-2 bg-gray-800 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-blue-500 to-blue-400 transition-all duration-300"
                  style={{ width: `${progress * 100}%` }}
                />
              </div>
            </div>
          )}
          
          {/* Elapsed Time */}
          {elapsed !== undefined && elapsed > 0 && (
            <div className="text-sm text-gray-400">
              Elapsed: {elapsed}s
            </div>
          )}
          
          {/* Reasoning Stream */}
          <div>
            <h4 className="text-sm font-semibold text-white mb-2">Reasoning:</h4>
            <ReasoningStream entries={reasoningEntries} />
          </div>
        </GlassSurface>
      )}
    </div>
  );
};
