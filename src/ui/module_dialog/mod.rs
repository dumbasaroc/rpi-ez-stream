mod layout;

use gtk4::glib;
use gtk4::prelude::*;

use crate::application_data::ModuleHandlerAPI;


glib::wrapper! {

    pub struct ModuleSelector(ObjectSubclass<imp::ModuleSelector>)
        @extends gtk4::Window, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget,
                    gtk4::Native, gtk4::Root, gtk4::ShortcutManager;

}

// glib::wrapper! {

//     pub struct ModuleSelectorActionGroup(ObjectSubclass<group_imp::ModuleSelectorActionGroup>)
//         @implements gio::ActionGroup, gio::ActionMap;
// }

// impl Default for ModuleSelectorActionGroup {
//     fn default() -> Self {
//         glib::Object::builder().build()
//     }
// }

impl ModuleSelector {

    pub fn new(parent_win: &crate::ui::MainWindow) -> Self {
        let mw: ModuleSelector = glib::Object::builder().build();

        mw.set_parent_window(parent_win);

        let modules = ModuleHandlerAPI::list_modules_in_folder()
            .expect("Failed to list modules in module folder.");

        let none_checkbox = gtk4::CheckButton::builder()
            .label("No Module")
            .build();

        if !ModuleHandlerAPI::is_module_loaded() {
            // set stuff to none by default
            none_checkbox.set_active(false);
            mw.set_new_mod_path(None::<String>);
        }

        none_checkbox.connect_toggled(glib::clone!(
            #[weak] mw,
            move |but| {
                if but.is_active() {
                    mw.set_new_mod_path(None::<String>);
                }
            }
        ));

        for (path, display) in modules {
            let checkbutton = gtk4::CheckButton::builder()
                .label(display.as_str())
                .group(&none_checkbox)
                .build();

            if let Some(mod_path) = ModuleHandlerAPI::get_module_path() {
                if path == mod_path {
                    checkbutton.set_active(true);
                    mw.set_new_mod_path(Some(path.to_str().unwrap()));
                }
            }

            checkbutton.connect_toggled(glib::clone!(
                #[strong] path,
                #[weak] mw,
                move |but| {
                    if but.is_active() {
                        mw.set_new_mod_path(Some(path.to_str().unwrap()));
                    }
                }
            ));

            mw.radio_button_box().insert_child_after(&checkbutton, mw.radio_button_box().last_child().as_ref());
        }

        mw.radio_button_box().insert_child_after(&none_checkbox, mw.radio_button_box().last_child().as_ref());


        // Set button callbacks
        mw.cancel_button().set_action_name(Some("window.close"));
        
        mw.confirm_button().connect_clicked(glib::clone!(
            #[weak(rename_to = win)]
            mw,
            move |but| {
                ModuleSelector::confirm_button_callback(but, &win);
            }
        ));

        mw
    }

    pub fn radio_button_box(&self) -> gtk4::Box {
        self.layout().modules_list()
    }

    pub fn cancel_button(&self) -> gtk4::Button {
        self.layout().cancel_button()
    }

    pub fn confirm_button(&self) -> gtk4::Button {
        self.layout().confirm_button()
    }

    
    // Callbacks

    fn confirm_button_callback(conf: &gtk4::Button, win: &Self) {
        match ModuleHandlerAPI::load_module(&win.parent_window(), win.new_mod_path()) {
            Ok(()) => {},
            Err(e) => {
                log::error!("Failed to switch modules: {}", e);
            }
        };
        let _ = conf.activate_action("window.close", None);
    }
}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::ui::MainWindow;


    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::ModuleSelector)]
    #[template(resource = "/edu/rpi/ezstream/ui/module_dialog/module_dialog.ui")]
    pub struct ModuleSelector {

        #[template_child]
        #[property(get)]
        pub layout: TemplateChild<super::layout::ModuleSelectorLayout>,

        #[property(get, set)]
        pub parent_window: Rc<RefCell<MainWindow>>,

        #[property(get, set, nullable)]
        pub new_mod_path: Rc<RefCell<Option<String>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModuleSelector {

        const NAME: &'static str = "ModuleSelector";
        type Type = super::ModuleSelector;
        type ParentType = gtk4::Window;


        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WindowImpl for ModuleSelector {}
    impl WidgetImpl for ModuleSelector {}

    #[glib::derived_properties]
    impl ObjectImpl for ModuleSelector {}
}