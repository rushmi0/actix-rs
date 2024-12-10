mod index;
mod hello;

use actix_web::web;

pub fn service_hub(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index);
    cfg.service(hello::hello);
}
