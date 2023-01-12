// SPDX-FileCopyrightText: 2015–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
// SPDX-License-Identifier: GPL-3.0-or-later

extern crate rexiv2;

use rexiv2::Metadata;

fn main() {
    rexiv2::initialize().expect("Unable to initialize rexiv2");

    if let Ok(meta) = Metadata::new_from_path("examples/example.jpg") {
        if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
            println!("Time: {time:?}");
        }
        if meta
            .set_tag_string("Exif.Image.DateTime", "2008:11:01 21:15:07")
            .is_ok()
        {
            meta.save_to_file("examples/example.jpg")
                .expect("Couldn't save metadata to file");
        }
    }
}
