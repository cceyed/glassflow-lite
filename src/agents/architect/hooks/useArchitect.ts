import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useArchitectStore } from '../store/architectStore';

export const useArchitect = () => {
  const store = useArchitectStore();
  
  useEffect(() => {
    // Listen to state-changed events
    const unsubscribeState = listen('architect:state-changed', (event: any) => {
      store.setAgentState(event.payload.to);
    });
    
    // Listen to reasoning events
    const unsubscribeReasoning = listen('architect:reasoning', (event: any) => {
      store.addReasoningEntry(event.payload);
    });
    
    // Listen to question events
    const unsubscribeQuestion = listen('architect:question', (event: any) => {
      store.setQuestion(event.payload);
    });
    
    // Listen to progress events
    const unsubscribeProgress = listen('architect:progress', (event: any) => {
      store.setProgress(event.payload.progress);
      store.setElapsed(event.payload.elapsed);
    });
    
    // Listen to complete events
    const unsubscribeComplete = listen('architect:complete', (event: any) => {
      store.setPlan(event.payload.plan);
      store.setConfidence(event.payload.confidence.overall);
    });
    
    // Listen to error events
    const unsubscribeError = listen('architect:error', (event: any) => {
      console.error('Architect error:', event.payload.message);
    });
    
    // Cleanup
    return () => {
      Promise.all([
        unsubscribeState,
        unsubscribeReasoning,
        unsubscribeQuestion,
        unsubscribeProgress,
        unsubscribeComplete,
        unsubscribeError,
      ]).then((unsubs) => unsubs.forEach((unsub) => unsub()));
    };
  }, [store]);
  
  // IPC command wrappers
  const analyze = async (spec: string) => {
    try {
      await invoke('architect_analyze', { spec });
    } catch (error) {
      console.error('Failed to analyze:', error);
      throw error;
    }
  };
  
  const answer = async (answerText: string) => {
    try {
      await invoke('architect_answer', { answer: answerText });
    } catch (error) {
      console.error('Failed to answer:', error);
      throw error;
    }
  };
  
  const getState = async () => {
    try {
      return await invoke('architect_get_state');
    } catch (error) {
      console.error('Failed to get state:', error);
      throw error;
    }
  };
  
  const exportPlan = async (path: string) => {
    try {
      await invoke('architect_export_plan', { path });
    } catch (error) {
      console.error('Failed to export plan:', error);
      throw error;
    }
  };
  
  const cancel = async () => {
    try {
      await invoke('architect_cancel');
    } catch (error) {
      console.error('Failed to cancel:', error);
      throw error;
    }
  };
  
  const retry = async (spec: string) => {
    try {
      await invoke('architect_retry', { spec });
    } catch (error) {
      console.error('Failed to retry:', error);
      throw error;
    }
  };
  
  return {
    // State
    ...store,
    
    // Actions
    analyze,
    answer,
    getState,
    exportPlan,
    cancel,
    retry,
  };
};
