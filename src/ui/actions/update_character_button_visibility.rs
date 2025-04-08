use gtk4::{gio::ActionEntry, prelude::WidgetExt};

use crate::application_data::ModuleHandlerAPI;
use crate::ui::MainWindow;


pub const UPDATE_CHARACTER_BUTTON_VISIBILITY_ACTION_NAME: &str = "UPDATECBUTVIS";

pub fn create_update_character_button_visibility_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(UPDATE_CHARACTER_BUTTON_VISIBILITY_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            win.main_screen().p1_character().set_visible(
                ModuleHandlerAPI::is_module_loaded()
            );
            win.main_screen().p2_character().set_visible(
                ModuleHandlerAPI::is_module_loaded()
            );
        })
        .build()

}
