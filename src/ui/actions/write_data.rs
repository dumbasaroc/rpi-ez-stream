use gtk4::gio::ActionEntry;

use crate::ui::MainWindow;
use crate::application_data::APPLICATION_STATE;


pub const WRITE_DATA_ACTION_NAME: &str = "TESTWRITE";

pub fn create_write_data_action() -> ActionEntry<MainWindow> {
    
    ActionEntry::builder(WRITE_DATA_ACTION_NAME)
        .activate(|_, _, _| {
            let lock = APPLICATION_STATE.lock().expect("Failed to get application state lock.");
            match lock.write_to_data_file() {
                Ok(_) => {},
                Err(e) => { panic!("FAILURE ON WRITE: {}", e); }
            };
        })
        .build()

}
