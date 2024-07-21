use actix_web::web;
use crate::handlers::flight_handler::calculate_flight_path;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(calculate_flight_path);
}
