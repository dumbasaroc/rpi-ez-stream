use gtk4::prelude::*;
use gtk4::gio::ActionEntry;

use crate::ui::MainWindow;
use log::info;


pub const INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME: &str = "INITCSS";

pub fn create_initialize_character_select_data_action() -> ActionEntry<MainWindow> {

    use crate::application_data::MODULE_HANDLER;
    use crate::ui::CharacterButton;
    
    ActionEntry::builder(INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            info!("Initializing character select data...");

            let css = win.character_select_screen();
            let flowbox = css.character_box();
            flowbox.remove_all();

            let module_handler = MODULE_HANDLER.lock().unwrap();
            if module_handler.is_some() {
                for character in &(module_handler.as_ref().unwrap().characters) {
                    let char_button = CharacterButton::new(
                        character.display_name.clone(),
                        character.aliases.clone()
                    );
                    flowbox.insert(&char_button, -1);
                }
            }
            
        })
        .build()

}
