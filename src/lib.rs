// Copyright © 2015–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
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

#![allow(clippy::needless_doctest_main)]

//! This library provides a Rust wrapper around the [gexiv2][gexiv2] library,
//! which is itself a GObject-based wrapper around the [Exiv2][exiv2] library,
//! which provides read and write access to the Exif, XMP, and IPTC metadata
//! for media files in various formats.
//!
//! Most functionality is exposed through methods on the [`Metadata`][struct-meta] type.
//!
//! ## Usage
//! A typical use of the library might look something like this:
//!
//! ```no_run
//! let file = "myimage.jpg";
//! let tag = "Iptc.Application2.Keywords";
//! let meta = rexiv2::Metadata::new_from_path(&file)?;
//! println!("{:?}", meta.get_tag_multiple_strings(tag));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! [gexiv2]:      https://wiki.gnome.org/Projects/gexiv2
//! [exiv2]:       http://exiv2.org/
//! [struct-meta]: struct.Metadata.html

#![crate_type = "lib"]
#![crate_name = "rexiv2"]

extern crate gexiv2_sys as gexiv2;
pub use gexiv2::GExiv2LogLevel as LogLevel;

use std::ffi;
use std::ptr;
use std::str;

use std::os::unix::ffi::OsStrExt;

/// A wrapper type for the kinds of errors one might encounter when using the library.
#[derive(Debug, PartialEq, Eq)]
pub enum Rexiv2Error {
    /// No value found
    NoValue,
    /// See std::str::Utf8Error
    Utf8(str::Utf8Error),
    /// An error generated from the wrapped gexiv2 or Exiv2 libraries.
    ///
    /// May or may not contain a description message.
    Internal(Option<String>),
}

impl std::fmt::Display for Rexiv2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Rexiv2Error::NoValue => write!(f, "No value found"),
            Rexiv2Error::Utf8(ref err) => write!(f, "IO error: {err}"),
            Rexiv2Error::Internal(Some(ref msg)) => write!(f, "Internal error: {msg}"),
            Rexiv2Error::Internal(None) => write!(f, "Unknown internal error"),
        }
    }
}

impl std::error::Error for Rexiv2Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Rexiv2Error::NoValue => None,
            Rexiv2Error::Utf8(ref err) => Some(err),
            Rexiv2Error::Internal(_) => None,
        }
    }
}

impl From<str::Utf8Error> for Rexiv2Error {
    fn from(err: str::Utf8Error) -> Rexiv2Error {
        Rexiv2Error::Utf8(err)
    }
}

/// A specialized Result type that specifies the Err instances will be Rexiv2Errors.
pub type Result<T> = std::result::Result<T, Rexiv2Error>;

/// An opaque structure that serves as a container for a media file's metadata.
#[derive(Debug, PartialEq, Eq)]
pub struct Metadata {
    raw: *mut gexiv2::GExiv2Metadata,
}

/// An opaque structure that serves as a container for a preview image.
#[derive(Debug, PartialEq, Eq)]
pub struct PreviewImage<'a> {
    raw: *mut gexiv2::GExiv2PreviewProperties,
    metadata: &'a Metadata, // Parent metadata to load a PreviewImage from a PreviewProperties.
}

/// Container for the three GPS coordinates: longitude, latitude, and altitude.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpsInfo {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64,
}

/// The possible data types that a tag can have.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TagType {
    /// Exif BYTE type, 8-bit unsigned integer.
    UnsignedByte,
    /// Exif ASCII type, 8-bit byte.
    AsciiString,
    /// Exif SHORT type, 16-bit (2-byte) unsigned integer.
    UnsignedShort,
    /// Exif LONG type, 32-bit (4-byte) unsigned integer.
    UnsignedLong,
    /// Exif RATIONAL type, two LONGs: numerator and denumerator of a fraction.
    UnsignedRational,
    /// Exif SBYTE type, an 8-bit signed (twos-complement) integer.
    SignedByte,
    /// Exif UNDEFINED type, an 8-bit byte that may contain anything.
    Undefined,
    /// Exif SSHORT type, a 16-bit (2-byte) signed (twos-complement) integer.
    SignedShort,
    /// Exif SLONG type, a 32-bit (4-byte) signed (twos-complement) integer.
    SignedLong,
    /// Exif SRATIONAL type, two SLONGs: numerator and denumerator of a fraction.
    SignedRational,
    /// TIFF FLOAT type, single precision (4-byte) IEEE format.
    TiffFloat,
    /// TIFF DOUBLE type, double precision (8-byte) IEEE format.
    TiffDouble,
    /// TIFF IFD type, 32-bit (4-byte) unsigned integer.
    TiffIfd,
    /// IPTC string type.
    String,
    /// IPTC date type.
    Date,
    /// IPTC time type.
    Time,
    /// Exiv2 type for the Exif user comment.
    Comment,
    /// Exiv2 type for a CIFF directory.
    Directory,
    /// XMP text type.
    XmpText,
    /// XMP alternative type.
    XmpAlt,
    /// XMP bag type.
    XmpBag,
    /// XMP sequence type.
    XmpSeq,
    /// XMP language alternative type.
    LangAlt,
    /// Invalid type.
    Invalid,
    /// Unknown type.
    Unknown,
}

/// The media types that an image might have.
///
/// This can be easily converted to/created from an Internet Media Type string with the `::from()`
/// method, thanks to the `std::convert::From` trait.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MediaType {
    /// image/x-ms-bmp
    Bmp,
    /// image/x-canon-cr2
    CanonCr2,
    /// image/x-canon-crw
    CanonCrw,
    /// application/postscript
    Eps,
    /// image/x-fuji-raf
    FujiRaf,
    /// image/gif
    Gif,
    /// image/jp2
    Jp2,
    /// image/jpeg
    Jpeg,
    /// image/x-minolta-mrw
    MinoltaMrw,
    /// image/x-olympus-orf
    OlympusOrf,
    /// image/png
    Png,
    /// image/x-photoshop
    Psd,
    /// image/x-panasonic-rw2
    PanasonicRw2,
    /// image/targa
    Tga,
    /// image/tiff
    Tiff,
    /// Some other, unrecognized, media type, contained within.
    Other(String),
}

