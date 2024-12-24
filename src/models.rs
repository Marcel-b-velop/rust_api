use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Eingabemodell für POST /submit
#[derive(Serialize, Deserialize, ToSchema)]
pub struct MyRequest {
    pub name: String,
}

// Ausgabemodell für Response
#[derive(Serialize, Deserialize, ToSchema)]
pub struct MyResponse {
    pub message: String,
}
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SubmitResponse {
    pub status: String,
    pub key: String,               
}