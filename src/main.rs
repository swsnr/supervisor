// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Keep an eye on your Linux system.
//!
//! Supervisor is a GNOME application which keeps an eye on some basic things on your
//! Linux system.

#![deny(warnings, missing_docs, clippy::all)]
#![forbid(unsafe_code)]

use gtk::glib;
use gtk::prelude::*;

mod ui;

/// The application ID.
///
/// Can be overridden at compile time with `$SUPERVISOR_APP_ID`.
const APP_ID: &str = match option_env!("SUPERVISOR_APP_ID") {
    Some(v) => v,
    None => "de.swsnr.app.Supervisor",
};

/// Activate the application.
fn activate(app: &adw::Application) {
    relm4_icons::initialize_icons();

    let window = ui::ApplicationWindow::new(app);
    window.present();
}

/// Main entry point.
fn main() -> glib::ExitCode {
    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(activate);

    // Run the application
    app.run()
}
