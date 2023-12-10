// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Keep an eye on your Linux system.
//!
//! Argus is a GNOME application which keeps an eye on some basic things on your
//! Linux system.

#![deny(warnings, missing_docs, clippy::all)]
#![forbid(unsafe_code)]

use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "de.swsnr.Argus";

/// Activate the application.
fn activate(app: &Application) {}

/// Main entry point.
fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(activate);

    // Run the application
    app.run();
}