impl<'a> std::convert::From<&'a MediaType> for String {
    fn from(t: &MediaType) -> String {
        match *t {
            MediaType::Bmp => "image/x-ms-bmp".to_string(),
            MediaType::CanonCr2 => "image/x-canon-cr2".to_string(),
            MediaType::CanonCrw => "image/x-canon-crw".to_string(),
            MediaType::Eps => "application/postscript".to_string(),
            MediaType::FujiRaf => "image/x-fuji-raf".to_string(),
            MediaType::Gif => "image/gif".to_string(),
            MediaType::Jp2 => "image/jp2".to_string(),
            MediaType::Jpeg => "image/jpeg".to_string(),
            MediaType::MinoltaMrw => "image/x-minolta-mrw".to_string(),
            MediaType::OlympusOrf => "image/x-olympus-orf".to_string(),
            MediaType::Png => "image/png".to_string(),
            MediaType::Psd => "image/x-photoshop".to_string(),
            MediaType::PanasonicRw2 => "image/x-panasonic-rw2".to_string(),
            MediaType::Tga => "image/targa".to_string(),
            MediaType::Tiff => "image/tiff".to_string(),
            MediaType::Other(ref s) => s.clone(),
        }
    }
}

impl<'a> std::convert::From<&'a str> for MediaType {
    fn from(t: &str) -> MediaType {
        match t {
            "image/x-ms-bmp" => MediaType::Bmp,
            "image/x-canon-cr2" => MediaType::CanonCr2,
            "image/x-canon-crw" => MediaType::CanonCrw,
            "application/postscript" => MediaType::Eps,
            "image/x-fuji-raf" => MediaType::FujiRaf,
            "image/gif" => MediaType::Gif,
            "image/jp2" => MediaType::Jp2,
            "image/jpeg" => MediaType::Jpeg,
            "image/x-minolta-mrw" => MediaType::MinoltaMrw,
            "image/x-olympus-orf" => MediaType::OlympusOrf,
            "image/png" => MediaType::Png,
            "image/x-photoshop" => MediaType::Psd,
            "image/x-panasonic-rw2" => MediaType::PanasonicRw2,
            "image/targa" => MediaType::Tga,
            "image/tiff" => MediaType::Tiff,
            _ => MediaType::Other(t.to_string()),
        }
    }
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

pub use gexiv2::Orientation;

impl Metadata {
    /// Load the metadata from the file found at the given path.
    ///
    /// # Examples
    /// ```no_run
    /// let path = "myphoto.jpg";
    /// let meta = rexiv2::Metadata::new_from_path(&path)?;
    /// assert_eq!(meta.get_media_type()?, rexiv2::MediaType::Jpeg);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new_from_path<S: AsRef<ffi::OsStr>>(path: S) -> Result<Metadata> {
        let mut err: *mut gexiv2::GError = ptr::null_mut();
        let c_str_path = ffi::CString::new(path.as_ref().as_bytes()).unwrap();
        unsafe {
            let metadata = gexiv2::gexiv2_metadata_new();
            let ok = gexiv2::gexiv2_metadata_open_path(metadata, c_str_path.as_ptr(), &mut err);
            if ok != 1 {
                let err_msg = ffi::CStr::from_ptr((*err).message).to_str();
                return Err(Rexiv2Error::Internal(
                    err_msg.ok().map(|msg| msg.to_string()),
                ));
            }
            Ok(Metadata { raw: metadata })
        }
    }

    /// Load the metadata from the given Exif data buffer.
    ///
    /// This is usually the data in the JPEG APP1 segment.
    pub fn new_from_app1_segment(data: &[u8]) -> Result<Metadata> {
        let mut err: *mut gexiv2::GError = ptr::null_mut();
        unsafe {
            let metadata = gexiv2::gexiv2_metadata_new();
            let ok = gexiv2::gexiv2_metadata_from_app1_segment(
                metadata,
                data.as_ptr(),
                data.len() as libc::c_long,
                &mut err,
            );
            if ok != 1 {
                let err_msg = ffi::CStr::from_ptr((*err).message).to_str();
                return Err(Rexiv2Error::Internal(
                    err_msg.ok().map(|msg| msg.to_string()),
                ));
            }
            Ok(Metadata { raw: metadata })
        }
    }

