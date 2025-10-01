import React from 'react';

interface GlassSurfaceProps {
  children: React.ReactNode;
  className?: string;
  glow?: boolean;
  glowColor?: 'blue' | 'white';
  shimmer?: boolean;
  onClick?: () => void;
}

export const GlassSurface: React.FC<GlassSurfaceProps> = ({
  children,
  className = '',
  glow = false,
  glowColor = 'blue',
  shimmer = false,
  onClick,
}) => {
  const glowClass = glow
    ? glowColor === 'blue'
      ? 'glass-blue-glow'
      : 'white-glow'
    : '';
  
  const shimmerClass = shimmer ? 'border-shimmer' : '';
  
  return (
    <div
      className={`glass-effect ${glowClass} ${shimmerClass} ${className}`}
      onClick={onClick}
    >
      {children}
    </div>
  );
};
