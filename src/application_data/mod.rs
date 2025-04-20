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
fn switch_active_module<P>(path: Option<P>) where P: ToString {
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

    pub fn set_bestof_firstto(is_bestof: bool) {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_is_bestof(is_bestof);
    }

    pub fn set_bestof_firstto_counter(ctr: u32) {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_bestof_firstto_counter(ctr);
    }

    pub fn set_tournament_name<C>(name: C) where C: Into<String> {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_tournament_name(name);
    }

    pub fn set_bracket_location<C>(loc: C) where C: Into<String> {
        let mut appstate = APPLICATION_STATE.lock().unwrap();
        appstate.set_bracket_location(loc);
    }
}

pub struct ModuleHandlerAPI;

#[allow(dead_code)]
impl ModuleHandlerAPI {

    pub fn load_module<P, W>(initiator: &W, modpath: Option<P>) -> anyhow::Result<()> where
        P: ToString,
        W: gtk4::prelude::WidgetExt
    {
        use crate::ui::actions;
        use crate::playerid;

        match modpath {
            None => {
                let mut module_state = MODULE_HANDLER.lock().unwrap();
                *module_state = None;
            },
            Some(path) => {
                let path = path.to_string();
                switch_active_module(Some(path));       
            }
        }

        let module_state = MODULE_HANDLER.lock().unwrap();

        match module_state.as_ref() {
            Some(module) => {
                ApplicationStateAPI::set_player_character_name(
                    playerid!(PLAYER1),
                    module.default_character.display_name.clone()
                );
                ApplicationStateAPI::set_player_character_name(
                    playerid!(PLAYER2),
                    module.default_character.display_name.clone()
                );
            },
            None => {
                ApplicationStateAPI::set_player_character_to_none(playerid!(PLAYER1));
                ApplicationStateAPI::set_player_character_to_none(playerid!(PLAYER2));
            }
        };

        drop(module_state);

        initiator.activate_action(
            format!("win.{}", actions::UPDATE_CHARACTER_BUTTON_VISIBILITY_ACTION_NAME).as_str(),
            None
        ).unwrap();

        initiator.activate_action(
            format!("win.{}", actions::INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME).as_str(),
            None
        ).unwrap();

        Ok(())
    }

    pub fn is_module_loaded() -> bool {
        let modhandler = MODULE_HANDLER.lock().unwrap();
        modhandler.is_some()
    }

    pub fn get_module_path() -> Option<std::path::PathBuf> {
        let modhandler = MODULE_HANDLER.lock().unwrap();
        if modhandler.is_none() { return None; }

        Some(modhandler.as_ref().unwrap().base_directory_path.clone())
    }

    pub fn get_module_name() -> Option<String> {
        let modhandler = MODULE_HANDLER.lock().unwrap();
        if modhandler.is_none() { return None; }

        Some(modhandler.as_ref().unwrap().current_module_name.clone())
    }

    pub fn get_module_default_character() -> Option<character_data::CharacterData> {
        let modhandler = MODULE_HANDLER.lock().unwrap();
        if modhandler.is_none() { return None; }

        Some(modhandler.as_ref().unwrap().default_character.clone())
    }

    pub fn get_module_characters() -> Option<Vec<character_data::CharacterData>> {
        let modhandler = MODULE_HANDLER.lock().unwrap();
        if modhandler.is_none() { return None; }

        Some(modhandler.as_ref().unwrap().characters.clone())
    }

    pub fn list_modules_in_folder() -> std::io::Result<Vec<(std::path::PathBuf, String)>>{
        const MODULES_FOLDER: &str = "./res/modules/";

        let dirs = match std::fs::read_dir(MODULES_FOLDER) {
            Ok(d) => d,
            Err(e) => {

                // Fix "modules" folder not existing
                // by creating it and trying again
                if e.kind() == std::io::ErrorKind::NotFound {
                    std::fs::create_dir_all(MODULES_FOLDER)?;
                    return ModuleHandlerAPI::list_modules_in_folder();
                }

                return Err(e);
            }
        };

        let mut out_mods: Vec<(std::path::PathBuf, String)> = vec![];

        for path in dirs {
            let path = path?;

            match path.metadata()?.is_dir() {
                true => {                    
                    match ModuleHandler::new(path.path()) {
                        Ok(mh) => {
                            // println!("  - Module display name: {}", mh.current_module_name);
                            out_mods.push((
                                path.path().clone(),
                                mh.current_module_name.clone()
                            ));
                        },
                        Err(e) => {
                            log::warn!(
                                "Invalid module detected at {}\n{}",
                                path.path().to_str().unwrap(),
                                e
                            );
                        }
                    }
                },
                _ => {}
            }
        }

        Ok(out_mods)
    }

}
