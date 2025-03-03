use gtk4::gio::prelude::ActionMapExtManual;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::*;

use crate::application_data::{P1_PLAYER_ID, P2_PLAYER_ID};
use crate::ui::actions;
use super::MainScreen;


glib::wrapper! {

    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Buildable;

}

impl MainWindow {
    pub fn new(app: &Application) -> Self {

        use crate::ui::common as cmn;

        // Create new window
        let win: MainWindow = glib::Object::builder().property("application", app).build();
        win.instantiate_actions();

        win.shown_screen().p1_name_input().set_change_callback(P1_PLAYER_ID);
        win.shown_screen().p2_name_input().set_change_callback(P2_PLAYER_ID);

        cmn::instantiate_score_entry(&win.shown_screen().p1_score_input(), P1_PLAYER_ID);
        cmn::instantiate_score_entry(&win.shown_screen().p2_score_input(), P2_PLAYER_ID);

        win
    }

    fn instantiate_actions(&self) {

        const MAIN_WINDOW_GROUP_PREFIX: &str = "win";

        // Add in the actions that we need buttons
        // and the like to call when they're activated.

        self.add_action_entries([
            actions::create_write_data_action(),
            actions::create_initialize_character_select_data_action()
        ]);
        
        // Connect the action to the update button.
        self.shown_screen().update_button().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::WRITE_DATA_ACTION_NAME).as_str() )
        );

        // Connect "initialize CSS" action to a button
        self.shown_screen().p1_character().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME).as_str() )
        );

        self.shown_screen().p2_character().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::INITIALIZE_CHARACTER_SELECT_DATA_ACTION_NAME).as_str() )
        );

    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    use super::MainScreen;

    #[allow(non_camel_case_types)]
    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::MainWindow)]
    #[template(file = "main_window.ui")]
    pub struct MainWindow {

        #[template_child]
        #[property(get)]
        pub shown_screen: TemplateChild<MainScreen>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {

        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = gtk4::ApplicationWindow;


        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }


    impl MainWindow {}


    impl ApplicationWindowImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl WidgetImpl for MainWindow {}

    #[glib::derived_properties]
    impl ObjectImpl for MainWindow {}
}
