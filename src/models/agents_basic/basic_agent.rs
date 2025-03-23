use super::basic_trait;
use crate::models::agents_basic::basic_trait::BasicTraits;
#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}
#[derive(Debug, PartialEq)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<String>,
}
impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::from([]),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }
    fn get_objective(&self) -> &String {
        &self.objective
    }
    fn get_position(&self) -> &String {
        &self.position
    }
    fn get_memory(&self) -> &Vec<String> {
        &self.memory
    }
    fn get_state(&self) -> &AgentState {
        &self.state
    }
}
