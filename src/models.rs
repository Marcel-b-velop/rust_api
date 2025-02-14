use std::fs;
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

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ListItem {
     key: String,
     name: String,
     short_description: String,
     preview: String,
}

pub fn read_list_item(file_path: &str) -> Vec<ListItem> {
    // Dateiinhalt lesen
    let data = fs::read_to_string(file_path)
        .expect("Fehler beim Lesen der JSON-Datei");

    // JSON in die Zielstruktur deserialisieren
    serde_json::from_str(&data)
        .expect("Fehler beim Parsen der JSON-Datei")
}
