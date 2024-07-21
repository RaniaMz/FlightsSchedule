use serde::{Deserialize, Serialize};

/// Represents a request containing a list of flights.
///
/// The `flights` field holds a vector of flight pairs, where each pair
/// consists of a departure and an arrival airport.
#[derive(Debug, Deserialize, Serialize)]
pub struct FlightRequest {
    pub flights: Vec<Vec<String>>,
}

/// Represents the response containing the sorted flight path.
///
/// The `path` field holds a vector of airports representing the ordered
/// flight path from the start to the end.
#[derive(Debug, Serialize)]
pub struct FlightPath {
    pub path: Vec<String>,
}
