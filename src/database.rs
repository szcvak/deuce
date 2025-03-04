use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

use log::*;
use serde::{Deserialize, Serialize};
use crate::player::*;
use std::collections::HashMap;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;
pub type DbConn = PooledConnection<PostgresConnectionManager<NoTls>>;

pub struct Database {
    pool: DbPool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub low_id: u32,
    pub club_id: u32,
    pub club_role: u32,
    pub player_experience: u32,
    pub solo_wins: u32,
    pub duo_wins: u32,
    pub three_x_three_wins: u32,
    pub gems: u32,
    pub gold: u32,
    pub elixir: u32,
    pub chips: u32,
    pub coins_doubler: u32,
    pub coins_booster: u32,
    pub trophies: u32,
    pub highest_trophies: u32,
    pub profile_icon: u32,
    pub room_id: u32,
    pub last_connection_time: u32,
    pub player_status: u32,
    pub region: String,
    pub control_mode: u32,
    pub has_battle_hints: bool,
    pub unlocked_brawlers: HashMap<i32, BrawlerData>,
    pub coins_reward: i32,
    pub event_count: i32,
}

impl Database {
    pub fn new(db_url: &str) -> Self {
        let manager = PostgresConnectionManager::new(db_url.parse().unwrap(), NoTls);
        let pool = Pool::new(manager).expect("deuce: failed to create database pool");

        Self { pool }
    }

    pub fn create_default(&mut self) {
        let mut conn = self.get_conn();

        // players
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS players (
                token TEXT PRIMARY KEY,
                data TEXT
            );"#,
            &[]
        ).expect("deuce: failed to execute sql");

        // rooms
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS rooms (
                id INT PRIMARY KEY,
                data TEXT
            );"#,
            &[]
        ).expect("deuce: failed to execute sql");

        // clubs
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS clubs (
                id INT PRIMARY KEY,
                data TEXT
            );"#,
            &[]
        ).expect("deuce: failed to execute sql");

        // club chats
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS club_chats (
                id INT PRIMARY KEY,
                data TEXT
            );"#,
            &[]
        ).expect("deuce: failed to execute sql");

        // room chats
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS room_chats (
                id INT PRIMARY KEY,
                data TEXT
            );"#,
            &[]
        ).expect("deuce: failed to execute sql");

        // events
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS events (
                state INT PRIMARY KEY,
                data TEXT
            );"#,
            &[]
        ).expect("deuce: failed to execute sql");
    }

    pub fn load_player(&mut self, player: &Player) -> Result<PlayerInfo, Box<dyn std::error::Error>> {
        let mut conn = self.get_conn();

        let result = conn.query_one(
            "SELECT data FROM players WHERE token = $1",
            &[&player.token.clone().unwrap()]
        ).expect("deuce: failed to execute sql");

        let data = result.get::<&str, String>("data");
        let prepared: PlayerInfo = serde_json::from_str(&data).unwrap();

        Ok(prepared)
    }

    pub fn create_player(&mut self, player: &Player) -> Result<(), &'static str> {
        if player.token.is_none() {
            error!("deuce: cannot create account if player's token is None");
            return Err("cannot create account: token is None");
        }
        
        let mut conn = self.get_conn();

        let result = conn.query_one(
            "SELECT COUNT(*) as count FROM players WHERE token = $1;",
            &[&player.token.clone().unwrap()]
        ).expect("deuce: failed to execute sql");

        if result.get::<&str, i64>("count") > 0 {
            error!("deuce: player with token already exists, will not create: {}", player.token.clone().unwrap());
            return Err("Player already exists");
        }

        let info = PlayerInfo {
            name: player.name.clone(),
            low_id: player.low_id,
            club_id: 0,
            club_role: 0,
            player_experience: player.player_experience,
            solo_wins: player.solo_wins,
            duo_wins: player.duo_wins,
            three_x_three_wins: player.three_x_three_wins,
            gems: player.gems,
            gold: player.gold,
            elixir: player.elixir,
            chips: player.chips,
            coins_doubler: player.coins_doubler,
            coins_booster: player.coins_booster,
            trophies: player.trophies,
            highest_trophies: player.highest_trophies,
            profile_icon: 0,
            room_id: 0,
            last_connection_time: 0,
            player_status: 0,
            region: player.region.clone(),
            control_mode: player.control_mode,
            has_battle_hints: false,
            unlocked_brawlers: player.unlocked_brawlers.clone(),
            coins_reward: player.coins_reward,
            event_count: player.event_count,
        };
        
        let serialized = serde_json::to_string(&info).expect("deuce: failed to serialize player info");
        
        conn.execute(
            "INSERT INTO players (token, data) VALUES ($1, $2)",
            &[&player.token.clone().unwrap(), &serialized]
        ).expect("deuce: failed to execute insert player info");

        Ok(())
    }

    pub fn token_exists(&self, token: String) -> bool {
        let mut conn = self.get_conn();

        let result = conn.query_one(
            "SELECT COUNT(*) as count FROM players WHERE token = $1;",
            &[&token.clone()]
        ).expect("deuce: failed to execute sql");

        if result.get::<&str, i64>("count") > 0 {
            return true;
        }
        
        false
    }
    
    pub fn get_free_id(&self) -> i64 {
        let mut conn = self.get_conn();
        
        let result = conn.query_one(
            "SELECT COUNT(*) as count FROM players;",
            &[]
        ).expect("deuce: failed to execute sql");
        
        result.get::<&str, i64>("count") + 1
    }

    pub fn get_conn(&self) -> DbConn {
        self.pool.get().expect("deuce: failed to get database connection")
    }
}
