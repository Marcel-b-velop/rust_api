use actix_web::{get, web, HttpResponse};
use crate::state::KeyValueStore;
use std::sync::Mutex; // Mutex importieren
use serde_json::json;


#[utoipa::path(
    get,
    path = "/values",
    responses(
        (status = 200, description = "Erfolgreiche Rückgabe der Key-Value-Daten", body = HashMap<String, String>)
    )
)]
#[get("/values")]
pub async fn get_values(data: web::Data<Mutex<KeyValueStore>>) -> HttpResponse {
    // Zugriff auf die Daten sichern
    let store = data.lock().unwrap();

    // Erstelle direkt eine JSON-Struktur aus der HashMap
    let response = json!(store.inner_store);

    // Gibt die JSON-Antwort zurück
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string())
}