use actix_web::{post, web, HttpResponse, Responder};
use crate::models::flight::{FlightRequest, FlightPath};
use crate::services::flight_service::sort_flights;

#[post("/calculate")]
async fn calculate_flight_path(flight_request: web::Json<FlightRequest>) -> impl Responder {
    let sorted_path = sort_flights(&flight_request.flights);

    match sorted_path {
        Ok(path) => HttpResponse::Ok().json(FlightPath { path }),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
