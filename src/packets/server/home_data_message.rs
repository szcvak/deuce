use crate::packets::packet::ServerPacket;
use crate::writer::ByteWriter;
use crate::player::*;
use crate::database::*;
use crate::csv_logic::*;
use crate::settings::*;
use crate::milestones::*;

use std::collections::HashMap;

const RESOURCE_IDS: [u16; 3] = [1, 5, 6];
const TROPHIES_FOR_RANK: [u32; 34] = [0, 10, 20, 30, 40, 60, 80, 100, 120, 140, 160,180,220,260,300,340,380,420,460,500,550,600,650,700,750,800,850,900,950,1000,1050,1100,1150,1200];

pub struct HomeDataMessage<'a> {
    pub id: u16,
    
    player: &'a mut Player,
    info: &'a PlayerInfo,

    settings: &'a Settings,
}

impl<'a> HomeDataMessage<'a> {
    pub fn new(player: &'a mut Player, info: &'a PlayerInfo, settings: &'a Settings) -> Self {
        Self {
            id: 24101,
            
            player,
            info,

            settings,
        }
    }
}

impl<'a> ServerPacket for HomeDataMessage<'a> {
    fn encode(&mut self) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        // load player data
        self.player.load(self.info);

        // load csv data
        let skins = Skins::get_skins();
        let unlock_cards = Cards::get_brawlers();
        let chars = Characters::get_brawlers();
        let cards = Cards::get_cards();

        // resources
        let resources = [self.player.gold, self.player.chips, self.player.elixir];

        self.player.player_status = 2;
        // TODO: replace in DB

        let mut brawlers_trophies = 1000;

        let max_rank = self.settings.max_rank;
        let max_upgrade_level = 5;

        let trophies = [0,10,20,30,40,60,80,100,120,140,160,180,220,260,300,340,380,420,460,500,550,600,650,700,750,800,850,900,950,1000,1050,1100,1150,1200];

        if max_rank <= 34 {
            brawlers_trophies = trophies[(max_rank - 1) as usize];
        } else {
            brawlers_trophies = trophies[33] + (50 * (max_rank - 34));
        }

        writer.write_vint(2017189);
        writer.write_vint(10);

        writer.write_vint(self.player.trophies as i32);
        writer.write_vint(self.player.highest_trophies as i32);
        writer.write_vint(0);
        writer.write_vint(self.player.player_experience as i32);

        writer.write_sc_id(28, self.player.profile_icon as i32);

        writer.write_vint(7);

        for x in 0..7 {
            writer.write_vint(x);
        }

        let non_zero: Vec<i32> = self.player.unlocked_brawlers
            .values()
            .filter(|b| b.selected != 0)
            .map(|b| b.selected)
            .collect();

        writer.write_vint(non_zero.len() as i32);

        for skin in non_zero {
            writer.write_sc_id(29, skin);
        }

        let non_zero_skins: Vec<i32> = self.player.unlocked_brawlers
            .values()
            .flat_map(|b| b.skins.iter())
            .filter(|&&skin| skin != 0)
            .cloned()
            .collect();

        writer.write_vint(non_zero_skins.len() as i32);

        for skin in non_zero_skins {
            writer.write_sc_id(29, skin);
        }

        writer.write_boolean(true);
        writer.write_vint(0);

        writer.write_vint(self.player.coins_reward);
        writer.write_boolean(false);
        writer.write_vint(self.player.control_mode as i32);
        writer.write_boolean(self.player.has_battle_hints);
        writer.write_vint(self.player.coins_doubler as i32);
        
        writer.write_vint(0);
        writer.write_vint(0);
        writer.write_boolean(false);

        writer.write_vlong(0, 1);
        writer.write_vlong(0, 1);
        writer.write_vlong(0, 1);
        writer.write_vlong(0, 1);

        writer.write_sc_id(0, 1);

        writer.write_vint(0);

        writer.write_boolean(true);
        writer.write_boolean(true);

        writer.write_vint(2017189);

        writer.write_vint(100);
        writer.write_vint(10);
        writer.write_vint(80);
        writer.write_vint(10);
        writer.write_vint(20);
        writer.write_vint(50);
        writer.write_vint(50);
        writer.write_vint(1000);
        writer.write_vint(7 * 24);
        writer.write_vint(brawlers_trophies as i32);
        writer.write_vint(50);
        writer.write_vint(9999);

