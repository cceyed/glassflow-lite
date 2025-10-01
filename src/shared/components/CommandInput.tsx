import React, { useState } from "react";
import { motion } from "framer-motion";
import { GlassSurface } from "./GlassSurface";

interface CommandInputProps {
  onSubmit: (command: string) => void;
  placeholder?: string;
}

export const CommandInput: React.FC<CommandInputProps> = ({
  onSubmit,
  placeholder = "Enter command...",
}) => {
  const [command, setCommand] = useState("");
  const [isFocused, setIsFocused] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (command.trim()) {
      onSubmit(command.trim());
      setCommand("");
    }
  };

  return (
    <GlassSurface
      className="p-1"
      glow={isFocused}
      glowColor="blue"
      shimmer={isFocused}
    >
      <form onSubmit={handleSubmit} className="flex items-center">
        <motion.div
          className="flex-1 flex items-center"
          animate={{ scale: isFocused ? 1.01 : 1 }}
          transition={{ duration: 0.2 }}
        >
          <span className="text-glass-blue-500 font-mono text-lg px-4">›</span>
          <input
            type="text"
            value={command}
            onChange={(e) => setCommand(e.target.value)}
            onFocus={() => setIsFocused(true)}
            onBlur={() => setIsFocused(false)}
            placeholder={placeholder}
            className="flex-1 bg-transparent border-none outline-none text-white placeholder-gray-500 font-mono text-base py-3 pr-4"
            autoFocus
          />
        </motion.div>
      </form>
    </GlassSurface>
  );
};
