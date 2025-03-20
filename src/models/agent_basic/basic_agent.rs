use crate::models::general::llm::Message;
use super::basic_traits::BasicTraits;

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished
}

impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        return Self {
            objective: objective,
            position: position,
            state: AgentState::Discovery,
            memory: Vec::from([])
        };
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        return &self.objective;
    }

    fn get_position(&self) -> &String {
        return &self.position;
    }

    fn get_state(&self) -> &AgentState {
        return &self.state;
    }

    fn get_memory(&self) -> &Vec<Message> {
        return &self.memory;
    }
}