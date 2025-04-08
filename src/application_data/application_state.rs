use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use super::PlayerData;
use crate::playerid;

// Type definitions

type PlayersHashMap = HashMap<&'static str, PlayerData>;


// Constants
const DATA_FILE_RELATIVE_PATH: &str = "./data.json";

#[derive(serde::Serialize)]
pub struct ApplicationData {
    
    // Player data
    #[cfg(not(test))]
    players: PlayersHashMap,

    #[cfg(test)]
    pub players: PlayersHashMap

}

impl ApplicationData {

    pub fn init() -> Self {

        let mut players: PlayersHashMap = HashMap::new();
        players.insert(playerid!(PLAYER1), PlayerData::default());
        players.insert(playerid!(PLAYER2), PlayerData::default());

        ApplicationData {
            players
        }
    }

    fn get_player_via_id_mut(&mut self, id: &str) -> Option<&mut PlayerData> {
        self.players.get_mut(id)
    }

    pub fn write_to_data_file(&self) -> Result<()> {
        let outfile = File::create(DATA_FILE_RELATIVE_PATH);
        let mut outfile = match outfile {
            Ok(f) => f,
            Err(e) => { return Err(anyhow!(e)); }
        };

        let self_serialized = match serde_json::to_string_pretty(self) {
            Ok(v) => v,
            Err(e) => { return Err(anyhow!(e)); }
        };

        match write!(outfile, "{}", self_serialized) {
            Ok(_) => {},
            Err(e) => { return Err(anyhow!(e)); }
        };

        Ok(())
    }
}

impl super::data_trait::AppStateDataAPI for ApplicationData {

    type PlayerIDType = &'static str;
    type CharacterDataType = super::player_data::CharacterData;

    fn get_player_character(&mut self, id: Self::PlayerIDType) -> Option<Self::CharacterDataType> {
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.character().cloned()
    }

    fn get_player_score(&mut self, id: Self::PlayerIDType) -> u32 {
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.score()
    }

    fn get_player_tag(&mut self, id: Self::PlayerIDType) -> String {
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.name().to_string()
    }

    fn set_player_tag<P>(&mut self, id: Self::PlayerIDType, tag: P) where P: Into<String> {
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.set_name(tag);
    }

    fn set_player_character_name<C>(&mut self, id: Self::PlayerIDType, char_name: C) where C: Into<String> {
        
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.set_character(Some(char_name));
    }

    fn set_player_score(&mut self, id: Self::PlayerIDType, score: u32) {
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.set_score(score);
    }

    fn set_player_character_costume(&mut self, id: Self::PlayerIDType, costume_id: u32) {
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.set_character_costume(costume_id);
    }

    fn set_player_character_to_none(&mut self, id: Self::PlayerIDType) {
        let pdata = match self.get_player_via_id_mut(id) {
            Some(c) => c,
            None => {
                panic!("Tried and failed to access invalid PlayerID \"{}\"", id);
            }
        };

        pdata.set_character::<String>(None);
    }
}
