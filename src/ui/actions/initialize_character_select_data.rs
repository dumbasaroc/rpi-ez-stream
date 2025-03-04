use gtk4::prelude::*;
use gtk4::gio::ActionEntry;

use crate::ui::MainWindow;
use log::info;


pub const INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME: &str = "INITCSS";

pub fn create_initialize_character_select_data_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            info!("Initializing character select data...");

            let css = win.character_select_screen();
            let flowbox = css.character_box();
            flowbox.remove_all();

            let go_back_button = gtk4::Button::builder()
                .label("Go back.")
                .build();
            go_back_button.set_action_name(
                Some( format!("win.{}", crate::ui::actions::SWITCH_TO_MAINSCREEN_ACTION_NAME).as_str() )
            );
            
            flowbox.insert(&go_back_button, -1);
        })
        .build()

}
