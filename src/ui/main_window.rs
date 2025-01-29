use gtk4::subclass::prelude::*;
use gtk4::glib;
use gtk4::*;

glib::wrapper! {

    pub struct MainWindow(ObjectSubclass<priv_MainWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Buildable;

}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        glib::Object::builder().property("application", app).build()
    }
}


#[allow(non_camel_case_types)]
#[derive(Default, CompositeTemplate)]
#[template(file = "source/main_window.ui")]
pub struct priv_MainWindow {

    #[template_child]
    pub imimage: TemplateChild<gtk4::Image>
}

#[glib::object_subclass]
impl ObjectSubclass for priv_MainWindow {

    const NAME: &'static str = "MainWindow";
    type Type = MainWindow;
    type ParentType = gtk4::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


impl priv_MainWindow {}


impl ApplicationWindowImpl for priv_MainWindow {}
impl WindowImpl for priv_MainWindow {}
impl WidgetImpl for priv_MainWindow {}
impl ObjectImpl for priv_MainWindow {}
