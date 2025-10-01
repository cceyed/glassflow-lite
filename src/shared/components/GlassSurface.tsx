import React from "react";
import { motion } from "framer-motion";

interface GlassSurfaceProps {
  children: React.ReactNode;
  className?: string;
  glow?: boolean;
  glowColor?: "white" | "blue";
  shimmer?: boolean;
  pulse?: boolean;
  onClick?: () => void;
}

export const GlassSurface: React.FC<GlassSurfaceProps> = ({
  children,
  className = "",
  glow = false,
  glowColor = "white",
  shimmer = false,
  pulse = false,
  onClick,
}) => {
  const glowClass = glow
    ? glowColor === "blue"
      ? "glass-blue-glow"
      : "shadow-[0_0_20px_rgba(255,255,255,0.3),0_0_40px_rgba(255,255,255,0.2)]"
    : "";

  const shimmerClass = shimmer ? "border-shimmer" : "";
  const pulseClass = pulse ? "animate-pulse-slow" : "";

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
      className={`glass-effect rounded-lg ${glowClass} ${shimmerClass} ${pulseClass} ${className}`}
      onClick={onClick}
    >
      {children}
    </motion.div>
  );
};
