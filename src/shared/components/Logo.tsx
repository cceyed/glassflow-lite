import React from "react";
import { motion } from "framer-motion";

export const Logo: React.FC = () => {
  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.9 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.5 }}
      className="flex items-center space-x-3"
    >
      {/* Glass icon with blue glow */}
      <motion.div
        className="relative"
        animate={{
          filter: [
            "drop-shadow(0 0 10px rgba(0, 102, 255, 0.5))",
            "drop-shadow(0 0 20px rgba(0, 102, 255, 0.8))",
            "drop-shadow(0 0 10px rgba(0, 102, 255, 0.5))",
          ],
        }}
        transition={{
          duration: 3,
          repeat: Infinity,
          ease: "easeInOut",
        }}
      >
        <svg
          width="40"
          height="40"
          viewBox="0 0 40 40"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          {/* Glass prism shape */}
          <path
            d="M20 2L38 14V26L20 38L2 26V14L20 2Z"
            fill="url(#glass-gradient)"
            stroke="#0066ff"
            strokeWidth="1.5"
            strokeLinejoin="round"
          />
          <path
            d="M20 2V38M2 14L38 26M38 14L2 26"
            stroke="#0066ff"
            strokeWidth="1"
            opacity="0.5"
          />
          <defs>
            <linearGradient
              id="glass-gradient"
              x1="2"
              y1="2"
              x2="38"
              y2="38"
              gradientUnits="userSpaceOnUse"
            >
              <stop offset="0%" stopColor="rgba(0, 102, 255, 0.2)" />
              <stop offset="50%" stopColor="rgba(0, 102, 255, 0.1)" />
              <stop offset="100%" stopColor="rgba(0, 102, 255, 0.05)" />
            </linearGradient>
          </defs>
        </svg>
      </motion.div>

      {/* Logo text */}
      <div className="flex flex-col">
        <motion.h1
          className="text-2xl font-bold tracking-tight"
          style={{
            background: "linear-gradient(135deg, #ffffff 0%, #0066ff 100%)",
            WebkitBackgroundClip: "text",
            WebkitTextFillColor: "transparent",
            backgroundClip: "text",
          }}
        >
          Glassflow
        </motion.h1>
        <motion.p
          className="text-xs text-gray-400 tracking-wider"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.3 }}
        >
          AI ORCHESTRATION
        </motion.p>
      </div>
    </motion.div>
  );
};
