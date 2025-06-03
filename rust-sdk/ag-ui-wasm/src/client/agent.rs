use crate::core::types::{Message, RunAgentInput, State};
use crate::stream::EventStream;
use crate::error::Result;

/// Trait for AG-UI agents in WASM environments
pub trait Agent {
    /// Run the agent with the given input
    fn run_agent(&self, input: RunAgentInput) -> Result<EventStream>;
    
    /// Get the current messages
    fn messages(&self) -> Vec<Message>;
    
    /// Set messages
    fn set_messages(&mut self, messages: Vec<Message>);
    
    /// Get the current state
    fn state(&self) -> State;
    
    /// Set state
    fn set_state(&mut self, state: State);
    
    /// Get the agent ID
    fn agent_id(&self) -> Option<String>;
    
    /// Get the thread ID
    fn thread_id(&self) -> Option<String>;
} 