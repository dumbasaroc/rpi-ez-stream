mod character_data;
mod data_trait;
pub use data_trait::AlterApplicationDataState;

mod player_data;
mod player_ids;
pub use player_ids::get_playerid_from_string;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use player_data::PlayerData;
use crate::playerid;
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

/* ******** MODULE HANDLER CHANGE FUNCTION ******* */
pub fn switch_active_module<P>(path: Option<P>) where P: ToString {
    let mut module_handler = MODULE_HANDLER.lock().unwrap();
    match path {
        Some(p) => {
            let path = p.to_string();
            log::debug!("Module path: {:?}", path);
            match ModuleHandler::new(path) {
                Ok(module) => {
                    *module_handler = Some(module);
                },
                Err(e) => {
                    *module_handler = None;
                    log::error!("Encountered an error while switching to new module: {}", e);
                }
            };
        },

        None => {
            *module_handler = None;
        }
    }

    
}


impl data_trait::AlterApplicationDataState for ApplicationData {

    type PlayerIDType = &'static str;

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
