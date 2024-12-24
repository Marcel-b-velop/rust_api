use actix_web::{post, web, Responder, HttpResponse};
use crate::state::KeyValueStore;
use crate::models::{MyRequest, SubmitResponse};
use std::sync::Mutex; // Mutex importieren
use crate::lib::get_guid;

#[utoipa::path(
    post,
    path = "/submit",
    operation_id = "submit_key_value_pair",
    description = "Akzeptiert ein JSON-Objekt",
    request_body = MyRequest,
    responses(
        (status = 200, description = "Erfolgreich Daten übermittelt", body = SubmitResponse),
        (status = 400, description = "Ungültige Anfrage, z. B. fehlende Felder.")
    )
)]
#[post("/submit")]
pub(crate) async fn submit_json(
    body: web::Json<MyRequest>,
    data: web::Data<Mutex<KeyValueStore>>, // KeyValueStore in Mutex einwickeln
) -> impl Responder {
    let mut store = data.lock().unwrap(); // Zugriff sichern
    let key = get_guid();
    store.insert(&key, body.name.clone()); // Daten einfügen
    HttpResponse::Ok().json(SubmitResponse {
        status: "success".to_string(),
        key,
    })
} 