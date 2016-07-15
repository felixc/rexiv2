// Copyright © 2016 Felix A. Crux <felixc@felixcrux.com>
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

extern crate rexiv2;

use std::path::Path;


#[test]
fn new_from_str_path() {
    let sample_path = concat!(env!("CARGO_MANIFEST_DIR"), "/tst/sample.png");
    let meta = rexiv2::Metadata::new_from_path(sample_path).unwrap();
    assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Png);
}

#[test]
fn new_from_path() {
    let sample_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tst/sample.png"));
    let meta = rexiv2::Metadata::new_from_path(sample_path).unwrap();
    assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Png);
}

#[test]
fn supports_exif() {
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    assert_eq!(meta.supports_exif(), true);
}

#[test]
fn supports_iptc() {
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    assert_eq!(meta.supports_iptc(), true);
}

#[test]
fn supports_xmp() {
    let meta = rexiv2::Metadata::new_from_buffer(include_bytes!("sample.png")).unwrap();
    assert_eq!(meta.supports_xmp(), true);
}
