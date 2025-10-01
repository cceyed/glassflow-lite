import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { BaseComponentProps } from '@/types/design-system';

interface CommandInputProps extends BaseComponentProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: (value: string) => void;
  placeholder?: string;
  autoFocus?: boolean;
}

export const CommandInput: React.FC<CommandInputProps> = ({
  value,
  onChange,
  onSubmit,
  placeholder = 'Enter command...',
  autoFocus = false,
  className = '',
  testId = 'command-input',
}) => {
  const [isFocused, setIsFocused] = useState(false);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && value.trim()) {
      onSubmit(value.trim());
    }
  };

  return (
    <motion.div
      data-testid={testId}
      className={`glass-effect p-1 ${isFocused ? 'glass-blue-glow border-shimmer' : ''} ${className}`}
      style={{ height: '60px' }}
      animate={{ scale: isFocused ? 1.01 : 1 }}
      transition={{ duration: 0.2 }}
    >
      <div className="flex items-center h-full">
        <span className="text-glass-blue-500 font-mono text-lg px-4">›</span>
        <input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onKeyDown={handleKeyDown}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          placeholder={placeholder}
          autoFocus={autoFocus}
          className="flex-1 bg-transparent border-none outline-none text-white placeholder-text-tertiary font-mono text-large py-3 pr-4"
        />
      </div>
    </motion.div>
  );
};
