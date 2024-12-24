use actix_web::{App, web, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi};
mod state;
mod routes;
mod models;
pub mod lib;
use crate::models::{MyRequest, SubmitResponse, MyResponse};

use state::KeyValueStore;
use routes::{submit, values};
use std::sync::Mutex;
// Define an OpenAPI structure for your routes
#[derive(OpenApi)] // Wichtig: OpenAPI-Definition der Endpunkte
#[openapi(
    paths(values::get_values, submit::submit_json), // Routen definieren
    components(schemas(MyRequest, MyResponse, SubmitResponse)),
    tags(
        (name = "example", description = "Beispiel-API mit OpenAPI")
    )
)]
struct ApiDoc;

use log::LevelFilter;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
        // Initialize logger
        env_logger::Builder::new()
            .filter_level(LevelFilter::Debug)
            .init();

    // Zentraler Key-Value-Store
    // Zentraler Key-Value-Store
    let kv_store = web::Data::new(Mutex::new(KeyValueStore::new())); // Wrapping with web::Data

    HttpServer::new(move || {
        App::new()
            .app_data(kv_store.clone()) // Shared AppState
            .service(submit::submit_json) // POST /submit
            .service(values::get_values) // GET /values
            .service(    SwaggerUi::new("/swagger-ui/{_:.*}") // Neuer Swagger-UI-Endpunkt
                .url("/api-docs/openapi.json", ApiDoc::openapi()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}