    /// Load the metadata from the given data buffer.
    ///
    /// # Examples
    /// ```
    /// let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1,
    ///                0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65, 84,
    ///                8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73, 69,
    ///                78, 68, 174, 66, 96, 130];
    /// let meta = rexiv2::Metadata::new_from_buffer(&minipng)?;
    /// assert_eq!(meta.get_media_type()?, rexiv2::MediaType::Png);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new_from_buffer(data: &[u8]) -> Result<Metadata> {
        let mut err: *mut gexiv2::GError = ptr::null_mut();
        unsafe {
            let metadata = gexiv2::gexiv2_metadata_new();
            let ok = gexiv2::gexiv2_metadata_open_buf(
                metadata,
                data.as_ptr(),
                data.len() as libc::c_long,
                &mut err,
            );
            if ok != 1 {
                let err_msg = ffi::CStr::from_ptr((*err).message).to_str();
                return Err(Rexiv2Error::Internal(
                    err_msg.ok().map(|msg| msg.to_string()),
                ));
            }
            Ok(Metadata { raw: metadata })
        }
    }

    /// Save metadata to the file found at the given path, which must already exist.
    pub fn save_to_file<S: AsRef<ffi::OsStr>>(&self, path: S) -> Result<()> {
        let mut err: *mut gexiv2::GError = ptr::null_mut();
        let c_str_path = ffi::CString::new(path.as_ref().as_bytes()).unwrap();
        unsafe {
            let ok = gexiv2::gexiv2_metadata_save_file(self.raw, c_str_path.as_ptr(), &mut err);
            if ok != 1 {
                let err_msg = ffi::CStr::from_ptr((*err).message).to_str();
                return Err(Rexiv2Error::Internal(
                    err_msg.ok().map(|msg| msg.to_string()),
                ));
            }
            Ok(())
        }
    }


    // Image information.

    /// Determine whether the type of file loaded supports Exif metadata.
    pub fn supports_exif(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_get_supports_exif(self.raw) == 1 }
    }

    /// Determine whether the type of file loaded supports IPTC metadata.
    pub fn supports_iptc(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_get_supports_iptc(self.raw) == 1 }
    }

    /// Determine whether the type of file loaded supports XMP metadata.
    pub fn supports_xmp(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_get_supports_xmp(self.raw) == 1 }
    }

    /// Return the media type of the loaded file.
    pub fn get_media_type(&self) -> Result<MediaType> {
        unsafe {
            let c_str_val = gexiv2::gexiv2_metadata_get_mime_type(self.raw);
            if c_str_val.is_null() {
                return Err(Rexiv2Error::NoValue);
            }
            Ok(MediaType::from(ffi::CStr::from_ptr(c_str_val).to_str()?))
        }
    }

    /// Get the actual un-rotated/un-oriented pixel width of the loaded image.
    ///
    /// Note that this may be different from the values reported by some metadata tags
    /// that take into account the intended orientation of the image.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert_eq!(meta.get_pixel_width(), 1);
    /// ```
    pub fn get_pixel_width(&self) -> i32 {
        unsafe { gexiv2::gexiv2_metadata_get_pixel_width(self.raw) }
    }

    /// Get the actual un-rotated/un-oriented pixel height of the loaded image.
    ///
    /// Note that this may be different from the values reported by some metadata tags
    /// that take into account the intended orientation of the image.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert_eq!(meta.get_pixel_height(), 1);
    /// ```
    pub fn get_pixel_height(&self) -> i32 {
        unsafe { gexiv2::gexiv2_metadata_get_pixel_height(self.raw) }
    }


    // Tag management.

    /// Indicates whether the given tag is present/populated in the loaded metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert!(!meta.has_tag("Exif.Image.DateTime"));
    /// meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// assert!(meta.has_tag("Exif.Image.DateTime"));
    /// ```
    pub fn has_tag(&self, tag: &str) -> bool {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe { gexiv2::gexiv2_metadata_has_tag(self.raw, c_str_tag.as_ptr()) == 1 }
    }

    /// Removes the tag from the metadata if it exists. Returns whether it was there originally.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// assert!(meta.has_tag("Exif.Image.DateTime"));
    /// assert!(meta.clear_tag("Exif.Image.DateTime"));
    /// assert!(!meta.has_tag("Exif.Image.DateTime"));
    /// ```
    pub fn clear_tag(&self, tag: &str) -> bool {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe { gexiv2::gexiv2_metadata_clear_tag(self.raw, c_str_tag.as_ptr()) == 1 }
    }

    /// Remove all tag values from the metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// assert!(meta.has_tag("Exif.Image.DateTime"));
    /// meta.clear();
    /// assert!(!meta.has_tag("Exif.Image.DateTime"));
    /// ```
    pub fn clear(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear(self.raw) }
    }

    /// Indicates whether the loaded file contains any Exif metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert!(!meta.has_exif());
    /// meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// assert!(meta.has_exif());
    /// ```
    pub fn has_exif(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_has_exif(self.raw) == 1 }
    }

    /// Removes all Exif metadata, leaving other types of metadata intact.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// meta.set_tag_string("Xmp.dc.Title", "Test");
    /// assert!(meta.has_exif());
    /// assert!(meta.has_xmp());
    /// meta.clear_exif();
    /// assert!(!meta.has_exif());
    /// assert!(meta.has_xmp());
    /// ```
    pub fn clear_exif(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear_exif(self.raw) }
    }

    /// List all Exif tags present in the loaded metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// assert_eq!(meta.get_exif_tags(), Ok(vec!["Exif.Image.DateTime".to_string()]));
    /// ```
    pub fn get_exif_tags(&self) -> Result<Vec<String>> {
        let mut tags = vec![];
        unsafe {
            let c_tags = gexiv2::gexiv2_metadata_get_exif_tags(self.raw);
            let mut cur_offset = 0;
            while !(*c_tags.offset(cur_offset)).is_null() {
                let tag = ffi::CStr::from_ptr(*c_tags.offset(cur_offset)).to_str();
                match tag {
                    Ok(v) => tags.push(v.to_string()),
                    Err(e) => {
                        free_array_of_pointers(c_tags as *mut *mut libc::c_void);
                        return Err(Rexiv2Error::from(e));
                    }
                }
                cur_offset += 1;
            }
            free_array_of_pointers(c_tags as *mut *mut libc::c_void);
        }
        Ok(tags)
    }

    /// Indicates whether the loaded file contains any XMP metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert!(!meta.has_xmp());
    /// meta.set_tag_string("Xmp.dc.Title", "Test Image");
    /// assert!(meta.has_xmp());
    /// ```
    pub fn has_xmp(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_has_xmp(self.raw) == 1 }
    }

    /// Removes all XMP metadata, leaving all other types of metadata intact.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// meta.set_tag_string("Xmp.dc.Title", "Test Image");
    /// meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// assert!(meta.has_xmp());
    /// assert!(meta.has_exif());
    /// meta.clear_xmp();
    /// assert!(!meta.has_xmp());
    /// assert!(meta.has_exif());
    /// ```
    pub fn clear_xmp(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear_xmp(self.raw) }
    }

    /// List all XMP tags present in the loaded metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// meta.set_tag_string("Xmp.dc.Title", "Test Image");
    /// assert_eq!(meta.get_xmp_tags(), Ok(vec!["Xmp.dc.Title".to_string()]));
    /// ```
    pub fn get_xmp_tags(&self) -> Result<Vec<String>> {
        let mut tags = vec![];
        unsafe {
            let c_tags = gexiv2::gexiv2_metadata_get_xmp_tags(self.raw);
            let mut cur_offset = 0;
            while !(*c_tags.offset(cur_offset)).is_null() {
                let tag = ffi::CStr::from_ptr(*c_tags.offset(cur_offset)).to_str();
                match tag {
                    Ok(v) => tags.push(v.to_string()),
                    Err(e) => {
                        free_array_of_pointers(c_tags as *mut *mut libc::c_void);
                        return Err(Rexiv2Error::from(e));
                    }
                }
                cur_offset += 1;
            }
            free_array_of_pointers(c_tags as *mut *mut libc::c_void);
        }
        Ok(tags)
    }

    /// Indicates whether the loaded file contains any IPTC metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert!(!meta.has_iptc());
    /// meta.set_tag_string("Iptc.Application2.Subject", "Test Image");
    /// assert!(meta.has_iptc());
    /// ```
    pub fn has_iptc(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_has_iptc(self.raw) == 1 }
    }

    /// Removes all XMP metadata, leaving all other types of metadata intact.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// meta.set_tag_string("Iptc.Application2.Subject", "Test Image");
    /// meta.set_tag_string("Exif.Image.DateTime", "2022-08-07 11:19:44");
    /// assert!(meta.has_iptc());
    /// assert!(meta.has_exif());
    /// meta.clear_iptc();
    /// assert!(!meta.has_iptc());
    /// assert!(meta.has_exif());
    /// ```
    pub fn clear_iptc(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear_iptc(self.raw) }
    }

    /// List all IPTC tags present in the loaded metadata.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// meta.set_tag_string("Iptc.Application2.Subject", "Test Image");
    /// assert_eq!(meta.get_iptc_tags(), Ok(vec!["Iptc.Application2.Subject".to_string()]));
    /// ```
    pub fn get_iptc_tags(&self) -> Result<Vec<String>> {
        let mut tags = vec![];
        unsafe {
            let c_tags = gexiv2::gexiv2_metadata_get_iptc_tags(self.raw);
            let mut cur_offset = 0;
            while !(*c_tags.offset(cur_offset)).is_null() {
                let tag = ffi::CStr::from_ptr(*c_tags.offset(cur_offset)).to_str();
                match tag {
                    Ok(v) => tags.push(v.to_string()),
                    Err(e) => {
                        free_array_of_pointers(c_tags as *mut *mut libc::c_void);
                        return Err(Rexiv2Error::from(e));
                    }
                }
                cur_offset += 1;
            }
            free_array_of_pointers(c_tags as *mut *mut libc::c_void);
        }
        Ok(tags)
    }

    /// Get the value of a tag as a string.
    ///
    /// Only safe if the tag is really of a string type.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_string("Iptc.Application2.Subject", "Test Image");
    /// assert_eq!(meta.get_tag_string("Iptc.Application2.Subject"), Ok("Test Image".to_string()));
    /// ```
    pub fn get_tag_string(&self, tag: &str) -> Result<String> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe {
            let c_str_val = gexiv2::gexiv2_metadata_get_tag_string(self.raw, c_str_tag.as_ptr());
            if c_str_val.is_null() {
                return Err(Rexiv2Error::NoValue);
            }
            let value = ffi::CStr::from_ptr(c_str_val).to_str()?.to_string();
            libc::free(c_str_val as *mut libc::c_void);
            Ok(value)
        }
    }

    /// Set the value of a tag to the given string.
    ///
    /// Only safe if the tag is really of a string type.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// meta.set_tag_string("Iptc.Application2.Subject", "Test Image");
    /// assert_eq!(meta.get_tag_string("Iptc.Application2.Subject"), Ok("Test Image".to_string()));
    /// ```
    pub fn set_tag_string(&self, tag: &str, value: &str) -> Result<()> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        let c_str_val = ffi::CString::new(value).unwrap();
        unsafe {
            int_bool_to_result(gexiv2::gexiv2_metadata_set_tag_string(
                self.raw,
                c_str_tag.as_ptr(),
                c_str_val.as_ptr(),
            ))
        }
    }

    /// Get the value of a tag as a string, potentially formatted for user-visible display.
    ///
    /// Only safe if the tag is really of a string type.
    pub fn get_tag_interpreted_string(&self, tag: &str) -> Result<String> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe {
            let c_str_val =
                gexiv2::gexiv2_metadata_get_tag_interpreted_string(self.raw, c_str_tag.as_ptr());
            if c_str_val.is_null() {
                return Err(Rexiv2Error::NoValue);
            }
            let value = ffi::CStr::from_ptr(c_str_val).to_str()?.to_string();
            libc::free(c_str_val as *mut libc::c_void);
            Ok(value)
        }
    }

    /// Retrieve the list of string values of the given tag.
    ///
    /// Only safe if the tag is in fact of a string type.
    pub fn get_tag_multiple_strings(&self, tag: &str) -> Result<Vec<String>> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        let mut vals = vec![];
        unsafe {
            let c_vals = gexiv2::gexiv2_metadata_get_tag_multiple(self.raw, c_str_tag.as_ptr());
            if c_vals.is_null() {
                return Err(Rexiv2Error::NoValue);
            }
            let mut cur_offset = 0;
            while !(*c_vals.offset(cur_offset)).is_null() {
                let value = ffi::CStr::from_ptr(*c_vals.offset(cur_offset)).to_str();
                match value {
                    Ok(v) => vals.push(v.to_string()),
                    Err(e) => {
                        free_array_of_pointers(c_vals as *mut *mut libc::c_void);
                        return Err(Rexiv2Error::from(e));
                    }
                }
                cur_offset += 1;
            }
            free_array_of_pointers(c_vals as *mut *mut libc::c_void);
        }
        Ok(vals)
    }

    /// Store the given strings as the values of a tag.
    pub fn set_tag_multiple_strings(&self, tag: &str, values: &[&str]) -> Result<()> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        let c_strs: std::result::Result<Vec<_>, _> =
            values.iter().map(|&s| ffi::CString::new(s)).collect();
        let c_strs = c_strs.unwrap();
        let mut ptrs: Vec<_> = c_strs.iter().map(|c| c.as_ptr()).collect();
        ptrs.push(ptr::null());
        unsafe {
            int_bool_to_result(gexiv2::gexiv2_metadata_set_tag_multiple(
                self.raw,
                c_str_tag.as_ptr(),
                ptrs.as_mut_ptr(),
            ))
        }
    }

    /// Get the value of a tag as a number.
    ///
    /// Only safe if the tag is really of a numeric type.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_numeric("Exif.Photo.MaxApertureValue", 5);
    /// assert_eq!(meta.get_tag_numeric("Exif.Photo.MaxApertureValue"), 5);
    /// ```
    pub fn get_tag_numeric(&self, tag: &str) -> i32 {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe { gexiv2::gexiv2_metadata_get_tag_long(self.raw, c_str_tag.as_ptr()) as i32 }
    }

    /// Set the value of a tag to the given number.
    ///
    /// Only safe if the tag is really of a numeric type.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_numeric("Exif.Photo.MaxApertureValue", 5);
    /// assert_eq!(meta.get_tag_numeric("Exif.Photo.MaxApertureValue"), 5);
    /// ```
    pub fn set_tag_numeric(&self, tag: &str, value: i32) -> Result<()> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe {
            int_bool_to_result(gexiv2::gexiv2_metadata_set_tag_long(
                self.raw,
                c_str_tag.as_ptr(),
                value as libc::c_long,
            ))
        }
    }

    /// Get the value of a tag as a Rational.
    ///
    /// Only safe if the tag is in fact of a rational type.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// let ratio = num_rational::Ratio::new_raw(16, 10);
    /// # meta.set_tag_rational("Exif.Photo.MaxApertureValue", &ratio);
    /// assert_eq!(meta.get_tag_rational("Exif.Photo.MaxApertureValue"), Some(ratio));
    /// ```
    pub fn get_tag_rational(&self, tag: &str) -> Option<num_rational::Ratio<i32>> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        let num = &mut 0;
        let den = &mut 0;
        match unsafe {
            gexiv2::gexiv2_metadata_get_exif_tag_rational(self.raw, c_str_tag.as_ptr(), num, den)
        } {
            0 => None,
            _ => Some(num_rational::Ratio::new(*num, *den)),
        }
    }

    /// Set the value of a tag to a Rational.
    ///
    /// Only safe if the tag is in fact of a rational type.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// let ratio = num_rational::Ratio::new_raw(16, 10);
    /// meta.set_tag_rational("Exif.Photo.MaxApertureValue", &ratio);
    /// assert_eq!(meta.get_tag_rational("Exif.Photo.MaxApertureValue"), Some(ratio));
    /// ```
    pub fn set_tag_rational(&self, tag: &str, value: &num_rational::Ratio<i32>) -> Result<()> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe {
            int_bool_to_result(gexiv2::gexiv2_metadata_set_exif_tag_rational(
                self.raw,
                c_str_tag.as_ptr(),
                *value.numer(),
                *value.denom(),
            ))
        }
    }

    /// Get the value of a tag as raw data.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_rational("Exif.Photo.MaxApertureValue", &num_rational::Ratio::new_raw(16, 10));
    /// assert_eq!(meta.get_tag_raw("Exif.Photo.MaxApertureValue"), Ok(vec![0, 0, 0, 16, 0, 0, 0, 10]));
    /// ```
    #[cfg(feature = "raw-tag-access")]
    pub fn get_tag_raw(&self, tag: &str) -> Result<Vec<u8>> {
        let c_str_tag = ffi::CString::new(tag).unwrap();
        unsafe {
            let raw_tag_value = gexiv2::gexiv2_metadata_get_tag_raw(self.raw, c_str_tag.as_ptr());
            let size = &mut 0;
            let ptr = glib_sys::g_bytes_get_data(raw_tag_value, size) as *const u8;
            let result = if ptr.is_null() {
                Err(Rexiv2Error::NoValue)
            } else {
                // Make a copy here
                // Could be optimized out but need to keep a reference to the returned GByte object
                Ok(std::slice::from_raw_parts(ptr, *size).to_owned())
            };
            glib_sys::g_bytes_unref(raw_tag_value);
            result
        }
    }

    // Helper & convenience getters/setters.

    /// Find out the orientation the image should have, according to the metadata tag.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert_eq!(meta.get_orientation(), rexiv2::Orientation::Unspecified);
    /// ```
    pub fn get_orientation(&self) -> Orientation {
        unsafe { gexiv2::gexiv2_metadata_get_orientation(self.raw) }
    }

    /// Set the intended orientation for the image.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert_eq!(meta.get_orientation(), rexiv2::Orientation::Unspecified);
    /// meta.set_orientation(rexiv2::Orientation::VerticalFlip);
    /// assert_eq!(meta.get_orientation(), rexiv2::Orientation::VerticalFlip);
    /// ```
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe { gexiv2::gexiv2_metadata_set_orientation(self.raw, orientation) }
    }

    /// Returns the camera exposure time of the photograph.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_rational("Exif.Photo.ExposureTime", &num_rational::Ratio::new_raw(1, 1000));
    /// assert_eq!(meta.get_exposure_time(), Some(num_rational::Ratio::new_raw(1, 1000)));
    /// ```
    pub fn get_exposure_time(&self) -> Option<num_rational::Ratio<i32>> {
        let num = &mut 0;
        let den = &mut 0;
        match unsafe { gexiv2::gexiv2_metadata_get_exposure_time(self.raw, num, den) } {
            0 => None,
            _ => Some(num_rational::Ratio::new(*num, *den)),
        }
    }

    /// Returns the f-number used by the camera taking the photograph.
    pub fn get_fnumber(&self) -> Option<f64> {
        match unsafe { gexiv2::gexiv2_metadata_get_fnumber(self.raw) } {
            error_value if error_value < 0.0 => None, // gexiv2 returns -1.0 on error
            fnumber => Some(fnumber),
        }
    }

    /// Returns the focal length used by the camera taking the photograph.
    pub fn get_focal_length(&self) -> Option<f64> {
        match unsafe { gexiv2::gexiv2_metadata_get_focal_length(self.raw) } {
            error_value if error_value < 0.0 => None, // gexiv2 returns -1.0 on error
            focal => Some(focal),
        }
    }

    /// Returns the ISO speed used by the camera taking the photograph.
    ///
    /// # Examples
    /// ```
    /// # let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0,
    /// #               1, 0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65,
    /// #               84, 8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73,
    /// #               69, 78, 68, 174, 66, 96, 130];
    /// # let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// # meta.set_tag_numeric("Exif.Photo.ISOSpeedRatings", 600);
    /// assert_eq!(meta.get_iso_speed(), Some(600));
    /// ```
    pub fn get_iso_speed(&self) -> Option<i32> {
        match unsafe { gexiv2::gexiv2_metadata_get_iso_speed(self.raw) } {
            0 => None,
            speed => Some(speed),
        }
    }

    // Thumbnail related methods.

    /// Get the thumbnail stored in the EXIF data.
    pub fn get_thumbnail(&self) -> Option<&[u8]> {
        let mut data: *mut u8 = ptr::null_mut();
        let mut size: libc::c_int = 0;
        unsafe {
            match gexiv2::gexiv2_metadata_get_exif_thumbnail(self.raw, &mut data, &mut size) {
                0 => None,
                _ => Some(std::slice::from_raw_parts_mut(data, size as usize)),
            }
        }
    }

    /// Remove the thumbnail from the EXIF data.
    pub fn erase_thumbnail(&self) {
        unsafe { gexiv2::gexiv2_metadata_erase_exif_thumbnail(self.raw) }
    }

    /// Set or replace the EXIF thumbnail with the image in the file.
    pub fn set_thumbnail_from_file<S: AsRef<ffi::OsStr>>(&self, path: S) -> Result<()> {
        let mut err: *mut gexiv2::GError = ptr::null_mut();
        let c_str_path = ffi::CString::new(path.as_ref().as_bytes()).unwrap();
        unsafe {
            let ok = gexiv2::gexiv2_metadata_set_exif_thumbnail_from_file(
                self.raw,
                c_str_path.as_ptr(),
                &mut err,
            );
            if ok != 1 {
                let err_msg = ffi::CStr::from_ptr((*err).message).to_str();
                return Err(Rexiv2Error::Internal(
                    err_msg.ok().map(|msg| msg.to_string()),
                ));
            }
            Ok(())
        }
    }

    /// Set or replace the EXIF thumbnail with the content of a buffer.
    pub fn set_thumbnail_from_buffer(&self, data: &[u8]) {
        unsafe {
            gexiv2::gexiv2_metadata_set_exif_thumbnail_from_buffer(
                self.raw,
                data.as_ptr(),
                data.len() as libc::c_int,
            )
        }
    }

    // Preview image related methods.

    /// Return the all the preview images found in this EXIF data.
    pub fn get_preview_images(&self) -> Option<Vec<PreviewImage>> {
        unsafe {
            let ptr = gexiv2::gexiv2_metadata_get_preview_properties(self.raw);
            if ptr.is_null() {
                return None;
            }

            let mut previews: Vec<PreviewImage> = vec![];
            let mut n = 0;
            while !(*ptr.offset(n)).is_null() {
                let preview_prop = *ptr.offset(n);
                previews.push(PreviewImage { raw: preview_prop, metadata: self });
                n += 1;
            }
            Some(previews)
        }
    }

    // GPS-related methods.

    /// Retrieve the stored GPS information from the loaded file.
    pub fn get_gps_info(&self) -> Option<GpsInfo> {
        let lon = &mut 0.0;
        let lat = &mut 0.0;
        let alt = &mut 0.0;
        match unsafe { gexiv2::gexiv2_metadata_get_gps_info(self.raw, lon, lat, alt) } {
            0 => None,
            _ => Some(GpsInfo { longitude: *lon, latitude: *lat, altitude: *alt }),
        }
    }

    /// Save the specified GPS values to the metadata.
    pub fn set_gps_info(&self, gps: &GpsInfo) -> Result<()> {
        unsafe {
            int_bool_to_result(gexiv2::gexiv2_metadata_set_gps_info(
                self.raw,
                gps.longitude,
                gps.latitude,
                gps.altitude,
            ))
        }
    }

    /// Remove all saved GPS information from the metadata.
    pub fn delete_gps_info(&self) {
        unsafe { gexiv2::gexiv2_metadata_delete_gps_info(self.raw) }
    }
}

