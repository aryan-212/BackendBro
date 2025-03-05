mod ai_functions;
mod apis;
mod helpers;
mod models;
use crate::helpers::command_line::get_user_response;
fn main() {
    let user_req = get_user_response("What WebServer are we building today !?");
    dbg!(user_req);
}
