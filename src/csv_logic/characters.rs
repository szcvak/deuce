use std::error::Error;
use csv::ReaderBuilder;

const CHARACTERS_CSV: &str = "assets/csv_logic/characters.csv";

pub struct Characters;

impl Characters {
    /// Checks whether the given brawler is disabled.
    /// Searches the CSV for a row where column 0 matches `brawler`
    /// and returns true if column 1 equals "true" (ignoring case/whitespace).
    pub fn is_disabled(brawler: &str) -> Result<bool, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CHARACTERS_CSV)?;
        for (i, result) in rdr.records().enumerate() {
            // Skip the first two header rows.
            if i < 2 {
                continue;
            }
            let record = result?;
            if let Some(name) = record.get(0) {
                if name == brawler {
                    // Treat missing or empty disabled field as not disabled.
                    if let Some(disabled) = record.get(1) {
                        return Ok(disabled.trim().to_lowercase() == "true");
                    }
                    return Ok(false);
                }
            }
        }
        // If not found, default to false.
        Ok(false)
    }

    /// Returns the 0-indexed row number (after headers) for the character with the given name.
    /// If no matching character is found, returns None.
    pub fn get_character_by_name(name: &str) -> Result<Option<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CHARACTERS_CSV)?;
        // Skip header rows.
        let records = rdr.records().skip(2);
        for (i, result) in records.enumerate() {
            let record = result?;
            if let Some(n) = record.get(0) {
                if n == name {
                    return Ok(Some(i));
                }
            }
        }
        Ok(None)
    }

    /// Returns a vector of character IDs (0-indexed after headers) for which:
    /// - The value in column 18 equals "Hero"
    /// - The value in column 1 (Disabled) is not "true" (ignoring case)
    pub fn get_brawlers() -> Result<Vec<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CHARACTERS_CSV)?;
        let mut brawlers = Vec::new();
        for (i, result) in rdr.records().enumerate() {
            if i < 2 {
                continue;
            }
            let record = result?;
            if let (Some(ctype), Some(disabled)) = (record.get(18), record.get(1)) {
                if ctype == "Hero" && disabled.trim().to_lowercase() != "true" {
                    // The row index in the CSV (after skipping 2 headers) is (i - 2).
                    brawlers.push(i - 2);
                }
            }
        }
        Ok(brawlers)
    }
}