impl Drop for Metadata {
    fn drop(&mut self) {
        unsafe { gexiv2::gexiv2_metadata_free(self.raw) }
    }
}

impl PreviewImage<'_> {
    /// Return the size of the preview image in bytes.
    pub fn get_size(&self) -> u32 {
        unsafe { gexiv2::gexiv2_preview_properties_get_size(self.raw) }
    }

    /// Return the width of the preview image.
    pub fn get_width(&self) -> u32 {
        unsafe { gexiv2::gexiv2_preview_properties_get_width(self.raw) }
    }

    /// Return the height of the preview image.
    pub fn get_height(&self) -> u32 {
        unsafe { gexiv2::gexiv2_preview_properties_get_height(self.raw) }
    }

    /// Return the media type of the preview image.
    pub fn get_media_type(&self) -> Result<MediaType> {
        unsafe {
            let c_str_val = gexiv2::gexiv2_preview_properties_get_mime_type(self.raw);
            if c_str_val.is_null() {
                return Err(Rexiv2Error::NoValue);
            }
            Ok(MediaType::from(ffi::CStr::from_ptr(c_str_val).to_str()?))
        }
    }

    /// Return the preview image's recommended file extension.
    pub fn get_extension(&self) -> Result<String> {
        unsafe {
            let c_str_val = gexiv2::gexiv2_preview_properties_get_extension(self.raw);
            if c_str_val.is_null() {
                return Err(Rexiv2Error::NoValue);
            }
            Ok((ffi::CStr::from_ptr(c_str_val).to_str())?.to_string())
        }
    }

    /// Get the preview image data.
    pub fn get_data(&self) -> Result<Vec<u8>> {
        let image =
            unsafe { gexiv2::gexiv2_metadata_get_preview_image(self.metadata.raw, self.raw) };

        let mut size: libc::c_uint = 0;
        unsafe {
            let data = gexiv2::gexiv2_preview_image_get_data(image, &mut size);
            let result = if data.is_null() {
                Err(Rexiv2Error::NoValue)
            } else {
                Ok(std::slice::from_raw_parts_mut(data as *mut u8, size as usize).to_vec())
            };
            gexiv2::gexiv2_preview_image_free(image);
            result
        }
    }

    /// Save the preview image to a file.
    pub fn save_to_file<S: AsRef<ffi::OsStr>>(&self, path: S) -> Result<()> {
        let image =
            unsafe { gexiv2::gexiv2_metadata_get_preview_image(self.metadata.raw, self.raw) };

        let c_str_path = ffi::CString::new(path.as_ref().as_bytes()).unwrap();
        unsafe {
            let ok = gexiv2::gexiv2_preview_image_write_file(image, c_str_path.as_ptr());
            gexiv2::gexiv2_preview_image_free(image);

            let expected = self.get_size() as libc::c_long;
            if ok != expected {
                Err(Rexiv2Error::Internal(None))
            } else {
                Ok(())
            }
        }
    }
}


