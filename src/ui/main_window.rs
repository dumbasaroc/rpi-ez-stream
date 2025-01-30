use gtk4::glib;
use gtk4::*;

use super::MainScreen;

glib::wrapper! {

    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Buildable;

}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        glib::Object::builder().property("application", app).build()
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
