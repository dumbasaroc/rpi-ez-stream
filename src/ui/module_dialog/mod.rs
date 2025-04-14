use gtk4::gio::prelude::ActionMapExtManual;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::window::WindowImpl;
use gtk4::*;


/// The internal name for the MainScreen object in
/// the GTK hierarchy
const MAIN_SCREEN_NAME: &str = "mainscreen";

/// The internal name for the CharacterSelectScreen object in
/// the GTK hierarchy
const CHARACTER_SELECT_SCREEN_NAME: &str = "css";


glib::wrapper! {

    pub struct ModuleSelector(ObjectSubclass<imp::ModuleSelector>)
        @extends gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Buildable;

}

impl ModuleSelector {

    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    #[derive(Default, CompositeTemplate)]
    #[template(file = "module_dialog.ui")]
    pub struct ModuleSelector {

        #[template_child]
        pub layout: TemplateChild<gtk4::Box>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModuleSelector {

        const NAME: &'static str = "ModuleSelector";
        type Type = super::ModuleSelector;
        type ParentType = gtk4::Window;


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

    impl WindowImpl for ModuleSelector {}
    impl WidgetImpl for ModuleSelector {}
    impl ObjectImpl for ModuleSelector {}
}