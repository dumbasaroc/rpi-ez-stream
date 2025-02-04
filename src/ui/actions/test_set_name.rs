use gtk4::prelude::*;
use gtk4::gio::ActionEntry;
use crate::ui::MainWindow;

pub const TEST_SET_NAME_ACTION_NAME: &str = "";

pub fn create_test_set_name_action() -> ActionEntry<MainWindow> {

    ActionEntry::builder(TEST_SET_NAME_ACTION_NAME)
        // .parameter_type(Some(&i32::static_variant_type()))
        .activate(|win: &MainWindow, _b, _param| {
            win.shown_screen().p1_text_input().set_text("Something here.");
            // let param = c.unwrap().get::<i32>().unwrap();
        })
        .build()
    
    // SimpleAction::new(name, parameter_type)
}
