//! Contains the struct that initializes and runs
//! the application.

use gtk4::gio::ApplicationFlags;
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
    /// # Parameters
    /// - `testing_callback` (only when run with `cargo test`):
    /// A function that will run commands on the main window
    /// just after creating it, simulating user input for
    /// automatic testing purposes. See the `gtk_tester` crate
    /// for more information.
    /// 
    /// # Returns
    /// A new MainApplication struct, ready to
    /// start running.
    pub fn new(
        #[cfg(test)]
        testing_callback: impl Fn(&ui::MainWindow) + 'static
    ) -> Self {

        // First, load the UI from the compiled
        // gresource
        gtk4::gio::resources_register_include!(
            "ui_resource.gresource"
        ).expect("Could not load UI gresource... aborting launch.");

        // Then, we instantiate the GTK settings
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

        let settings = Settings::new_full(&settings_schema, None::<&SettingsBackend>, None);

        // Next up, creating the application object.
        debug!("Creating application...");
        let m_app = MainApplication {
            app: Application::builder()
                .application_id(APPLICATION_ID)
                .flags(ApplicationFlags::HANDLES_COMMAND_LINE)
                .build()
        };

        m_app.app.connect_command_line(|app, _| {
            app.activate();
            0
        });

        // This is where the widgets get created.
        debug!("Connecting activation callback to application...");
        m_app.app.connect_activate(move |app| {
            debug!("Creating main window, attaching to application...");
            let mainwindow = ui::MainWindow::new(app);
        
            #[cfg(not(test))]
            {
                debug!("Setting widget callbacks and properties...");
                instantiate_widget_properties(&mainwindow, &settings);
            }

            #[cfg(test)]
            testing_callback(&mainwindow);

            mainwindow.present();
            
            #[cfg(test)]
            mainwindow.close();
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

/// @TODO Repurpose or remove this method, it basically
/// does nothing besides binding settings to parameters,
/// which can probably be done elsewhere.
#[allow(unused_variables)]
#[cfg(not(test))]
fn instantiate_widget_properties(win: &ui::MainWindow, settings: &Settings) {

    use settings_data::*;
    
    {
        settings.bind(P1_SCORE_SETTING_KEY, &win.main_screen().p1_score_input(), "value").build();
        settings.bind(P2_SCORE_SETTING_KEY, &win.main_screen().p2_score_input(), "value").build();
        settings.bind(P1_NAME_SETTING_KEY, &win.main_screen().p1_name_input(), "text").build();
        settings.bind(P2_NAME_SETTING_KEY, &win.main_screen().p2_name_input(), "text").build();
    }
}

