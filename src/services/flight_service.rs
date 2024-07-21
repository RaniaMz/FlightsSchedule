/// Sorts a list of flights and returns the starting and final airport.
///
/// This function processes a list of flights represented as pairs of departure and arrival airports.
/// It determines the starting airport (the one that does not appear as a destination) and follows
/// the sequence of flights to find the final airport (the one that does not appear as a departure).
///
/// # Arguments
///
/// * `flights` - A reference to a vector of flight pairs, where each pair is a vector containing
///   the departure and arrival airports. Each airport is represented as a `String`.
///
/// # Returns
///
/// This function returns a `Result` where:
/// - `Ok(Vec<String>)` contains a vector with two elements:
///   1. The starting airport as a `String`.
///   2. The final airport as a `String`.
/// - `Err(&'static str)` if the input is invalid (e.g., no flights provided, or flights are not properly formatted).
///
/// # Errors
///
/// This function returns an error if:
/// - The input `flights` vector is empty.
/// - Any flight pair does not contain exactly two elements.
/// - No starting airport can be determined.
///
/// # Example
///
/// ```
/// let flights = vec![
///     vec!["IND".to_string(), "EWR".to_string()],
///     vec!["SFO".to_string(), "ATL".to_string()],
///     vec!["GSO".to_string(), "IND".to_string()],
///     vec!["ATL".to_string(), "GSO".to_string()],
/// ];
///
/// let result = sort_flights(&flights);
/// assert_eq!(result, Ok(vec!["SFO".to_string(), "EWR".to_string()]));
/// ```
use std::collections::HashMap;

pub fn sort_flights(flights: &Vec<Vec<String>>) -> Result<Vec<String>, &'static str> {
    if flights.is_empty() {
        return Err("No flights provided");
    }

    let mut flight_map = HashMap::new();
    let mut reverse_map = HashMap::new();

    for flight in flights {
        if flight.len() != 2 {
            return Err("Invalid flight format");
        }
        flight_map.insert(flight[0].clone(), flight[1].clone());
        reverse_map.insert(flight[1].clone(), flight[0].clone());
    }

    // Collect the keys into a vector
    let keys: Vec<String> = flight_map.keys().cloned().collect();

    // Find the starting airport
    let start = keys.iter()
        .find(|&airport| !reverse_map.contains_key(airport))
        .ok_or("No starting airport found")?;

    let mut current = start.clone();
    let mut final_airport = current.clone();

    // Traverse the flight path to find the final airport
    while let Some(next) = flight_map.remove(&current) {
        final_airport = next.clone();
        current = next;
    }

    Ok(vec![start.clone(), final_airport])
}
