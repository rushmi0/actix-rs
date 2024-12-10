use actix_web::{get, web, HttpResponse, Responder};
use crate::module::greet;

#[get("/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    let greeting = greet::greet(&name);
    HttpResponse::Ok().body(greeting)
}
