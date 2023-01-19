use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientResponse {
    pub success: bool,
    pub status: i32,
    pub message: String,
    pub data: serde_json::Value,
}
