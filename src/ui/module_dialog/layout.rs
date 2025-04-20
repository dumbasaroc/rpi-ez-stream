use gtk4::glib;
use gtk4::prelude::*;


glib::wrapper! {

    pub struct ModuleSelectorLayout(ObjectSubclass<imp::ModuleSelectorLayout>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Orientable, gtk4::Buildable, gtk4::ConstraintTarget;

}

impl ModuleSelectorLayout {

    pub fn confirm_button(&self) -> gtk4::Button {
        self.button_box().last_child().unwrap().downcast().unwrap()
    }

    pub fn cancel_button(&self) -> gtk4::Button {
        self.button_box().first_child().unwrap().downcast().unwrap()
    }

}

mod imp {

    use gtk4::glib::Properties;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;
    use gtk4::glib;
    use gtk4::*;

    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::ModuleSelectorLayout)]
    #[template(resource = "/edu/rpi/ezstream/ui/module_dialog/layout.ui")]
    pub struct ModuleSelectorLayout {

        #[template_child]
        pub dialog_label: TemplateChild<gtk4::Label>,

        #[template_child]
        #[property(get)]
        pub modules_list: TemplateChild<gtk4::Box>,

        #[template_child]
        #[property(get)]
        pub button_box: TemplateChild<gtk4::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ModuleSelectorLayout {

        const NAME: &'static str = "ModuleSelectorLayout";
        type Type = super::ModuleSelectorLayout;
        type ParentType = gtk4::Box;


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

    impl BoxImpl for ModuleSelectorLayout {}
    impl WidgetImpl for ModuleSelectorLayout {}

    #[glib::derived_properties]
    impl ObjectImpl for ModuleSelectorLayout {}
}