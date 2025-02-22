use crate::models::ListItem;
use actix_web::{App, web, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_cors::Cors;


mod state;
mod routes;
mod models;
mod utils;

use crate::models::{MyRequest, SubmitResponse, MyResponse, read_list_item};

use state::KeyValueStore;
use routes::{submit, values};
use std::sync::Mutex;
// Define an OpenAPI structure for your routes
#[derive(OpenApi)] // Wichtig: OpenAPI-Definition der Endpunkte
#[openapi(
    paths(values::get_values, submit::submit_json, list::get_items), // Routen definieren
    components(schemas(MyRequest, MyResponse, SubmitResponse, ListItem)),
    tags(
        (name = "example", description = "Beispiel-API mit OpenAPI")
    )
)]
struct ApiDoc;

use log::LevelFilter;
use crate::routes::list;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
        // Initialize logger
        env_logger::Builder::new()
            .filter_level(LevelFilter::Debug)
            .init();
    let current_dir = env::current_dir().unwrap();
    println!("Aktuelles Arbeitsverzeichnis: {:?}", current_dir);

    let stammdaten = read_list_item("/Users/marcel/RustroverProjects/untitled/src/resources/stammdaten.json");
    let shared_data = web::Data::new(Mutex::new(stammdaten));

    // Zentraler Key-Value-Store
    let kv_store = web::Data::new(Mutex::new(KeyValueStore::new())); // Wrapping with web::Data

    
    HttpServer::new(move || {
        // CORS konfigurieren, um alle localhost-Ports zu erlauben
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // Prüfe, ob die Origin mit "http://localhost:" beginnt
                origin.as_bytes().starts_with(b"http://localhost:")
            })
            .allowed_methods(vec!["GET", "POST"]) // HTTP-Methoden definieren
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .allow_any_header()
            .max_age(3600); // Cache CORS-Regeln für 1 Stunde


        App::new()
            .wrap(Logger::default()) // Logging-Middleware
            .wrap(cors) // CORS hinzufügen
            .app_data(kv_store.clone()) // Shared AppState
            .app_data(shared_data.clone())
            .service(submit::submit_json) // POST /submit
            .service(values::get_values) // GET /values
            .service(list::get_items)
            .service(    SwaggerUi::new("/swagger-ui/{_:.*}") // Neuer Swagger-UI-Endpunkt
                .url("/api-docs/openapi.json", ApiDoc::openapi()))
    })
        .bind("localhost:8080")?
        .run()
        .await
}