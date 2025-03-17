use super::command_line::PrintCommand;
use crate::{
    ai_functions::{
        ai_func_architect::print_project_scope, ai_func_managing::convert_user_input_to_goal,
    },
    models::general::llm::{self, send_request},
};
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> String {
    let ai_function_str = ai_func(func_input);
    format!(
        "FUNCTION: {}\nINSTRUCTION: You are a function printer. \
        You ONLY print the results of functions. Nothing else. No commentary. \
        Here is the input to the function: {}. Print out what the function will return.",
        ai_function_str, func_input
    ) // Return String instead of &'static str
}
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&str) -> &'static str,
) -> String {
    let extended_msg = extend_ai_function(function_pass, &msg_context);
    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);
    // Get agent response
    let llm_response = match send_request(&extended_msg).await {
        Ok(response) => response,
        Err(e) => send_request(&msg_context)
            .await
            .expect("Failed to call Gemini"),
    };
    llm_response
}
pub async fn ai_task_request_decoded(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&str) -> &'static str,
) -> String {
    let extended_msg = extend_ai_function(function_pass, &msg_context);
    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);
    // Get agent response
    let llm_response = match send_request(&extended_msg).await {
        Ok(response) => response,
        Err(_) => send_request(&msg_context)
            .await
            .expect("Failed to call Gemini"),
    };
    let decoded_response =
        serde_json::from_str(llm_response.as_str()).expect("Failed to decode AI response");
    decoded_response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::ai_func_architect::print_project_scope;
    use crate::ai_functions::ai_func_managing::convert_user_input_to_goal;
    #[test]
    fn tests_extending_ai_function() {
        // let x_str = convert_user_input_to_goal("dummy variable");
        // dbg!(x_str);
        let mssg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        println!("{mssg}");
    }
}
#[tokio::test]
async fn tests_ai_task_request() {
    let ai_func_param = "display btc prices".to_string();
    let res = ai_task_request(
        ai_func_param,
        "Managing Agent",
        "Defining user requirements",
        convert_user_input_to_goal,
    )
    .await;
    let res2 = ai_task_request_decoded(
        res.clone(),
        "Solutions Architect",
        "Finding Project Scope",
        print_project_scope,
    )
    .await;
    dbg!(res2);
}
#[test]
fn tests_convert_user_input_to_goal() {
    let user_input = "Build me a web site for making stock price api requests";
    let result = convert_user_input_to_goal(user_input);
    println!("{}", result);
}
