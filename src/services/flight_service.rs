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
