mod application_data;
mod main_application;
mod ui;

#[cfg(not(test))]
use env_logger::{self, Env};
use main_application::MainApplication;


/// Sets up the logger for output, depending
/// on whether we are in a debug context
/// or not.
/// 
/// Debug contexts will log up through "trace".<br>
/// Release contexts will log up through "warn".
#[cfg(not(test))]
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

#[cfg(not(test))]
fn main() -> gtk4::glib::ExitCode {
    initialize_logger();

    let app = MainApplication::new();
    app.run_application()
}

// ---------------
// ---- TESTS ----
// ---------------

#[cfg(test)]
mod tests {
    use gtk_tester::create_test;
    use crate::MainApplication;

    create_test!{
        test_if_app_opens,
        MainApplication,
        |_win| {

        }
    }
}
