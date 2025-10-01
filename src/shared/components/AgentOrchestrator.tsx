import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { AgentCard } from "./AgentCard";

interface Agent {
  id: string;
  name: string;
  status: "idle" | "thinking" | "executing" | "complete";
  reasoning: string[];
}

interface AgentOrchestratorProps {
  command: string;
  onComplete?: () => void;
}

export const AgentOrchestrator: React.FC<AgentOrchestratorProps> = ({
  command,
  onComplete,
}) => {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [orchestrationPhase, setOrchestrationPhase] = useState<string>("");

  useEffect(() => {
    // Simulate agent orchestration
    simulateOrchestration();
  }, [command]);

  const simulateOrchestration = async () => {
    setOrchestrationPhase("Analyzing command...");
    await delay(1000);

    // Initialize agents
    const initialAgents: Agent[] = [
      {
        id: "planner",
        name: "Planner Agent",
        status: "thinking",
        reasoning: ["Breaking down command into subtasks", "Identifying dependencies"],
      },
      {
        id: "executor",
        name: "Executor Agent",
        status: "idle",
        reasoning: [],
      },
      {
        id: "validator",
        name: "Validator Agent",
        status: "idle",
        reasoning: [],
      },
    ];

    setAgents(initialAgents);
    await delay(2000);

    // Planner completes
    setOrchestrationPhase("Planning complete. Executing...");
    setAgents((prev) =>
      prev.map((agent) =>
        agent.id === "planner"
          ? { ...agent, status: "complete", reasoning: [...agent.reasoning, "Plan created successfully"] }
          : agent
      )
    );
    await delay(500);

    // Executor starts
    setAgents((prev) =>
      prev.map((agent) =>
        agent.id === "executor"
          ? {
              ...agent,
              status: "executing",
              reasoning: ["Executing planned tasks", "Processing command logic"],
            }
          : agent
      )
    );
    await delay(2500);

    // Executor completes
    setOrchestrationPhase("Execution complete. Validating...");
    setAgents((prev) =>
      prev.map((agent) =>
        agent.id === "executor"
          ? { ...agent, status: "complete", reasoning: [...agent.reasoning, "Execution successful"] }
          : agent
      )
    );
    await delay(500);

    // Validator starts
    setAgents((prev) =>
      prev.map((agent) =>
        agent.id === "validator"
          ? {
              ...agent,
              status: "thinking",
              reasoning: ["Validating results", "Checking for errors"],
            }
          : agent
      )
    );
    await delay(2000);

    // Validator completes
    setOrchestrationPhase("Task complete!");
    setAgents((prev) =>
      prev.map((agent) =>
        agent.id === "validator"
          ? { ...agent, status: "complete", reasoning: [...agent.reasoning, "Validation passed"] }
          : agent
      )
    );

    if (onComplete) {
      await delay(1000);
      onComplete();
    }
  };

  const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

  return (
    <div className="space-y-6">
      {orchestrationPhase && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-center"
        >
          <h2 className="text-2xl font-bold text-shimmer">{orchestrationPhase}</h2>
        </motion.div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <AnimatePresence>
          {agents.map((agent, index) => (
            <AgentCard
              key={agent.id}
              name={agent.name}
              status={agent.status}
              reasoning={agent.reasoning}
              delay={index * 0.1}
            />
          ))}
        </AnimatePresence>
      </div>
    </div>
  );
};
