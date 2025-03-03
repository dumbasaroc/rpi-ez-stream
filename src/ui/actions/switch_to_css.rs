use gtk4::gio::ActionEntry;

use crate::ui::MainWindow;


pub const SWITCH_TO_CSS_ACTION_NAME: &str = "SWITCHTOCSS";

pub fn create_switch_to_css_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(SWITCH_TO_CSS_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            win.scene_switcher().set_visible_child_full(
                "css",
                gtk4::StackTransitionType::None
            );
        })
        .build()

}
