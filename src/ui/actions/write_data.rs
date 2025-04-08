use gtk4::gio::ActionEntry;

use crate::application_data::ApplicationStateAPI;
use crate::ui::MainWindow;


pub const WRITE_DATA_ACTION_NAME: &str = "TESTWRITE";

pub fn create_write_data_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(WRITE_DATA_ACTION_NAME)
        .activate(|_, _, _| {
            match ApplicationStateAPI::write_to_data_file() {
                Ok(_) => {},
                Err(e) => { panic!("FAILURE ON WRITE: {}", e); }
            };
        })
        .build()

}
