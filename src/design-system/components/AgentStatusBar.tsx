import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { BaseComponentProps, AgentStatus } from '@/types/design-system';

interface AgentStatusBarProps extends BaseComponentProps {
  agents: AgentStatus[];
  onAgentClick?: (agentId: string) => void;
}

export const AgentStatusBar: React.FC<AgentStatusBarProps> = ({
  agents,
  onAgentClick,
  className = '',
  testId = 'agent-status-bar',
}) => {
  const [hoveredAgent, setHoveredAgent] = useState<string | null>(null);

  const getStatusColor = (status: AgentStatus['status']) => {
    switch (status) {
      case 'active':
        return 'text-glass-blue-400';
      case 'complete':
        return 'text-white';
      default:
        return 'text-text-tertiary';
    }
  };

  return (
    <div
      data-testid={testId}
      className={`bg-surface border-t border-border flex items-center divide-x divide-border ${className}`}
      style={{ height: '48px' }}
    >
      {agents.map((agent) => (
        <div
          key={agent.id}
          className="flex-1 px-4 flex items-center justify-between cursor-pointer hover:bg-white/5 transition-colors"
          onClick={() => onAgentClick?.(agent.id)}
          onMouseEnter={() => setHoveredAgent(agent.id)}
          onMouseLeave={() => setHoveredAgent(null)}
        >
          <div className="flex items-center space-x-2">
            {/* Status LED */}
            <div
              data-testid="status-led"
              className={`w-2 h-2 rounded-full ${
                agent.status === 'active'
                  ? 'bg-glass-blue-500 animate-pulse-slow'
                  : agent.status === 'complete'
                  ? 'bg-white'
                  : 'bg-text-disabled'
              }`}
            />
            <span className="text-sm font-medium text-text-primary">{agent.name}</span>
          </div>

          <div className="flex items-center space-x-2">
            <span className={`text-xs ${getStatusColor(agent.status)}`}>{agent.status}</span>
            {/* Progress */}
            <div
              role="progressbar"
              aria-valuenow={agent.progress}
              className="w-12 h-1 bg-border rounded-full overflow-hidden"
            >
              <div
                className="h-full bg-glass-blue-500 transition-all"
                style={{ width: `${agent.progress}%` }}
              />
            </div>
          </div>

          {/* Reasoning Panel (on hover) */}
          {hoveredAgent === agent.id && agent.reasoning && agent.reasoning.length > 0 && (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              className="absolute bottom-full mb-2 left-0 right-0 glass-effect p-4 rounded-lg"
            >
              <div className="space-y-1">
                {agent.reasoning.map((step, index) => (
                  <div key={index} className="text-sm text-text-secondary">
                    <span className="text-glass-blue-500">▸</span> {step}
                  </div>
                ))}
              </div>
            </motion.div>
          )}
        </div>
      ))}
    </div>
  );
};
