use actix_cors::Cors;
use actix_web::{App, HttpServer};
use env_logger::Env;

use crate::services::api::v1;

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(cors_config())
            .configure(v1::init_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn cors_config() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:8080")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
        .send_wildcard()
}
