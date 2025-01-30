//! Contains the struct that initializes and runs
//! the application.

use log::info;
use gtk4::prelude::*;
use gtk4::Application;

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
        let m_app = MainApplication {
            app: Application::builder()
                .application_id(APPLICATION_ID)
                .build()
        };

        m_app.app.connect_activate(|app| {
            let mainwindow = ui::MainWindow::new(app);

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
        self.app.run()
    }
}