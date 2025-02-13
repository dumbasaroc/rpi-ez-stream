/// Used to easily define a test, given
/// a test name and the steps to take.
/// ~~Should return a Result<(), String>.~~
#[macro_export]
macro_rules! create_test {
    ( $fn_name:ident, $app_ty:ty, $($t:tt)* ) => {
        #[gtk4::test]
        fn $fn_name() {
            let app = <$app_ty>::new( $($t)* );
            let code = app.run_application();
            assert!(code == gtk4::glib::ExitCode::SUCCESS);
        }       
    };
}


/// Used to emulate a GTK4 button press
/// by manually triggering the signal
/// emitted when clicked.
/// 
/// # Usage
/// ```
/// click_button!(button);
/// ```
#[macro_export]
macro_rules! click_button {
    ( $x: expr ) => {
        $x.emit_clicked();
    };
}
