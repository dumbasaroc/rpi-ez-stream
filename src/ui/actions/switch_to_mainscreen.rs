use gtk4::{gio::ActionEntry, prelude::{ButtonExt, EditableExt}};

use crate::ui::MainWindow;


pub const SWITCH_TO_MAINSCREEN_ACTION_NAME: &str = "SWITCHTOMAINSCREEN";

pub fn create_switch_to_mainscreen_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(SWITCH_TO_MAINSCREEN_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {

            use crate::application_data::APPLICATION_STATE;
            use crate::playerid;

            let mut app_state = APPLICATION_STATE.lock().unwrap();

            win.main_screen().p1_character().set_label(
                format!(
                    "Player 1 Character\n({})",
                    match app_state.get_player_via_id_mut(playerid!(PLAYER1)).unwrap().character() {
                        Some(c) => c.character_name.clone(),
                        None => "NoCharacter".to_string()
                    }
                ).as_str()
            );
            win.main_screen().p2_character().set_label(
                format!(
                    "Player 2 Character\n({})",
                    match app_state.get_player_via_id_mut(playerid!(PLAYER2)).unwrap().character() {
                        Some(c) => c.character_name.clone(),
                        None => "NoCharacter".to_string()
                    }
                ).as_str()
            );

            drop(app_state);

            win.scene_switcher().set_visible_child_full(
                "mainscreen",
                gtk4::StackTransitionType::None
            );
            win.character_select_screen().search_bar().set_text("");
        })
        .build()

}

