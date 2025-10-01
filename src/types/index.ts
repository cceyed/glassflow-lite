// Core types for Glassflow

export interface Agent {
  id: string;
  name: string;
  status: "idle" | "thinking" | "executing" | "complete";
  reasoning: string[];
}

export interface CommandHistory {
  id: string;
  command: string;
  timestamp: Date;
  status: "processing" | "complete";
}

export interface AgentTask {
  id: string;
  agent_type: string;
  description: string;
  status: string;
  reasoning: string[];
}

export interface AgentResponse {
  task_id: string;
  result: string;
  reasoning_steps: string[];
}
