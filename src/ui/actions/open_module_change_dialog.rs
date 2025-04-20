use gtk4::prelude::*;
use gtk4::gio::ActionEntry;

use crate::ui::MainWindow;


pub const OPEN_MODULE_CHANGE_DIALOG_ACTION_NAME: &str = "OPENMODULEDIALOG";

pub fn create_open_module_change_dialog_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(OPEN_MODULE_CHANGE_DIALOG_ACTION_NAME)
        .activate(|win: &MainWindow, _, _| {
            use crate::application_data::ModuleHandlerAPI;

            let mod_select = crate::ui::ModuleSelector::new(win);
            mod_select.set_application(win.application().as_ref());
            mod_select.present();
            
            match ModuleHandlerAPI::list_modules_in_folder() {
                Ok(v) => {
                    for (path, mod_name) in v {
                        println!("  - Found module \"{}\" at \"{}\"", mod_name, path.to_str().unwrap());
                    }
                },
                Err(e) => {
                    log::error!(
                        "Failed to load module listings from path: {}", e
                    )
                }
            }
        })
        .build()

}
