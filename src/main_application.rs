//! Contains the struct that initializes and runs
//! the application.

use gtk4::gio::Settings;
use gtk4::gio::SettingsBackend;
use log::debug;
use gtk4::prelude::*;
use gtk4::Application;
use gtk4::gio::SettingsSchemaSource;

use super::ui;


/// The GTK Application ID to give the newly
/// created app.
const APPLICATION_ID: &str = "edu.rpi.smashclub.ezstream";

/// The actual application struct, used in main.rs
/// to start the main application.
pub struct MainApplication {
    app: gtk4::Application
}

impl MainApplication {

    /// Instantiates the application, initializing
    /// all widgets required for it to start.
    /// 
    /// # Returns
    /// A new MainApplication struct, ready to
    /// start running.
    pub fn new() -> Self {

        // Start by instantiating the GTK settings
        // object
        debug!("Instantiating/Loading settings...");

        // @TODO: change this unwrap to real error-checking code
        let settings_schema_source = SettingsSchemaSource::from_directory(
            "./data",
            None,
            false
        ).unwrap();

        let settings_schema = settings_schema_source.lookup(APPLICATION_ID, false);
        let settings_schema = match settings_schema {
            Some(s) => s,
            None => {
                panic!("Failed to find the necessary settings schema for the application!");
            }
        };

        let _settings = Settings::new_full(&settings_schema, None::<&SettingsBackend>, None);


        // Next up, creating the application object.
        debug!("Creating application...");
        let m_app = MainApplication {
            app: Application::builder()
                .application_id(APPLICATION_ID)
                .build()
        };

        // This is where the widgets get created.
        debug!("Connecting activation callback to application...");
        m_app.app.connect_activate(|app| {
            debug!("Creating main window, attaching to application...");
            let mainwindow = ui::MainWindow::new(app);

            debug!("Setting P1 Button click callback...");
            mainwindow.shown_screen().p1_character().connect_clicked(|_b| {
                println!("Clicked p1 character button!");
            });

            mainwindow.present();
        });

        m_app
    }

    /// This method will start the application's
    /// main loop.
    /// 
    /// # Returns
    /// An ExitCode that will be reported to the runing
    /// terminal.
    pub fn run_application(&self) -> gtk4::glib::ExitCode {
        debug!("Running application!");
        self.app.run()
    }
}