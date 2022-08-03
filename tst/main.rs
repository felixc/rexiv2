// Copyright © 2016–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
//
// SPDX-FileCopyrightText: 2015–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate gexiv2_sys as gexiv2;
extern crate rexiv2;

use std::path::Path;

use std::sync::Once;

static INIT: Once = Once::new();

/// Should be called before any test runs. Will ensure that the library is initialized at most once.
fn test_setup() {
    INIT.call_once(|| rexiv2::initialize().expect("Unable to initialize rexiv2"));
}

#[test]
fn new_from_str_path() {
    test_setup();
    let sample_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tst/sample.png");
    let meta = rexiv2::Metadata::new_from_path(sample_path).unwrap();
    assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Png);
}

#[test]
fn new_from_path() {
    test_setup();
    let sample_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tst/sample.png"));
    let meta = rexiv2::Metadata::new_from_path(sample_path).unwrap();
    assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Png);
}

#[test]
fn new_from_buffer() {
    test_setup();
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Png);
}

#[test]
fn new_from_buffer_error() {
    test_setup();
    let mut bytes = include_bytes!("sample.png").to_vec();
    bytes.swap(0, 1);
    let meta_result = rexiv2::Metadata::new_from_buffer(&bytes);
    assert_eq!(
        meta_result,
        Err(rexiv2::Rexiv2Error::Internal(Some(
            "unsupported format".to_string()
        )))
    );
}

#[test]
fn supports_exif() {
    test_setup();
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    assert!(meta.supports_exif());
}

#[test]
fn supports_iptc() {
    test_setup();
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    assert!(meta.supports_iptc());
}

#[test]
fn supports_xmp() {
    test_setup();
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    assert!(meta.supports_xmp());
}

#[test]
fn supports_bmff() {
    test_setup();

    // iPhone devices use the HEIC (BMFF) file format which only works properly
    // after gexiv2 has been initialized (and the underlying libraries are the
    // right version gexiv2 v0.13.0/Exiv2 v0.27.4)
    if unsafe { gexiv2::gexiv2_get_version() } < 1300 {
        return;
    }

    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.HEIC")).unwrap();
    let gps = meta.get_gps_info().unwrap();
    assert_eq!(gps.latitude as i32, -27);
    assert_eq!(gps.longitude as i32, 114);
    let phone_model = meta.get_tag_string("Exif.Image.Model").unwrap();
    assert_eq!(phone_model, "iPhone XS");

    // This seems strange since we can read the above information
    // We may be missing a "supports" function for bmff tags, or the functions may be returning incorrectly
    assert!(!meta.supports_exif());
    assert!(!meta.supports_iptc());
    assert!(!meta.supports_xmp());
}

#[test]
fn log_levels() {
    test_setup();
    assert_eq!(rexiv2::get_log_level(), rexiv2::LogLevel::WARN);
    rexiv2::set_log_level(rexiv2::LogLevel::INFO);
    assert_eq!(rexiv2::get_log_level(), rexiv2::LogLevel::INFO);
}

#[test]
#[cfg(feature = "raw-tag-access")]
fn get_tag_raw() {
    test_setup();
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    meta.set_tag_string("Exif.Image.DateTime", "2020:07:12 11:16:35")
        .unwrap();
    assert_eq!(
        meta.get_tag_raw("Exif.Image.DateTime").unwrap(),
        b"2020:07:12 11:16:35\0"
    );
}
