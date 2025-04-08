mod application_state;
mod character_data;
mod data_trait;
pub use data_trait::AppStateDataAPI;

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

#[allow(dead_code)]
impl ApplicationStateAPI {

    pub fn write_to_data_file() -> anyhow::Result<()> {
        let appstate = APPLICATION_STATE.lock().unwrap();
        appstate.write_to_data_file()
    }

    pub fn get_player_character(player_id: &'static str) -> Option<player_data::CharacterData> {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.get_player_character(player_id)
    }

    pub fn get_player_tag(player_id: &'static str) -> String {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.get_player_tag(player_id)
    }

    pub fn get_player_score(player_id: &'static str) -> u32 {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.get_player_score(player_id)
    }

    pub fn set_player_score(player_id: &'static str, score: u32) {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_player_score(player_id, score);
    }

    pub fn set_player_tag<P>(player_id: &'static str, tag: P) where P: Into<String> {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_player_tag(player_id, tag);
    }

    pub fn set_player_character_name<C>(player_id: &'static str, name: C) where C: Into<String> {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_player_character_name(player_id, name);
    }

    pub fn set_player_character_costume(player_id: &'static str, costume_id: u32) {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_player_character_costume(player_id, costume_id);
    }

    pub fn set_player_character_to_none(player_id: &'static str) {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_player_character_to_none(player_id);
    }
}

