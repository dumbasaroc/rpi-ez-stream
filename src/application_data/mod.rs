mod player_data;
mod player_ids;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use player_data::PlayerData;
use crate::playerid;

// LAZY STATIC BLOCK FOR SINGLETON MUT DATA
lazy_static! {

    pub static ref APPLICATION_STATE: Mutex<ApplicationData> = Mutex::new(
        ApplicationData::init()
    );

}


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

    pub fn get_player_via_id_mut(&mut self, id: &str) -> Option<&mut PlayerData> {
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
