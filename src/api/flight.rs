use actix_web::web;
use crate::handlers::flight_handler::calculate_flight_path;

/// Initializes the routes for the flight API.
///
/// Registers the `calculate_flight_path` handler with the service configuration.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(calculate_flight_path);
}
