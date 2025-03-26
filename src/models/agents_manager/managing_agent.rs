use crate::models::agents::agents_traits::{FactSheet, SpecialFunctions};
use crate::models::agents_basic::basic_agent::{AgentState, BasicAgent};
#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}
