// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Keep an eye on your Linux system.
//!
//! Supervisor is a GNOME application which keeps an eye on some basic things on your
//! Linux system.

#![deny(warnings, missing_docs, clippy::all)]
#![forbid(unsafe_code)]

use gtk::prelude::*;
use gtk::{gio, glib};

mod ui;

/// The application ID.
///
/// Can be overridden at compile time with `$SUPERVISOR_APP_ID`.
const APP_ID: &str = match option_env!("SUPERVISOR_APP_ID") {
    Some(v) => v,
    None => "de.swsnr.app.Supervisor",
};

/// Load our resources.
///
/// If the app ID was overwritten, e.g. for the developer build, register the
/// original resource path for the default app ID, to make icons work even for
/// a different app ID.
fn load_resources() {
    if option_env!("SUPERVISOR_APP_ID").is_some() {
        // SAFETY: We can unwrap because AdwApplication will already have checked the display for us.
        let display = gtk::gdk::Display::default().expect("DISPLAY?");

        let theme = gtk::IconTheme::for_display(&display);
        theme.add_resource_path("/de/swsnr/app/Supervisor/icons");
    }
}

/// Activate the application.
fn activate(app: &adw::Application) {
    load_resources();

    let window = ui::ApplicationWindow::new(app);
    window.present();
}

/// Main entry point.
fn main() -> glib::ExitCode {
    gio::resources_register_include!("icons.gresource")
        .expect("Failed to register icon resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(activate);

    // Run the application
    app.run()
}
