use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

mod config;
mod api;
mod handlers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = config::get_config();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(api::init_routes)
    })
        .bind(config.server_addr)?
        .run()
        .await
}
