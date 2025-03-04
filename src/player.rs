/*
reference:
high_id = 0
    low_id = 1
    token = None
    usedVersion = 1
    name = "Brawler"
    eventCount = 4
    highest_trophies = 0
    teamID = 0
    teamStatus = 0
    isReady = False
    selectedCard = [16, 0]
    isTeamInPracticeMode = False
    teamEventIndex = 0
    teamType = 0
    teamStreamMessageCount = 0
    isAdvertiseToBand = False
    matchmakeStartTime = 0
    battleTicks = 0
    club_id = 0
    wifi = 0
    region = "CAT"
    player_status = 3
    last_connection_time = 0
    friends = {}
    room_id = 0
    unlocked_brawlers = {
        0: {'Cards': {0: 1}, 'Skins': [0], 'selectedSkin': 0, 'Trophies': 0, 'HighestTrophies': 0, 'PowerLevel': 0, 'PowerPoints': 0, 'State': 2, 'StarPower': 0}}
    player_experience = 0
    profile_icon = 0
    trophies = 0
    solo_wins = 0
    duo_wins = 0
    ThreeVSThree_wins = 0
    gems = 0
    gold = 92
    chips = 0
    elexir = 0
    coinsdoubler = 0
    coinsbooster = 0
    coins_reward = 0
    map_id = 7
    skin_id = 0
    brawler_id = 0
    team = 0
    x = 1950
    y = 9750
    control_mode = 0
    has_battle_hints = False
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::database::PlayerInfo;

#[derive(Serialize, Deserialize, Clone)]
pub struct BrawlerData {
    pub cards: HashMap<i32, i32>,
    pub skins: Vec<i32>,
    pub selected: i32,
    pub trophies: i32,
    pub highest_trophies: i32,
    pub level: i32,
    pub power_points: i32,
    pub state: i32,
    pub star_power: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub low_id: u32,
    pub high_id: u32,
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
    pub coins_reward: i32,
    pub event_count: i32,

    pub token: Option<String>,
    pub version: u16,
    pub unlocked_brawlers: HashMap<i32, BrawlerData>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "Brawler".to_string(),
            low_id: 1,
            high_id: 0,
            player_experience: 0,
            solo_wins: 0,
            duo_wins: 0,
            three_x_three_wins: 0,
            gems: 0,
            gold: 0,
            elixir: 0,
            chips: 0,
            coins_doubler: 0,
            coins_booster: 0,
            trophies: 0,
            highest_trophies: 0,
            profile_icon: 0,
            room_id: 0,
            last_connection_time: 0,
            player_status: 0,
            region: "".to_string(),
            control_mode: 0,
            has_battle_hints: false,
            coins_reward: 0,
            event_count: 4,

            token: None,
            version: 1,
            unlocked_brawlers: HashMap::new(),
        }
    }

    pub fn load(&mut self, info: &PlayerInfo) -> Result<(), Box<dyn std::error::Error>> {
        let token = self.token.as_ref().ok_or("deuce: cannot load when token is None");
        
        self.name = info.name.clone();
        self.low_id = info.low_id;
        self.player_experience = info.player_experience;
        self.solo_wins = info.solo_wins;
        self.duo_wins = info.duo_wins;
        self.three_x_three_wins = info.three_x_three_wins;
        self.gems = info.gems;
        self.gold = info.gold;
        self.elixir = info.elixir;
        self.chips = info.chips;
        self.coins_doubler = info.coins_doubler;
        self.coins_booster = info.coins_booster;
        self.trophies = info.trophies;
        self.highest_trophies = info.highest_trophies;
        self.profile_icon = 0;
        self.room_id = info.room_id;
        self.last_connection_time = info.last_connection_time;
        self.player_status = info.player_status;
        self.region = info.region.clone();
        self.control_mode = info.control_mode;
        self.has_battle_hints = info.has_battle_hints;
        self.unlocked_brawlers = info.unlocked_brawlers.clone();
        self.coins_reward = info.coins_reward;
        self.event_count = info.event_count;

        Ok(())
    }
}

/*
 name: player.name.clone(),
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
*/
