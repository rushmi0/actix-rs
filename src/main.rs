mod app;
mod services;
mod module;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