// Tag information.

/// Indicates whether the given tag is from the Exif domain.
///
/// # Examples
/// ```
/// assert!(rexiv2::is_exif_tag("Exif.Photo.FocalLength"));
/// assert!(!rexiv2::is_exif_tag("Iptc.Application2.Subject"));
/// ```
pub fn is_exif_tag(tag: &str) -> bool {
    let c_str_tag = ffi::CString::new(tag).unwrap();
    unsafe { gexiv2::gexiv2_metadata_is_exif_tag(c_str_tag.as_ptr()) == 1 }
}

/// Indicates whether the given tag is part of the IPTC domain.
///
/// # Examples
/// ```
/// assert!(rexiv2::is_iptc_tag("Iptc.Application2.Subject"));
/// assert!(!rexiv2::is_iptc_tag("Xmp.dc.Title"));
/// ```
pub fn is_iptc_tag(tag: &str) -> bool {
    let c_str_tag = ffi::CString::new(tag).unwrap();
    unsafe { gexiv2::gexiv2_metadata_is_iptc_tag(c_str_tag.as_ptr()) == 1 }
}

/// Indicates whether the given tag is from the XMP domain.
///
/// # Examples
/// ```
/// assert!(rexiv2::is_xmp_tag("Xmp.dc.Title"));
/// assert!(!rexiv2::is_xmp_tag("Exif.Photo.FocalLength"));
/// ```
pub fn is_xmp_tag(tag: &str) -> bool {
    let c_str_tag = ffi::CString::new(tag).unwrap();
    unsafe { gexiv2::gexiv2_metadata_is_xmp_tag(c_str_tag.as_ptr()) == 1 }
}

