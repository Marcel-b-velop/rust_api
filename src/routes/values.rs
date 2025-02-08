use actix_web::{get, web, HttpResponse};
use crate::state::KeyValueStore;
use std::sync::Mutex; // Mutex importieren


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

    // Hier nehmen wir an, dass `inner_store` etwas wie eine `HashMap<String, String>` ist
    let response = &store.inner_store; // Bezieht sich auf die innere HashMap

    // Gibt eine JSON-Antwort direkt aus der HashMap zurück
    HttpResponse::Ok()
        .json(response) // Automatische Serialisierung in JSON
}