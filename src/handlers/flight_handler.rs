/// Handles POST requests to calculate the flight path.
///
/// This handler function receives a JSON payload containing a list of flights,
/// processes it to determine the sorted flight path using the `sort_flights` function,
/// and returns the result as a JSON response.
///
/// # Arguments
///
/// * `flight_request` - A JSON payload containing a `FlightRequest` object, which includes
///   a list of flights represented as pairs of departure and arrival airports.
///
/// # Returns
///
/// This function returns an HTTP response:
/// - `HttpResponse::Ok()` with a JSON body containing the `FlightPath` if the flight path is successfully sorted.
/// - `HttpResponse::BadRequest()` with a body containing an error message if there is an issue processing the flights.
///
/// # Example
///
/// ```
/// // Example JSON request body:
/// {
///     "flights": [
///         ["IND", "EWR"],
///         ["SFO", "ATL"],
///         ["GSO", "IND"],
///         ["ATL", "GSO"]
///     ]
/// }
///
/// // Example successful response:
/// {
///     "path": ["SFO", "EWR"]
/// }
/// ```
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
