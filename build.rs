// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

fn main() {
    glib_build_tools::compile_resources(&["icons"], "icons/icons.gresource.xml", "icons.gresource");
}
