use actix_web::{get, web, HttpResponse};
use std::sync::Mutex;
use crate::models::ListItem;
// Mutex importieren


#[utoipa::path(
    get,
    path = "/list",
    responses(
        (status = 200, description = "Erfolgreiche Rückgabe der Stammdaten", body = [ListItem])
    )
)]
#[get("/list")]
pub async fn get_items(data: web::Data<Mutex<Vec<ListItem>>>) -> HttpResponse {
    // Zugriff auf die Daten sichern
    let store = data.lock().unwrap();

    // Gibt die Stammdaten direkt als JSON aus
    HttpResponse::Ok().json(&*store)

}