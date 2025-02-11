mod application_data;
mod main_application;
mod ui;

use env_logger::{self, Env};
use main_application::MainApplication;

#[cfg(test)]
use gtk_tester::*;

/// Sets up the logger for output, depending
/// on whether we are in a debug context
/// or not.
/// 
/// Debug contexts will log up through "trace".<br>
/// Release contexts will log up through "warn".
fn initialize_logger() {
    #[cfg(debug_assertions)]
    env_logger::init_from_env(
        Env::default()
            .default_filter_or("trace")
    );

    #[cfg(not(debug_assertions))]
    env_logger::init_from_env(
        Env::default()
            .default_filter_or("warn")  
    );
}

fn main() -> gtk4::glib::ExitCode {
    initialize_logger();

    let app = MainApplication::new();
    app.run_application()
}

#[test]
fn test_application() {
    use gtk_tester::GTKTester;

    impl GTKTester<String> for MainApplication {

        fn steps() -> Result<(), String> {

            Ok(())
        }
    }

    let app = MainApplication::new();
    app.run_application();
}