/// Get a short label for a tag.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_tag_label("Iptc.Application2.Subject"), Ok("Subject".to_string()));
/// ```
pub fn get_tag_label(tag: &str) -> Result<String> {
    let c_str_tag = ffi::CString::new(tag).unwrap();
    unsafe {
        let c_str_val = gexiv2::gexiv2_metadata_get_tag_label(c_str_tag.as_ptr());
        if c_str_val.is_null() {
            return Err(Rexiv2Error::NoValue);
        }
        Ok(ffi::CStr::from_ptr(c_str_val).to_str()?.to_string())
    }
}

/// Get the long-form description of a tag.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_tag_description("Iptc.Application2.Subject"),
///     Ok("The Subject Reference is a structured definition of the subject matter.".to_string()));
/// ```
pub fn get_tag_description(tag: &str) -> Result<String> {
    let c_str_tag = ffi::CString::new(tag).unwrap();
    unsafe {
        let c_str_val = gexiv2::gexiv2_metadata_get_tag_description(c_str_tag.as_ptr());
        if c_str_val.is_null() {
            return Err(Rexiv2Error::NoValue);
        }
        Ok(ffi::CStr::from_ptr(c_str_val).to_str()?.to_string())
    }
}

/// Determine the type of the given tag.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_tag_type("Iptc.Application2.Subject"), Ok(rexiv2::TagType::String));
/// assert_eq!(rexiv2::get_tag_type("Iptc.Application2.DateCreated"), Ok(rexiv2::TagType::Date));
/// ```
pub fn get_tag_type(tag: &str) -> Result<TagType> {
    let c_str_tag = ffi::CString::new(tag).unwrap();
    let tag_type = unsafe {
        let c_str_val = gexiv2::gexiv2_metadata_get_tag_type(c_str_tag.as_ptr());
        if c_str_val.is_null() {
            return Err(Rexiv2Error::NoValue);
        }
        ffi::CStr::from_ptr(c_str_val).to_str()?
    };
    match tag_type {
        "Byte" => Ok(TagType::UnsignedByte),
        "Ascii" => Ok(TagType::AsciiString),
        "Short" => Ok(TagType::UnsignedShort),
        "Long" => Ok(TagType::UnsignedLong),
        "Rational" => Ok(TagType::UnsignedRational),
        "SByte" => Ok(TagType::SignedByte),
        "Undefined" => Ok(TagType::Undefined),
        "SShort" => Ok(TagType::SignedShort),
        "SLong" => Ok(TagType::SignedLong),
        "SRational" => Ok(TagType::SignedRational),
        "Float" => Ok(TagType::TiffFloat),
        "Double" => Ok(TagType::TiffDouble),
        "Ifd" => Ok(TagType::TiffIfd),
        "String" => Ok(TagType::String),
        "Date" => Ok(TagType::Date),
        "Time" => Ok(TagType::Time),
        "Comment" => Ok(TagType::Comment),
        "Directory" => Ok(TagType::Directory),
        "XmpText" => Ok(TagType::XmpText),
        "XmpAlt" => Ok(TagType::XmpAlt),
        "XmpBag" => Ok(TagType::XmpBag),
        "XmpSeq" => Ok(TagType::XmpSeq),
        "LangAlt" => Ok(TagType::LangAlt),
        "Invalid" => Ok(TagType::Invalid),
        _ => Ok(TagType::Unknown),
    }
}

