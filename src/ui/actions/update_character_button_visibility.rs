use gtk4::{gio::ActionEntry, prelude::WidgetExt};

use crate::ui::MainWindow;


pub const UPDATE_CHARACTER_BUTTON_VISIBILITY_ACTION_NAME: &str = "UPDATECBUTVIS";

pub fn create_update_character_button_visibility_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(UPDATE_CHARACTER_BUTTON_VISIBILITY_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            let module_handler = crate::application_data::MODULE_HANDLER.lock().unwrap();
            win.main_screen().p1_character().set_visible(
                module_handler.is_some()
            );
            win.main_screen().p2_character().set_visible(
                module_handler.is_some()
            );
        })
        .build()

}
