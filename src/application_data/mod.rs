mod application_state;
mod character_data;
mod data_trait;
pub use data_trait::AlterApplicationDataState;

mod player_data;
mod player_ids;
pub use player_ids::get_playerid_from_string;

use lazy_static::lazy_static;
use std::sync::Mutex;

use application_state::ApplicationData;
use character_data::*;
use player_data::PlayerData;

// LAZY STATIC BLOCK FOR SINGLETON MUT DATA
lazy_static! {

    pub static ref APPLICATION_STATE: Mutex<ApplicationData> = Mutex::new(
        ApplicationData::init()
    );

    pub static ref MODULE_HANDLER: Mutex<Option<ModuleHandler>> = Mutex::new(
        None
    );

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

pub struct ApplicationStateAPI;

impl ApplicationStateAPI {

    pub fn get_player_character(player_id: &'static str) -> Option<player_data::CharacterData> {
        todo!();
    }

    pub fn get_player_name(player_id: &'static str) -> String {
        todo!();
    }

    pub fn get_player_score(player_id: &'static str) -> u32 {
        todo!();
    }

    pub fn set_player_score(player_id: &'static str, score: u32) {
        todo!();
    }

    pub fn set_player_tag<P>(player_id: &'static str, tag: P) where P: Into<String> {
        todo!()
    }

    pub fn set_player_character_name<C>(player_id: &'static str, char_data: C) where C: Into<String> {
        todo!()
    }

    pub fn set_player_character_costume(player_id: &'static str, costume_id: u32) {
        todo!()
    }

    pub fn set_player_character_to_none(player_id: &'static str) {
        todo!()
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
