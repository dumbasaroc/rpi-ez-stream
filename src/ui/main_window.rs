use gtk4::gio::prelude::ActionMapExtManual;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::*;

use crate::application_data::{P1_PLAYER_ID, P2_PLAYER_ID};
use crate::ui::actions;
use crate::ui::{CharacterSelectScreen, MainScreen};


// SCENE NAMES
const MAIN_SCREEN_NAME: &str = "mainscreen";
const CHARACTER_SELECT_SCREEN_NAME: &str = "css";


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

        win.main_screen().p1_name_input().set_change_callback(P1_PLAYER_ID);
        win.main_screen().p2_name_input().set_change_callback(P2_PLAYER_ID);

        cmn::instantiate_score_entry(&win.main_screen().p1_score_input(), P1_PLAYER_ID);
        cmn::instantiate_score_entry(&win.main_screen().p2_score_input(), P2_PLAYER_ID);

        win
    }

    pub fn main_screen(&self) -> MainScreen {

        let correct_child: gtk4::Widget = match self.scene_switcher().child_by_name(MAIN_SCREEN_NAME) {
            Some(c) => c,
            None => {
                log::error!("Stack switcher could not find main scene with name \"{}\"!", MAIN_SCREEN_NAME);
                panic!();
            }
        };

        if !correct_child.is::<MainScreen>() {
            log::error!("The provided child with name \"{}\" cannot be cast into the MainScreen type!", MAIN_SCREEN_NAME);
            panic!();
        }

        correct_child.downcast().unwrap()
    }

    pub fn character_select_screen(&self) -> CharacterSelectScreen {
        let correct_child = match self.scene_switcher().child_by_name(CHARACTER_SELECT_SCREEN_NAME) {
            Some(c) => c,
            None => {
                log::error!("Stack switcher could not find main scene with name \"{}\"!", CHARACTER_SELECT_SCREEN_NAME);
                panic!();
            }
        };

        if !correct_child.is::<CharacterSelectScreen>() {
            log::error!("The provided child with name \"{}\" cannot be cast into the CharacterSelectScreen type!", CHARACTER_SELECT_SCREEN_NAME);
            panic!();
        }

        correct_child.downcast().unwrap()
    }

    fn instantiate_actions(&self) {

        const MAIN_WINDOW_GROUP_PREFIX: &str = "win";

        // Add in the actions that we need buttons
        // and the like to call when they're activated.

        self.add_action_entries([
            actions::create_write_data_action(),
            actions::create_initialize_character_select_data_action(),
            actions::create_switch_to_css_action(),
        ]);
        
        // Connect the action to the update button.
        self.main_screen().update_button().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::WRITE_DATA_ACTION_NAME).as_str() )
        );

        // Connect "initialize CSS" action to a button
        self.main_screen().p1_character().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SWITCH_TO_CSS_ACTION_NAME).as_str() )
        );

        self.main_screen().p2_character().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::SWITCH_TO_CSS_ACTION_NAME).as_str() )
        );

    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::MainWindow)]
    #[template(file = "main_window.ui")]
    pub struct MainWindow {

        #[template_child]
        #[property(get)]
        pub scene_switcher: TemplateChild<Stack>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {

        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = gtk4::ApplicationWindow;


        fn class_init(klass: &mut Self::Class) {
            // Ensure child types are loaded at this point
            crate::ui::MainScreen::static_type();
            crate::ui::CharacterSelectScreen::static_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }




    impl ApplicationWindowImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl WidgetImpl for MainWindow {}

    #[glib::derived_properties]
    impl ObjectImpl for MainWindow {}
}