        writer.write_array_vint(vec![1, 2, 5, 10, 20, 60]);
        writer.write_array_vint(vec![3, 10, 20, 60, 200, 500]);
        writer.write_array_vint(vec![0, 30, 80, 170, 0, 0]);

        writer.write_vint(self.player.event_count);
        
        let required = [0, 3, 5, 8];

        for event in 0..self.player.event_count {
            writer.write_vint(event + 1);
            writer.write_vint(required[event as usize]);
        }

        writer.write_vint(self.player.event_count);

        for event in 0..self.player.event_count {
            writer.write_vint(event + 1);
            writer.write_vint(event + 1);

            writer.write_vint(1);
            writer.write_vint(39120);
            writer.write_vint(8);
            writer.write_vint(8);
            writer.write_vint(999);

            writer.write_boolean(false);
            writer.write_boolean(event == 4);

            writer.write_sc_id(15, 2);

            writer.write_vint(0);
            writer.write_vint(2);

            writer.write_string(Some("deuce server v1.01"));
            writer.write_boolean(false);
        }

        writer.write_vint(4);

        for event in 0..4 {
            writer.write_vint(event + 1);
            writer.write_vint(event + 1);

            writer.write_vint(1337);
            writer.write_vint(39120);
            writer.write_vint(8);
            writer.write_vint(8);
            writer.write_vint(999);

            writer.write_boolean(false);
            writer.write_boolean(event == 4);

            writer.write_sc_id(15, 3);

            writer.write_vint(0);
            writer.write_vint(2);

            writer.write_string(Some("deuce server v1.01 (2)"));
            writer.write_boolean(false);
        }

        writer.write_vint(max_upgrade_level);

        for x in 0..max_upgrade_level {
            writer.write_vint(x + 1);
        }

        // TODO: milestones
        let milestones = Milestones::new(self.settings);
        milestones.write_all(&mut writer);
        
        writer.write_long(self.player.high_id as i32, self.player.low_id as i32);
        writer.write_vint(0);

        for id in 0..3 {
            writer.write_vlong(self.player.high_id as i32, self.player.low_id as i32);
        }

        writer.write_string(Some(self.player.name.as_str()));
        writer.write_boolean(self.player.name.as_str() != "Brawler");
        writer.write_int(1);

        writer.write_vint(5);

        let mut cards: HashMap<u32, u32> = HashMap::new();

        for (id, brawler) in self.player.unlocked_brawlers.iter() {
            for (&card, &amt) in brawler.cards.iter() {
                cards.insert(card as u32, amt as u32);
            }
        }

        writer.write_vint((cards.len() + RESOURCE_IDS.len()) as i32);

        for (&card, &amt) in cards.iter() {
            writer.write_sc_id(23, card as i32);
            writer.write_vint(amt as i32);
        }

        for (i, &res) in RESOURCE_IDS.iter().enumerate() {
            writer.write_sc_id(5, res as i32);
            writer.write_vint(resources[i] as i32);
        }

        let b_count = self.player.unlocked_brawlers.len();

        writer.write_vint(b_count as i32);

        for (&id, data) in self.player.unlocked_brawlers.iter() {
            writer.write_sc_id(16, id as i32);
            writer.write_vint(data.trophies as i32);
        }

        writer.write_vint(b_count as i32);

        for (&id, data) in self.player.unlocked_brawlers.iter() {
            writer.write_sc_id(16, id as i32);
            writer.write_vint(data.highest_trophies as i32);
        }

        writer.write_vint(0);

        writer.write_vint(b_count as i32);

        for (&id, data) in self.player.unlocked_brawlers.iter() {
            writer.write_sc_id(16, id as i32);
            writer.write_vint(2);
        }

        writer.write_vint(self.player.gems as i32);
        writer.write_vint(13);

        writer.write_vint(0);
        writer.write_vint(0);
        writer.write_vint(0);
        writer.write_vint(0);
        writer.write_vint(0);
        writer.write_vint(0);
        writer.write_vint(0);
        writer.write_vint(0);

        writer.write_vint(2);
        writer.write_vint(2017189);

        self.player.coins_reward = 0;

        writer.buffer
    }
}

