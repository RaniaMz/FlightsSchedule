use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FlightRequest {
    pub flights: Vec<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct FlightPath {
    pub path: Vec<String>,
}