use gtk4::gio::prelude::ActionMapExtManual;
use gtk4::gio::ActionGroup;
use gtk4::gio::SimpleActionGroup;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::*;

use crate::ui::actions;

use super::MainScreen;

glib::wrapper! {

    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Buildable;

}

impl MainWindow {
    pub fn new(app: &Application) -> Self {

        const MAIN_WINDOW_GROUP_PREFIX: &str = "win";

        // Create new window
        let win: MainWindow = glib::Object::builder().property("application", app).build();

        // Add in the actions that we need buttons
        // and the like to call when they're activated.

        win.add_action_entries([
            actions::create_test_set_name_action(),
            actions::create_test_write_action()
        ]);

        // let mainwin_action_group = SimpleActionGroup::new();
        // mainwin_action_group.add_action_entries([
        //     actions::create_test_set_name_action_simp()
        // ]);

        // win.insert_action_group(MAIN_WINDOW_GROUP_PREFIX, Some(&mainwin_action_group));

        
        // Connect the action to the update button.
        win.shown_screen().update_button().set_action_name(
            Some( format!("{}.{}", MAIN_WINDOW_GROUP_PREFIX, actions::TEST_WRITE_ACTION_NAME).as_str() )
        );

        win
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
