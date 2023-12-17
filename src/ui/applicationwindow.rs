// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use gtk::glib;

mod imp {
    use adw::subclass::prelude::*;
    use gtk::glib::{self, subclass::InitializingObject};

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/ui/ApplicationWindow.blp")]
    pub struct ApplicationWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationWindow {
        const NAME: &'static str = "ApplicationWindow";

        type Type = super::ApplicationWindow;

        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ApplicationWindow {}

    // Trait shared by all windows
    impl WindowImpl for ApplicationWindow {}

    impl ApplicationWindowImpl for ApplicationWindow {}

    impl AdwApplicationWindowImpl for ApplicationWindow {}
}

glib::wrapper! {
    pub struct ApplicationWindow(ObjectSubclass<imp::ApplicationWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget;
}

impl ApplicationWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
