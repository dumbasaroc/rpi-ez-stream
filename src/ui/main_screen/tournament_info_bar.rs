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

    #[derive(Default, CompositeTemplate, glib::Properties)]
    #[template(file = "tournament_info_bar.ui")]
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

    #[object_subclass]
    impl ObjectSubclass for TournamentInfoBar {
        const NAME: &'static str = "TournamentInfoBar";
        type Type = super::TournamentInfoBar;
        type ParentType = gtk4::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
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

