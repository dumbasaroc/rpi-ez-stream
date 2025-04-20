use gtk4::glib;

glib::wrapper! {
    pub struct TournamentInfoBar(ObjectSubclass<imp::TournamentInfoBar>)
    @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Orientable, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl TournamentInfoBar {

    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

mod imp {

    use gtk4::glib::object_subclass;
    use gtk4::CompositeTemplate;
    use gtk4::glib;
    use gtk4::glib::*;
    use gtk4::glib::subclass::types::*;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;

    use crate::application_data::ApplicationStateAPI;

    #[derive(Default, CompositeTemplate, glib::Properties)]
    #[template(resource = "/edu/rpi/ezstream/ui/main_screen/tournament_info_bar.ui")]
    #[properties(wrapper_type = super::TournamentInfoBar)]
    pub struct TournamentInfoBar {

        #[property(get)]
        #[template_child]
        pub best_of_radio: TemplateChild<gtk4::CheckButton>,

        #[property(get)]
        #[template_child]
        pub first_to_radio: TemplateChild<gtk4::CheckButton>,

        #[property(get)]
        #[template_child]
        pub bo_ft_counter: TemplateChild<gtk4::SpinButton>,

        #[property(get)]
        #[template_child]
        pub bracket_position: TemplateChild<gtk4::Entry>,

        #[property(get)]
        #[template_child]
        pub tournament_name: TemplateChild<gtk4::Entry>,
    }

    #[gtk4::template_callbacks]
    impl TournamentInfoBar {

        #[template_callback]
        fn best_of_toggled_on(but: &gtk4::CheckButton) {
            if but.is_active() {
                ApplicationStateAPI::set_bestof_firstto(true);
            }
        }

        #[template_callback]
        fn first_to_toggled_on(but: &gtk4::CheckButton) {
            if but.is_active() {
                ApplicationStateAPI::set_bestof_firstto(false);
            }
        }

        #[template_callback]
        fn on_bestof_counter_change(ctr: &gtk4::SpinButton) {
            ApplicationStateAPI::set_bestof_firstto_counter(ctr.value_as_int() as u32);
        }

        #[template_callback]
        fn on_change_tournname(name: &gtk4::Entry) {
            ApplicationStateAPI::set_tournament_name(name.text());
        }

        #[template_callback]
        fn on_change_bracketpos(loc: &gtk4::Entry) {
            ApplicationStateAPI::set_bracket_location(loc.text());
        }
    }

    #[object_subclass]
    impl ObjectSubclass for TournamentInfoBar {
        const NAME: &'static str = "TournamentInfoBar";
        type Type = super::TournamentInfoBar;
        type ParentType = gtk4::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[derived_properties]
    impl ObjectImpl for TournamentInfoBar {}
    impl WidgetImpl for TournamentInfoBar {}
    impl BoxImpl for TournamentInfoBar {}

}

