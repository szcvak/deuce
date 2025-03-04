use std::error::Error;
use csv::ReaderBuilder;

pub struct Locations;

impl Locations {
    /// Returns a vector of location IDs (0-indexed, computed as line index minus 2)
    /// for rows in the CSV where the first column is not "Tutorial".
    pub fn get_locations() -> Result<Vec<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path("assets/csv_logic/locations.csv")?;
        let mut locations_id = Vec::new();
        // Enumerate gives the line count starting at 0.
        for (i, record) in rdr.records().enumerate() {
            let record = record?;
            // Skip the two header rows.
            if i < 2 {
                continue;
            }
            // If the first field is not "Tutorial", add (i - 2) to the vector.
            if record.get(0).unwrap_or("") != "Tutorial" {
                locations_id.push(i - 2);
            }
        }
        Ok(locations_id)
    }
}

