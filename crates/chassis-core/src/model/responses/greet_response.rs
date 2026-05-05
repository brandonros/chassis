use serde::Serialize;

#[derive(Serialize)]
pub struct GreetResponse {
    pub message: String,
    pub name: String,
}
