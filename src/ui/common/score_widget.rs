use crate::application_data::APPLICATION_STATE;

pub type ScoreEntry = gtk4::SpinButton;

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
        let player = lock.get_player_via_id_mut(
            player_id
        ).unwrap();

        player.set_score(val);
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
            use crate::application_data::{APPLICATION_STATE, P1_PLAYER_ID};
            
            let score_input_p1 = win.main_screen().p1_score_input();
            score_input_p1.spin(
                gtk4::SpinType::StepForward,
                5.0
            );

            let mut data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.get_player_via_id_mut(P1_PLAYER_ID).unwrap();
            assert!(p1_data.score() == 5);
        }
    }

    create_test!{
        test_decrement_normal,
        MainApplication,
        |win| {
            use crate::application_data::{APPLICATION_STATE, P1_PLAYER_ID};
            
            let score_input_p1 = win.main_screen().p1_score_input();
            score_input_p1.set_value(7.0);
            score_input_p1.spin(
                gtk4::SpinType::StepBackward,
                5.0
            );

            let mut data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.get_player_via_id_mut(P1_PLAYER_ID).unwrap();
            assert!(p1_data.score() == 2);
        }
    }

        create_test!{
        test_increment_at_max,
        MainApplication,
        |win| {
            use crate::application_data::{APPLICATION_STATE, P1_PLAYER_ID};
            
            let score_input_p1 = win.main_screen().p1_score_input();
            let limit: u32 = 100;

            score_input_p1.set_value(limit as f64);
            score_input_p1.spin(
                gtk4::SpinType::StepForward,
                5.0
            );

            let mut data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.get_player_via_id_mut(P1_PLAYER_ID).unwrap();
            assert!(p1_data.score() == 100);
        }
    }

    create_test!{
        test_decrement_at_min,
        MainApplication,
        |win| {
            use crate::application_data::{APPLICATION_STATE, P1_PLAYER_ID};
            
            let score_input_p1 = win.main_screen().p1_score_input();
            score_input_p1.set_value(1.0);
            score_input_p1.spin(
                gtk4::SpinType::StepBackward,
                5.0
            );

            let mut data = APPLICATION_STATE.lock().unwrap();
            let p1_data = data.get_player_via_id_mut(P1_PLAYER_ID).unwrap();
            assert!(p1_data.score() == 0);
        }
    }
}
