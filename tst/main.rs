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
fn new_from_app1() {
    test_setup();
    static APP1_SEGMENT: &[u8] = &[
        255, 225, 0, 232, 69, 120, 105, 102, 0, 0, 73, 73, 42, 0, 8, 0, 0, 0, 2, 0, 50, 1, 2, 0,
        20, 0, 0, 0, 38, 0, 0, 0, 37, 136, 4, 0, 1, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 50, 48, 48,
        56, 58, 49, 49, 58, 48, 49, 32, 50, 49, 58, 49, 53, 58, 48, 55, 0, 8, 0, 0, 0, 1, 0, 4, 0,
        0, 0, 2, 0, 0, 0, 1, 0, 2, 0, 2, 0, 0, 0, 78, 0, 0, 0, 2, 0, 5, 0, 3, 0, 0, 0, 160, 0, 0,
        0, 3, 0, 2, 0, 2, 0, 0, 0, 69, 0, 0, 0, 4, 0, 5, 0, 3, 0, 0, 0, 184, 0, 0, 0, 5, 0, 1, 0,
        1, 0, 0, 0, 0, 0, 0, 0, 6, 0, 10, 0, 1, 0, 0, 0, 208, 0, 0, 0, 18, 0, 2, 0, 7, 0, 0, 0,
        216, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 64,
        66, 15, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 64, 66, 15, 0, 3, 0,
        0, 0, 1, 0, 0, 0, 87, 71, 83, 45, 56, 52, 0,
    ];
    let meta = rexiv2::Metadata::new_from_app1_segment(APP1_SEGMENT).unwrap();
    assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Jpeg);
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
fn get_tag_rational_values_are_not_reduced() {
    test_setup();
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    meta.set_tag_rational(
        "Exif.Photo.ApertureValue",
        &num_rational::Ratio::new_raw(16, 10),
    )
    .unwrap();
    let result = meta.get_tag_rational("Exif.Photo.ApertureValue").unwrap();
    assert_eq!(*result.numer(), 16);
    assert_eq!(*result.denom(), 10);
}

#[test]
fn get_exposure_time_values_are_not_reduced() {
    test_setup();
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    meta.set_tag_rational(
        "Exif.Photo.ExposureTime",
        &num_rational::Ratio::new_raw(10, 1000),
    )
    .unwrap();
    let result = meta.get_exposure_time().unwrap();
    assert_eq!(*result.numer(), 10);
    assert_eq!(*result.denom(), 1000);
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
