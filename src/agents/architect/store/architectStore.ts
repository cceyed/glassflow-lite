import { create } from 'zustand';

type AgentState = 'idle' | 'analyzing' | 'questioning' | 'designing' | 'complete' | 'error';

interface ReasoningEntry {
  timestamp: string;
  type: string;
  content: string;
}

interface Question {
  id: string;
  text: string;
  question_type: 'SingleChoice' | 'MultipleChoice' | 'YesNo' | 'FreeText' | 'ConfirmationWithDefault';
  options?: Array<{
    value: string;
    label: string;
    description?: string;
  }>;
  recommended_answer?: string;
  reasoning?: string;
  impact: 'High' | 'Medium' | 'Low';
}

interface ArchitecturePlan {
  projectName: string;
  techStack: any;
  components: any[];
  decisions: any[];
}

interface ArchitectStore {
  // State
  agentState: AgentState;
  reasoningEntries: ReasoningEntry[];
  currentQuestion: Question | null;
  plan: ArchitecturePlan | null;
  confidence: number;
  progress: number;
  elapsed: number;
  
  // Actions
  setAgentState: (state: AgentState) => void;
  addReasoningEntry: (entry: ReasoningEntry) => void;
  setQuestion: (question: Question | null) => void;
  setPlan: (plan: ArchitecturePlan | null) => void;
  setConfidence: (confidence: number) => void;
  setProgress: (progress: number) => void;
  setElapsed: (elapsed: number) => void;
  reset: () => void;
}

export const useArchitectStore = create<ArchitectStore>((set) => ({
  // Initial state
  agentState: 'idle',
  reasoningEntries: [],
  currentQuestion: null,
  plan: null,
  confidence: 0,
  progress: 0,
  elapsed: 0,
  
  // Actions
  setAgentState: (agentState) => set({ agentState }),
  
  addReasoningEntry: (entry) =>
    set((state) => ({
      reasoningEntries: [...state.reasoningEntries, entry],
    })),
  
  setQuestion: (currentQuestion) => set({ currentQuestion }),
  
  setPlan: (plan) => set({ plan }),
  
  setConfidence: (confidence) => set({ confidence }),
  
  setProgress: (progress) => set({ progress }),
  
  setElapsed: (elapsed) => set({ elapsed }),
  
  reset: () =>
    set({
      agentState: 'idle',
      reasoningEntries: [],
      currentQuestion: null,
      plan: null,
      confidence: 0,
      progress: 0,
      elapsed: 0,
    }),
}));
