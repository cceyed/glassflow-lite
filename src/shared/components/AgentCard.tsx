import React from "react";
import { motion } from "framer-motion";
import { GlassSurface } from "./GlassSurface";

interface AgentCardProps {
  name: string;
  status: "idle" | "thinking" | "executing" | "complete";
  reasoning?: string[];
  delay?: number;
}

export const AgentCard: React.FC<AgentCardProps> = ({
  name,
  status,
  reasoning = [],
  delay = 0,
}) => {
  const statusColors = {
    idle: "text-gray-400",
    thinking: "text-glass-blue-400",
    executing: "text-glass-blue-500",
    complete: "text-white",
  };

  const isActive = status === "thinking" || status === "executing";

  return (
    <motion.div
      initial={{ opacity: 0, x: -20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ duration: 0.5, delay }}
    >
      <GlassSurface
        className="p-4"
        glow={isActive}
        glowColor="blue"
        pulse={status === "thinking"}
      >
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-lg font-semibold text-white">{name}</h3>
          <motion.div
            className={`text-sm font-mono ${statusColors[status]}`}
            animate={
              isActive
                ? {
                    opacity: [0.5, 1, 0.5],
                  }
                : {}
            }
            transition={{
              duration: 2,
              repeat: Infinity,
              ease: "easeInOut",
            }}
          >
            {status}
          </motion.div>
        </div>

        {reasoning.length > 0 && (
          <div className="space-y-2">
            {reasoning.map((step, index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ duration: 0.3, delay: index * 0.1 }}
                className="flex items-start space-x-2"
              >
                <span className="text-glass-blue-500 text-xs mt-1">▸</span>
                <span className="text-sm text-gray-300 flex-1">{step}</span>
              </motion.div>
            ))}
          </div>
        )}
      </GlassSurface>
    </motion.div>
  );
};
