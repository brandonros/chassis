use serde::Deserialize;

#[derive(Deserialize)]
pub struct EchoRequest {
    pub message: String,
}