/// Initialize gexiv2.
///
/// This must be called in a thread-safe fashion before using rexiv2.
/// The library may appear to work without calling this, but some
/// features such as HEIC/BMFF will silently fail, and the underlying
/// libraries make the assumption that this will be called so it
/// is safer to do so.
///
/// Calling it first thing in the main function should ensure that
/// it is executed on the main thread and is thread safe. Do not call
/// it in a threaded or async context (such as in a tokio context).
///
/// # See also
///
/// Associated Gexiv2 source code: <https://gitlab.gnome.org/GNOME/gexiv2/-/blob/e4d65b31cd77f28ef248117e161de9d8cc31d712/gexiv2/gexiv2-startup.cpp#L14>
///
/// # Examples
///
/// It is normally sufficient to simply call the function in the usual
/// obvious way:
///
/// ```
/// fn main() {
///     rexiv2::initialize().expect("Unable to initialize rexiv2");
/// }
/// ```
///
/// However if you have a more complex multi-threaded environment, you
/// might want to ensure the function only gets set up once:
///
/// ```
/// use std::sync::Once;
///
/// fn main() {
///     static START: Once = Once::new();
///
///     START.call_once(|| unsafe {
///         rexiv2::initialize().expect("Unable to initialize rexiv2");
///     });
/// }
/// ```
pub fn initialize() -> Result<()> {
    unsafe { int_bool_to_result(gexiv2::gexiv2_initialize()) }
}


