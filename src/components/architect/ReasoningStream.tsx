import React, { useEffect, useRef } from 'react';

interface ReasoningEntry {
  timestamp: string;
  type: string;
  content: string;
}

interface ReasoningStreamProps {
  entries: ReasoningEntry[];
}

const typeColors = {
  observation: 'text-blue-400',
  analysis: 'text-purple-400',
  decision: 'text-green-400',
  question: 'text-yellow-400',
  conclusion: 'text-white',
};

const typeIcons = {
  observation: '👁',
  analysis: '🔍',
  decision: '✓',
  question: '?',
  conclusion: '→',
};

export const ReasoningStream: React.FC<ReasoningStreamProps> = ({ entries }) => {
  const bottomRef = useRef<HTMLDivElement>(null);
  
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [entries]);
  
  return (
    <div className="max-h-64 overflow-y-auto space-y-2 pr-2 custom-scrollbar">
      {entries.slice(-100).map((entry, index) => (
        <div key={index} className="flex gap-2 text-sm animate-fade-in">
          <span className="text-gray-500 text-xs mt-0.5">
            {new Date(entry.timestamp).toLocaleTimeString()}
          </span>
          <span className="text-lg">
            {typeIcons[entry.type as keyof typeof typeIcons] || '•'}
          </span>
          <span className={typeColors[entry.type as keyof typeof typeColors] || 'text-gray-400'}>
            {entry.content}
          </span>
        </div>
      ))}
      <div ref={bottomRef} />
    </div>
  );
};
