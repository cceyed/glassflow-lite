import React from 'react';
import { motion } from 'framer-motion';
import { BaseComponentProps } from '@/types/design-system';

interface CardProps extends BaseComponentProps {
  children: React.ReactNode;
  onClick?: () => void;
  hoverable?: boolean;
  active?: boolean;
}

export const Card: React.FC<CardProps> = ({
  children,
  onClick,
  hoverable = false,
  active = false,
  className = '',
  testId = 'card',
}) => {
  return (
    <motion.div
      data-testid={testId}
      onClick={onClick}
      className={`bg-surface border border-border rounded-lg p-6 ${
        active ? 'glass-blue-glow' : ''
      } ${hoverable || onClick ? 'cursor-pointer' : ''} ${className}`}
      whileHover={hoverable || onClick ? { borderColor: 'rgba(255, 255, 255, 0.3)' } : {}}
      transition={{ duration: 0.2 }}
    >
      {children}
    </motion.div>
  );
};