// XMP namespace management.

/// Add a new XMP namespace for tags to exist under.
///
/// It is an error to register a duplicate namespace.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::register_xmp_namespace("http://creativecommons.org/ns#/", "cc"), Ok(()));
/// // But note you can't duplicate a namespace that has already been registered:
/// assert_eq!(rexiv2::register_xmp_namespace("http://creativecommons.org/ns#/", "cc"),
///    Err(rexiv2::Rexiv2Error::Internal(None)));
/// ```
pub fn register_xmp_namespace(name: &str, prefix: &str) -> Result<()> {
    let c_str_name = ffi::CString::new(name).unwrap();
    let c_str_prefix = ffi::CString::new(prefix).unwrap();
    unsafe {
        int_bool_to_result(gexiv2::gexiv2_metadata_register_xmp_namespace(
            c_str_name.as_ptr(),
            c_str_prefix.as_ptr(),
        ))
    }
}

/// Remove an XMP namespace from the set of known ones.
///
/// It is an error to unregister a namespace that isn't registered.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::register_xmp_namespace("http://creativecommons.org/ns#/", "cc"), Ok(()));
/// assert_eq!(rexiv2::unregister_xmp_namespace("http://creativecommons.org/ns#/"), Ok(()));
/// // But note you can't unregister a namespace that has already been removed:
/// assert_eq!(rexiv2::unregister_xmp_namespace("http://creativecommons.org/ns#/"),
///    Err(rexiv2::Rexiv2Error::Internal(None)));
/// ```
pub fn unregister_xmp_namespace(name: &str) -> Result<()> {
    let c_str_name = ffi::CString::new(name).unwrap();
    unsafe {
        int_bool_to_result(gexiv2::gexiv2_metadata_unregister_xmp_namespace(
            c_str_name.as_ptr(),
        ))
    }
}

/// Forget all known XMP namespaces.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::register_xmp_namespace("http://creativecommons.org/ns#/", "cc"), Ok(()));
/// rexiv2::unregister_all_xmp_namespaces();
/// assert_eq!(rexiv2::register_xmp_namespace("http://creativecommons.org/ns#/", "cc"), Ok(()));
/// ```
pub fn unregister_all_xmp_namespaces() {
    unsafe { gexiv2::gexiv2_metadata_unregister_all_xmp_namespaces() }
}


// Logging

/// Get the GExiv2 log level.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_log_level(), rexiv2::LogLevel::WARN);
/// rexiv2::set_log_level(rexiv2::LogLevel::INFO);
/// assert_eq!(rexiv2::get_log_level(), rexiv2::LogLevel::INFO);
/// ```
pub fn get_log_level() -> LogLevel {
    unsafe { gexiv2::gexiv2_log_get_level() }
}

/// Set the GExiv2 log level.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_log_level(), rexiv2::LogLevel::WARN);
/// rexiv2::set_log_level(rexiv2::LogLevel::INFO);
/// assert_eq!(rexiv2::get_log_level(), rexiv2::LogLevel::INFO);
/// ```
pub fn set_log_level(level: LogLevel) {
    unsafe { gexiv2::gexiv2_log_set_level(level) }
}


// Private internal helpers.

/// Helper function to free an array of pointers, such as those returned by some gexiv2 functions.
fn free_array_of_pointers(list: *mut *mut libc::c_void) {
    unsafe {
        let mut idx = 0;
        while !(*list.offset(idx)).is_null() {
            libc::free(*list.offset(idx));
            idx += 1;
        }
        libc::free(list as *mut libc::c_void);
    }
}

/// Convert a success/failure integer representing a boolean into a Result.
fn int_bool_to_result(success: libc::c_int) -> Result<()> {
    match success {
        0 => Err(Rexiv2Error::Internal(None)),
        _ => Ok(()),
    }
}
