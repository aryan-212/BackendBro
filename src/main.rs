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
#[tokio::main]
async fn main() {
    let user_req = get_user_response("What WebServer are we building today !?");
    let res = ai_task_request(
        user_req,
        "Managing Agent",
        "Defining user requirements",
        convert_user_input_to_goal,
    )
    .await;
    let res2 = ai_task_request(
        res.clone(),
        "Solutions Architect",
        "Finding Project Scope",
        print_project_scope,
    )
    .await;
    println!("{:#?}", res2);
}
