mod character_data;
mod player_data;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use player_data::PlayerData;
use character_data::*;

// LAZY STATIC BLOCK FOR SINGLETON MUT DATA
lazy_static! {

    pub static ref APPLICATION_STATE: Mutex<ApplicationData> = Mutex::new(
        ApplicationData::init()
    );

    pub static ref MODULE_HANDLER: Mutex<Option<ModuleHandler>> = Mutex::new(
        None
    );

}


// Type definitions

type PlayersHashMap = HashMap<String, PlayerData>;


// Constants
pub const P1_PLAYER_ID: &str = "player_1";
pub const P2_PLAYER_ID: &str = "player_2";

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
        players.insert(P1_PLAYER_ID.to_string(), PlayerData::default());
        players.insert(P2_PLAYER_ID.to_string(), PlayerData::default());

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
