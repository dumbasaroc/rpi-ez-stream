use gtk4::gio::ActionEntry;

use crate::ui::MainWindow;
use log::info;


pub const INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME: &str = "INITCSS";

pub fn create_initialize_character_select_data_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            info!("Initializing character select data...");
        })
        .build()

}
