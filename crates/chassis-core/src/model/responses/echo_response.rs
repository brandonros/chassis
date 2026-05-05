use serde::Serialize;

#[derive(Serialize)]
pub struct EchoResponse {
    pub echoed: String,
    pub length: usize,
}
