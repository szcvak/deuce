use std::error::Error;
use csv::ReaderBuilder;

use crate::csv_logic::characters::Characters;

const CARDS_CSV: &str = "assets/csv_logic/cards.csv";

pub struct Cards;

impl Cards {
    /// Returns a vector of card IDs (0-indexed after headers).
    pub fn get_cards() -> Result<Vec<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CARDS_CSV)?;
        let mut cards = Vec::new();
        for (i, _record) in rdr.records().enumerate() {
            if i < 2 {
                continue;
            }
            cards.push(i - 2);
        }
        Ok(cards)
    }

    /// Returns the brawler rarity for the card with the given ID (taken from column 10).
    pub fn get_brawler_rarity(id: usize) -> Result<Option<String>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CARDS_CSV)?;
        let mut records = rdr.records().skip(2);
        if let Some(result) = records.nth(id) {
            let record = result?;
            if let Some(rarity) = record.get(10) {
                return Ok(Some(rarity.to_string()));
            }
        }
        Ok(None)
    }

    /// Implements the getUnlock logic:
    /// 1. If the card at the given index (card) has "unlock" in column 5, returns that index.
    /// 2. Otherwise, searches for the row with the same brawler (column 3) that has "unlock" in column 5.
    pub fn get_unlock(card: usize) -> Result<Option<usize>, Box<dyn Error>> {
        // First pass: read the card row.
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CARDS_CSV)?;
        let mut records = rdr.records().skip(2);
        let mut brawler_name: Option<String> = None;
        if let Some(result) = records.nth(card) {
            let record = result?;
            if let Some(name) = record.get(3) {
                brawler_name = Some(name.to_string());
            }
            if let Some(field) = record.get(5) {
                if field == "unlock" {
                    return Ok(Some(card));
                }
            }
        }
        // Second pass: search for a matching row with "unlock".
        if let Some(ref name) = brawler_name {
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_path(CARDS_CSV)?;
            let records = rdr.records().skip(2);
            for (i, result) in records.enumerate() {
                let record = result?;
                if let Some(row_name) = record.get(3) {
                    if row_name == name {
                        if let Some(field) = record.get(5) {
                            if field == "unlock" {
                                return Ok(Some(i));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Returns true if the card with the given ID has "unlock" in column 5.
    pub fn is_unlock(id: usize) -> Result<bool, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CARDS_CSV)?;
        let mut records = rdr.records().skip(2);
        if let Some(result) = records.nth(id) {
            let record = result?;
            if let Some(field) = record.get(5) {
                return Ok(field == "unlock");
            }
        }
        Ok(false)
    }

    /// Retrieves the brawler ID by reading the card row (column 3 gives the brawler name)
    /// and then calling Characters::get_character_by_name.
    pub fn get_brawler_id(card: usize) -> Result<Option<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CARDS_CSV)?;
        let mut records = rdr.records().skip(2);
        if let Some(result) = records.nth(card) {
            let record = result?;
            if let Some(brawler_name) = record.get(3) {
                // Characters::get_character_by_name returns an Option<usize>
                return Characters::get_character_by_name(brawler_name);
            }
        }
        Ok(None)
    }

    /// Returns a vector of card IDs for which the card row has "unlock" in column 5
    /// and the brawler (from column 3) is not disabled.
    pub fn get_brawlers() -> Result<Vec<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CARDS_CSV)?;
        let mut brawlers = Vec::new();
        for (i, result) in rdr.records().enumerate() {
            if i < 2 {
                continue;
            }
            let record = result?;
            if let Some(field) = record.get(5) {
                if field == "unlock" {
                    if let Some(name) = record.get(3) {
                        if !Characters::is_disabled(name)? {
                            brawlers.push(i - 2);
                        }
                    }
                }
            }
        }
        Ok(brawlers)
    }

    /// Returns a vector of card IDs for which the card row has "unlock" in column 5,
    /// the rarity (column 10) matches the given rarity,
    /// and the brawler (column 3) is not disabled.
    pub fn get_brawlers_with_rarity(rarity: &str) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(CARDS_CSV)?;
        let mut brawlers = Vec::new();
        for (i, result) in rdr.records().enumerate() {
            if i < 2 {
                continue;
            }
            let record = result?;
            if let (Some(field), Some(card_rarity), Some(name)) =
                (record.get(5), record.get(10), record.get(3))
            {
                if field == "unlock" && card_rarity == rarity {
                    if !Characters::is_disabled(name)? {
                        brawlers.push(i - 2);
                    }
                }
            }
        }
        Ok(brawlers)
    }
}

