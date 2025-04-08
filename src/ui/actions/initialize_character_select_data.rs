use gtk4::gio::ActionEntry;

use crate::ui::MainWindow;
use log::info;


pub const INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME: &str = "INITCSS";

pub fn create_initialize_character_select_data_action() -> ActionEntry<MainWindow> {

    use crate::application_data::ModuleHandlerAPI;
    use crate::ui::CharacterButton;
    
    ActionEntry::builder(INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            info!("Initializing character select data...");

            let css = win.character_select_screen();
            let flowbox = css.character_box();
            flowbox.remove_all();

            if ModuleHandlerAPI::is_module_loaded() {
                for character in ModuleHandlerAPI::get_module_characters().unwrap() {
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
