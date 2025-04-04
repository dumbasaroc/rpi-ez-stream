use crate::application_data::{AlterApplicationDataState, APPLICATION_STATE};

pub type ScoreEntry = gtk4::SpinButton;

/// This is akin to a "new" function, but
/// performed on an existing object. It
/// sets minimums and maximums for the
/// spinbuttons, as well as sets up the
/// callback for when the value is changed.
/// 
/// @TODO Follow the example in
/// CharacterSelectButton::on_click(),
/// it handles this much more gracefully
/// than we do here.
/// 
/// # Parameters
/// - `entry`: The ScoreEntry widget to propogate
/// data to.
/// - `player_id`: The Player ID that this ScoreEntry
/// should alter when it changes value. See
/// the `playerid!()` macro for more information.
pub fn instantiate_score_entry(entry: &ScoreEntry, player_id: &'static str) {
    const MIN: f64 = 0.0;
    const MAX: f64 = 100.0;

    let adjustment = gtk4::Adjustment::builder()
        .lower(MIN)
        .upper(MAX)
        .step_increment(1.0)
        .build();
    entry.set_adjustment(&adjustment);

    entry.connect_value_changed(move |x| {
        let val: u32 = x.value() as u32;
        let mut lock = APPLICATION_STATE.lock().unwrap();
        lock.set_player_score(player_id, val);
    });
}


#[allow(unused_imports)]
mod tests {

    use crate::MainApplication;
    use gtk4::prelude::EditableExt;
    use gtk_tester::*;

    create_test!{
        test_increment_normal,
        MainApplication,
        |win| {
            use crate::application_data::APPLICATION_STATE;
            use crate::playerid;
            
            let score_input_p1 = win.main_screen().p1_score_input();
            score_input_p1.spin(
                gtk4::SpinType::StepForward,
                5.0
            );

            let data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.players.get(playerid!(PLAYER1)).unwrap();
            assert!(p1_data.score() == 5);
        }
    }

    create_test!{
        test_decrement_normal,
        MainApplication,
        |win| {
            use crate::application_data::APPLICATION_STATE;
            use crate::playerid;
            
            let score_input_p1 = win.main_screen().p1_score_input();
            score_input_p1.set_value(7.0);
            score_input_p1.spin(
                gtk4::SpinType::StepBackward,
                5.0
            );

            let data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.players.get(playerid!(PLAYER1)).unwrap();
            assert!(p1_data.score() == 2);
        }
    }

        create_test!{
        test_increment_at_max,
        MainApplication,
        |win| {
            use crate::application_data::APPLICATION_STATE;
            use crate::playerid;
            
            let score_input_p1 = win.main_screen().p1_score_input();
            let limit: u32 = 100;

            score_input_p1.set_value(limit as f64);
            score_input_p1.spin(
                gtk4::SpinType::StepForward,
                5.0
            );

            let data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.players.get(playerid!(PLAYER1)).unwrap();
            assert!(p1_data.score() == 100);
        }
    }

    create_test!{
        test_decrement_at_min,
        MainApplication,
        |win| {
            use crate::application_data::APPLICATION_STATE;
            use crate::playerid;
            
            let score_input_p1 = win.main_screen().p1_score_input();
            score_input_p1.set_value(1.0);
            score_input_p1.spin(
                gtk4::SpinType::StepBackward,
                5.0
            );

            let data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.players.get(playerid!(PLAYER1)).unwrap();
            assert!(p1_data.score() == 0);
        }
    }
}
