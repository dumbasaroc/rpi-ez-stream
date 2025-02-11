mod testing_trait;
pub use testing_trait::*;


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
