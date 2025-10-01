// Message Bus - Simple event system for inter-agent communication
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Message {
    pub from: AgentType,
    pub to: AgentType,
    pub payload: MessagePayload,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentType {
    Orchestrator,
    Architect,
    Engineer,
    Quality,
    Debug,
}

#[derive(Debug, Clone)]
pub enum MessagePayload {
    StateChanged(String),
    OutputReady(String),
    ErrorOccurred(String),
    ConfidenceUpdated(f32),
    RequestDecision(String),
    ProgressUpdate(f32),
}

pub struct MessageBus {
    messages: Vec<Message>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    
    pub fn send(&mut self, message: Message) {
        self.messages.push(message);
    }
    
    pub fn broadcast(&mut self, from: AgentType, payload: MessagePayload) {
        let message = Message {
            from,
            to: AgentType::Orchestrator,
            payload,
            timestamp: Instant::now(),
        };
        self.messages.push(message);
    }
    
    pub fn get_messages_for(&self, agent: AgentType) -> Vec<&Message> {
        self.messages.iter()
            .filter(|m| m.to == agent)
            .collect()
    }
    
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}
