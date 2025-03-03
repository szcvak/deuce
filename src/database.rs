use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

use log::*;
use serde::{Deserialize, Serialize};
use crate::player::Player;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;
pub type DbConn = PooledConnection<PostgresConnectionManager<NoTls>>;

pub struct Database {
    pool: DbPool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    name: String,
    low_id: u32,
    club_id: u32,
    club_role: u32,
    player_experience: u32,
    solo_wins: u32,
    duo_wins: u32,
    three_x_there_wins: u32,
    gems: u32,
    gold: u32,
    elixir: u32,
    chips: u32,
    coins_reward: u32,
    coins_doubler: u32,
    coins_booster: u32,
    trophies: u32,
    highest_trophies: u32,
    profile_icon: u32,
    room_id: u32,
    last_connection_time: u32,
    player_status: u32,
    region: String,
    control_mode: u32,
    has_battle_hints: bool,
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

        if result.get::<&str, u32>("count") > 0 {
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
            three_x_there_wins: player.three_x_there_wins,
            gems: player.gems,
            gold: player.gold,
            elixir: player.elixir,
            chips: player.chips,
            coins_reward: 0,
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

        if result.get::<&str, u32>("count") > 0 {
            return true;
        }
        
        false
    }
    
    pub fn get_free_id(&self) -> u32 {
        let mut conn = self.get_conn();
        
        let result = conn.query_one(
            "SELECT COUNT(*) as count FROM players;",
            &[]
        ).expect("deuce: failed to execute sql");
        
        result.get::<&str, u32>("count") + 1
    }

    pub fn get_conn(&self) -> DbConn {
        self.pool.get().expect("deuce: failed to get database connection")
    }
}
