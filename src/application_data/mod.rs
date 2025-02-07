mod player_data;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use player_data::PlayerData;

// LAZY STATIC BLOCK FOR SINGLETON MUT DATA
lazy_static! {

    pub static ref APPLICATION_STATE: Mutex<ApplicationData> = Mutex::new(
        ApplicationData::init()
    );

}


// Type definitions

type PlayersHashMap = HashMap<String, PlayerData>;


// Constants
const P1_PLAYER_ID: &str = "player_1";
const P2_PLAYER_ID: &str = "player_2";

const DATA_FILE_RELATIVE_PATH: &str = "./data.json";

#[derive(serde::Serialize)]
pub struct ApplicationData {
    
    // Player data
    players: PlayersHashMap

}

impl ApplicationData {

    pub fn init() -> Self {

        let mut players: PlayersHashMap = HashMap::new();
        players.insert(format!("{}", P1_PLAYER_ID), PlayerData::default());
        players.insert(format!("{}", P2_PLAYER_ID), PlayerData::default());

        ApplicationData {
            players
        }
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
