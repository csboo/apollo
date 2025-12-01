use dioxus::prelude::*;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
