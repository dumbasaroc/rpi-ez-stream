//! Contains the struct that initializes and runs
//! the application.

use log::debug;
use gtk4::prelude::*;
use gtk4::Application;
use gtk4::gio::ActionEntry;

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
        debug!("Creating application...");
        let m_app = MainApplication {
            app: Application::builder()
                .application_id(APPLICATION_ID)
                .build()
        };

        debug!("Connecting activation callback to application...");
        m_app.app.connect_activate(|app| {
            debug!("Creating main window, attaching to application...");
            let mainwindow = ui::MainWindow::new(app);

            debug!("Setting P1 Button click callback...");
            mainwindow.shown_screen().p1_character().connect_clicked(|_b| {
                println!("Clicked p1 character button!");
            });

            debug!("Creating a dummy action to close the window!");
            let action_close = ActionEntry::builder("close")
                .activate(|window: &ui::MainWindow, _, _| {
                    window.close();
                })
                .build();
            mainwindow.add_action_entries([action_close]);

            mainwindow.present();
        });

        debug!("Adding shortcut for closing the application.");
        m_app.app.set_accels_for_action("win.close", &["<Ctrl>W"]);

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