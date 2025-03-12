pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) {
    let ai_function_str = ai_func(func_input);
    let msg=format!("FUNCTION : {}
    INSTRUCTION: You are a function printer.You ONLY print the results of functions.Nothing else.No commentary. Here is the input to the function : {}, Print out what the fucntion will return", ai_function_str,func_input);
    dbg!(msg);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::ai_func_managing::convert_user_input_to_goal;
    #[test]
    fn tests_extending_ai_function() {
        let x_str = convert_user_input_to_goal("dummy variable");
        dbg!(x_str);
    }
}
