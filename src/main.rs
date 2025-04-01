#[macro_export]
macro_rules! get_function_string {
    ($func:ident) => {{ stringify!($func) }};
}
#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;
use crate::ai_functions::ai_func_architect::print_project_scope;
use crate::ai_functions::ai_func_managing::convert_user_input_to_goal; // If it's from ai_func_architect
use crate::helpers::command_line::get_user_response;
use crate::helpers::general::*;
use crate::models::agents_manager::managing_agent::ManagingAgent;
#[tokio::main]
async fn main() {
    let user_req = get_user_response("What WebServer are we building today !?");

    let mut manage_agent: ManagingAgent = ManagingAgent::new(user_req)
        .await
        .expect("Error creating agent");

    manage_agent.execute_project().await;
}
