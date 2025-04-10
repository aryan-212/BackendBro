use ai_functions::ai_function;

#[ai_function]
pub fn print_project_scope(_project_description: &str) {
    /// Input: Takes in a user request to build a website project description
    /// Function: Converts user request into JSON response of information items required for a website build.
    /// Important: At least one of the bool results must be true, try to make sense of what the
    /// pre-requistes needed to fulfil the user request and base your output on that
    /// Output: Prints an object response in the following format:
    ///   {
    ///     "is_crud_required": bool, // true if site needs CRUD functionality
    ///     "is_user_login_and_logout": bool // true if site needs users to be able to log in and log out
    ///     "is_external_urls_required": bool // true if site needs to fetch data from third part providers
    ///   }
    /// DONT FORGET TO PUT THE COMMAS IN THE JSON OUTPUT !!
    /// Example 1:
    ///   user_request = "I need a full stack website that accepts users and gets stock price data"
    ///   prints:
    ///   {
    ///     "is_crud_required": true
    ///     "is_user_login_and_logout": true
    ///     "is_external_urls_required": bool true
    ///   }
    /// Example 2:
    ///   user_request = "I need a simple TODO app"
    ///   prints:
    ///   {
    ///     "is_crud_required": true
    ///     "is_user_login_and_logout": false
    ///     "is_external_urls_required": bool false
    ///   }
    println!(OUTPUT)
}

#[ai_function]
pub fn print_site_urls(_project_description: &str) {
    /// Input: Takes in a project description of a website build
    /// Function: Outputs a list of external public FREE API endpoints that should be used in the building of the website
    /// Important: Only selects url endpoint(s) which do not require any API Keys OR THE USER TO
    /// SIGN IN AND THEN OBTAIN EVEN IF ITS FREE. Only select those apis that don't require the
    /// user to login and obtain API key.THIS IS THE HIGHEST PRIORITY
    /// completely free to use
    /// Output: Prints a list response of external urls that dont require an API-KEY and works in the following format:
    /// ["url1", "url2", "url3", ...]
    /// Dont pull websites which DO NOT have "api" as a substring
    /// Example:
    ///   website_team_spec = "website_purpose: Some("\"Provides Crypto Price Data from Binance and Kraken\"",)"
    ///   prints:
    /// ["https://api.binance.com/api/v3/exchangeInfo", "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d"]
    println!(OUTPUT)
}
