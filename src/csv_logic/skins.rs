use std::error::Error;
use csv::{ReaderBuilder, StringRecord};

const SKINS_PATH: &'static str = "assets/csv_logic/skins.csv";
const CHARACTERS_PATH: &'static str = "assets/csv_logic/characters.csv";

pub struct Skins;

impl Skins {
    /// Reads the skins CSV and returns a vector of skin IDs (line index minus 2).
    pub fn get_skins() -> Result<Vec<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(SKINS_PATH)?;
        let mut skins_id = Vec::new();
        for (i, _record) in rdr.records().enumerate() {
            if i < 2 {
                continue;
            }
            skins_id.push(i - 2);
        }
        Ok(skins_id)
    }

    /// Given a skin ID, returns the corresponding brawler index.
    /// It first loads the skins CSV (skipping headers) to get the target brawler name from column 1.
    /// Then it loads the characters CSV and searches (skipping headers) for a row
    /// whose first column matches that target name.
    /// If no match is found (or if the skin_id is out-of-range), it returns 0.
    pub fn get_brawler(skin_id: usize) -> Result<usize, Box<dyn Error>> {
        // Load all skins (skipping first two rows).
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(SKINS_PATH)?;
        let skins: Vec<StringRecord> = rdr.records()
            .filter_map(Result::ok)
            .skip(2)
            .collect();
        if skin_id >= skins.len() {
            return Ok(0);
        }
        // Get the target brawler name from column 1.
        let target = skins[skin_id].get(1).unwrap_or("");
        
        // Load characters CSV.
        let mut rdr_chars = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CHARACTERS_PATH)?;
        let cards: Vec<StringRecord> = rdr_chars.records()
            .filter_map(Result::ok)
            .skip(2)
            .collect();
        // Search for a matching row (by comparing column 0).
        for (index, row) in cards.iter().enumerate() {
            if row.get(0).unwrap_or("") == target {
                return Ok(index);
            }
        }
        Ok(0)
    }

    /// Returns a vector of indices for skins that are non‑default.
    /// The logic is:
    ///   1. Read characters.csv (skipping headers) and collect the default skin values from column 20.
    ///   2. Read skins.csv (skipping headers) and for each row,
    ///      if the skin name (column 0) is not found among the default skins and column 1 is not empty,
    ///      then include its index.
    pub fn get_non_default_skins() -> Result<Vec<usize>, Box<dyn Error>> {
        // Get default skins from characters.csv.
        let mut rdr_chars = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CHARACTERS_PATH)?;
        let cards: Vec<StringRecord> = rdr_chars.records()
            .filter_map(Result::ok)
            .skip(2)
            .collect();
        let default_skins: Vec<String> = cards.iter()
            .map(|row| row.get(20).unwrap_or("").to_string())
            .collect();

        // Read skins.csv.
        let mut rdr_skins = ReaderBuilder::new()
            .has_headers(false)
            .from_path(SKINS_PATH)?;
        let skins: Vec<StringRecord> = rdr_skins.records()
            .filter_map(Result::ok)
            .skip(2)
            .collect();
        let mut non_default = Vec::new();
        for (index, row) in skins.iter().enumerate() {
            let name = row.get(0).unwrap_or("");
            let second = row.get(1).unwrap_or("");
            if !default_skins.contains(&name.to_string()) && !second.is_empty() {
                non_default.push(index);
            }
        }
        Ok(non_default)
    }

    /// Returns the cost (price) of a skin given its ID.
    /// If the skin_id is out of range, returns 0.
    pub fn get_skin_price(skin_id: usize) -> Result<i32, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(SKINS_PATH)?;
        let skins: Vec<StringRecord> = rdr.records()
            .filter_map(Result::ok)
            .skip(2)
            .collect();
        if skin_id >= skins.len() {
            return Ok(0);
        }
        // Parse the price from column 3; if parsing fails, default to 0.
        let price_str = skins[skin_id].get(3).unwrap_or("0");
        let price: i32 = price_str.parse().unwrap_or(0);
        Ok(price)
    }

    /// Checks if the skin (by its ID) is a default skin.
    /// It does this by:
    ///   1. Reading the default skin names from characters.csv (column 20).
    ///   2. Reading skins.csv (skipping headers) and comparing the skin's name (column 0)
    ///      against the list of default skin names.
    /// If the skin_id is out of range, returns false.
    pub fn get_is_default_skin(skin_id: usize) -> Result<bool, Box<dyn Error>> {
        let mut rdr_chars = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CHARACTERS_PATH)?;
        let cards: Vec<StringRecord> = rdr_chars.records()
            .filter_map(Result::ok)
            .skip(2)
            .collect();
        let default_skins: Vec<String> = cards.iter()
            .map(|row| row.get(20).unwrap_or("").to_string())
            .collect();
        let mut rdr_skins = ReaderBuilder::new()
            .has_headers(false)
            .from_path(SKINS_PATH)?;
        let skins: Vec<StringRecord> = rdr_skins.records()
            .filter_map(Result::ok)
            .skip(2)
            .collect();
        if skin_id >= skins.len() {
            return Ok(false);
        }
        let name = skins[skin_id].get(0).unwrap_or("");
        Ok(default_skins.contains(&name.to_string()))
    }
}

