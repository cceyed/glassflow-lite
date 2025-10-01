import React from 'react';
import { motion } from 'framer-motion';
import { BaseComponentProps, ButtonVariant } from '@/types/design-system';

interface ButtonProps extends BaseComponentProps {
  variant?: ButtonVariant;
  disabled?: boolean;
  onClick?: () => void;
  children: React.ReactNode;
  type?: 'button' | 'submit' | 'reset';
}

export const Button: React.FC<ButtonProps> = ({
  variant = 'primary',
  disabled = false,
  onClick,
  children,
  type = 'button',
  className = '',
  testId = 'button',
}) => {
  const variantClasses = {
    primary: 'bg-white text-black hover:bg-gray-100',
    secondary: 'border border-white text-white hover:bg-white/10',
    ghost: 'text-white hover:bg-white/5',
  };

  return (
    <motion.button
      data-testid={testId}
      type={type}
      onClick={onClick}
      disabled={disabled}
      className={`px-6 py-3 rounded font-semibold transition-all ${variantClasses[variant]} ${
        disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
      } ${className}`}
      whileHover={!disabled ? { scale: 1.02 } : {}}
      whileTap={!disabled ? { scale: 0.98 } : {}}
    >
      {children}
    </motion.button>
  );
};
