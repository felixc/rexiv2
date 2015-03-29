// Copyright © 2015 Felix A. Crux <felixc@felixcrux.com>
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
//! let meta = rexiv2::Metadata::new_from_path(&file).unwrap();
//! println!("{:?}", meta.get_tag_multiple_strings(tag))
//! ```
//!
//! [gexiv2]:      https://wiki.gnome.org/Projects/gexiv2
//! [exiv2]:       http://exiv2.org/
//! [struct-meta]: struct.Metadata.html

#![crate_type = "lib"]
#![crate_name = "rexiv2"]

#![feature(core)]         // TODO: Remove once stabilized.

extern crate gexiv2_sys as gexiv2;
extern crate num;

use std::ffi;
use std::ptr;
use std::str;

/// An opaque structure that serves as a container for a media file's metadata.
pub struct Metadata {
    raw: *mut gexiv2::GExiv2Metadata
}

/// Container for the three GPS coordinates: longitude, latitude, and altitude.
#[derive(Copy, Debug)]
pub struct GpsInfo {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64
}

/// The possible data types that a tag can have.
#[derive(Copy, Debug, PartialEq, Eq)]
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
    Unknown
}

pub use gexiv2::Orientation;

impl Metadata {
    /// Load the metadata from the file found at the given path.
    ///
    /// # Examples
    /// ```no_run
    /// let path = "myphoto.jpg";
    /// let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
    /// assert_eq!(meta.get_media_type().unwrap(), "image/jpeg".to_string());
    /// ```
    pub fn new_from_path(path: &str) -> Result<Metadata, String> {
        unsafe {
            let mut err: *mut gexiv2::GError = ptr::null_mut();
            let c_str_path = ffi::CString::new(path).unwrap().as_ptr();
            let metadata = gexiv2::gexiv2_metadata_new();
            let ok = gexiv2::gexiv2_metadata_open_path(metadata, c_str_path, &mut err);
            if !ok {
                let err_msg = str::from_utf8(ffi::CStr::from_ptr((*err).message).to_bytes());
                match err_msg {
                    Ok(v) => { return Err(v.to_string()); }
                    Err(_) => { return Err("Unknown error".to_string()); }
                }
            }
            Ok(Metadata { raw: metadata })
        }
    }

    /// Load the metadata from the given data buffer.
    ///
    /// # Examples
    /// ```
    /// # extern crate rexiv2;
    /// extern crate rustc_serialize;
    /// use rustc_serialize::hex::FromHex;
    ///
    /// # fn main() {
    /// let minipng = "89504e470d0a1a0a0000000d49484452000000010000000108000000\
    ///                003a7e9b550000000a4944415408d763f80f00010101001bb6ee5600\
    ///                00000049454e44ae426082".from_hex().unwrap();
    /// let meta = rexiv2::Metadata::new_from_buffer(&minipng).unwrap();
    /// assert_eq!(meta.get_media_type().unwrap(), "image/png".to_string());
    /// # }
    /// ```
    pub fn new_from_buffer(data: &[u8]) -> Result<Metadata, String> {
        unsafe {
            let mut err: *mut gexiv2::GError = ptr::null_mut();
            let metadata = gexiv2::gexiv2_metadata_new();
            let ok = gexiv2::gexiv2_metadata_open_buf(
                metadata, data.as_ptr(), data.len() as i64, &mut err);
            if !ok {
                let err_msg = str::from_utf8(ffi::CStr::from_ptr((*err).message).to_bytes());
                match err_msg {
                    Ok(v) => { return Err(v.to_string()); }
                    Err(_) => { return Err("Unknown error".to_string()); }
                }
            }
            Ok(Metadata { raw: metadata })
        }
    }

    /// Save metadata to the file found at the given path, which must already exist.
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        unsafe {
            let mut err: *mut gexiv2::GError = ptr::null_mut();
            let c_str_path = ffi::CString::new(path).unwrap().as_ptr();
            let ok = gexiv2::gexiv2_metadata_save_file(
                self.raw, c_str_path, &mut err);
            if !ok {
                let err_msg = str::from_utf8(ffi::CStr::from_ptr((*err).message).to_bytes());
                match err_msg {
                    Ok(v) => { return Err(v.to_string()); }
                    Err(_) => { return Err("Unknown error".to_string()); }
                }
            }
            Ok(())
        }
    }


    //
    // Image information.
    //

    /// Determine whether the type of file loaded supports Exif metadata.
    pub fn supports_exif(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_get_supports_exif(self.raw) }
    }

    /// Determine whether the type of file loaded supports IPTC metadata.
    pub fn supports_iptc(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_get_supports_iptc(self.raw) }
    }

    /// Determine whether the type of file loaded supports XMP metadata.
    pub fn supports_xmp(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_get_supports_xmp(self.raw) }
    }

    /// Return the Internet Media Type of the loaded file.
    pub fn get_media_type(&self) -> Result<String, str::Utf8Error> {
        // TODO: Return an enum?
        unsafe {
            let c_str_mime = gexiv2::gexiv2_metadata_get_mime_type(self.raw);
            let mime = str::from_utf8(ffi::CStr::from_ptr(c_str_mime).to_bytes());
            match mime {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(e)
            }
        }
    }

    /// Get the actual un-rotated/un-oriented pixel width of the loaded image.
    ///
    /// Note that this may be different from the values reported by some metadata tags
    /// that take into account the intended orientation of the image.
    pub fn get_pixel_width(&self) -> i32 {
        unsafe { gexiv2::gexiv2_metadata_get_pixel_width(self.raw) }
    }

    /// Get the actual un-rotated/un-oriented pixel height of the loaded image.
    ///
    /// Note that this may be different from the values reported by some metadata tags
    /// that take into account the intended orientation of the image.
    pub fn get_pixel_height(&self) -> i32 {
        unsafe { gexiv2::gexiv2_metadata_get_pixel_height(self.raw) }
    }


    //
    // Tag management.
    //

    /// Indicates whether the given tag is present/populated in the loaded metadata.
    pub fn has_tag(&self, tag: &str) -> bool {
        let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
        unsafe { gexiv2::gexiv2_metadata_has_tag(self.raw, c_str_tag) }
    }

    /// Removes the tag from the metadata if it exists. Returns whether it was there originally.
    pub fn clear_tag(&self, tag: &str) -> bool {
        let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
        unsafe { gexiv2::gexiv2_metadata_clear_tag(self.raw, c_str_tag) }
    }

    /// Remove all tag values from the metadata.
    pub fn clear(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear(self.raw) }
    }

    /// Indicates whether the loaded file contains any Exif metadata.
    pub fn has_exif(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_has_exif(self.raw) }
    }

    /// Removes all Exif metadata.
    pub fn clear_exif(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear_exif(self.raw) }
    }

    /// List all Exif tags present in the loaded metadata.
    pub fn get_exif_tags(&self) -> Result<Vec<String>, str::Utf8Error> {
        unsafe {
            let mut tags = vec![];
            let c_tags = gexiv2::gexiv2_metadata_get_exif_tags(self.raw);
            let mut cur_offset = 0;
            while !(*c_tags.offset(cur_offset)).is_null() {
                let tag = str::from_utf8(
                    ffi::CStr::from_ptr((*c_tags.offset(cur_offset))).to_bytes());
                match tag {
                    Ok(v) => { tags.push(v.to_string()); }
                    Err(e) => { return Err(e); }
                }
                cur_offset += 1;
            }
            Ok(tags)
        }
    }

    /// Indicates whether the loaded file contains any XMP metadata.
    pub fn has_xmp(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_has_xmp(self.raw) }
    }

    /// Removes all XMP metadata.
    pub fn clear_xmp(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear_xmp(self.raw) }
    }

    /// List all XMP tags present in the loaded metadata.
    pub fn get_xmp_tags(&self) -> Result<Vec<String>, str::Utf8Error> {
        unsafe {
            let mut tags = vec![];
            let c_tags = gexiv2::gexiv2_metadata_get_xmp_tags(self.raw);
            let mut cur_offset = 0;
            while !(*c_tags.offset(cur_offset) as u8 == 0) {
                let tag = str::from_utf8(
                    ffi::CStr::from_ptr((*c_tags.offset(cur_offset))).to_bytes());
                match tag {
                    Ok(v) => { tags.push(v.to_string()); }
                    Err(e) => { return Err(e); }
                }
                cur_offset += 1;
            }
            Ok(tags)
        }
    }

    /// Indicates whether the loaded file contains any IPTC metadata.
    pub fn has_iptc(&self) -> bool {
        unsafe { gexiv2::gexiv2_metadata_has_iptc(self.raw) }
    }

    /// Removes all XMP metadata.
    pub fn clear_iptc(&self) {
        unsafe { gexiv2::gexiv2_metadata_clear_iptc(self.raw) }
    }

    /// List all IPTC tags present in the loaded metadata.
    pub fn get_iptc_tags(&self) -> Result<Vec<String>, str::Utf8Error> {
        unsafe {
            let mut tags = vec![];
            let c_tags = gexiv2::gexiv2_metadata_get_iptc_tags(self.raw);
            let mut cur_offset = 0;
            while !(*c_tags.offset(cur_offset) as u8 == 0) {
                let tag = str::from_utf8(
                    ffi::CStr::from_ptr((*c_tags.offset(cur_offset))).to_bytes());
                match tag {
                    Ok(v) => { tags.push(v.to_string()); }
                    Err(e) => { return Err(e); }
                }
                cur_offset += 1;
            }
            Ok(tags)
        }
    }

    /// Get the value of a tag as a string.
    ///
    /// Only safe if the tag is really of a string type.
    pub fn get_tag_string(&self, tag: &str) -> Result<String, str::Utf8Error> {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            let c_str_val = gexiv2::gexiv2_metadata_get_tag_string(self.raw, c_str_tag);
            let value = str::from_utf8(ffi::CStr::from_ptr(c_str_val).to_bytes());
            match value {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(e)
            }
        }
    }

    /// Set the value of a tag to the given string.
    ///
    /// Only safe if the tag is really of a string type.
    pub fn set_tag_string(&self, tag: &str, value: &str) -> bool {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            let c_str_val = ffi::CString::new(value).unwrap().as_ptr();
            gexiv2::gexiv2_metadata_set_tag_string(self.raw, c_str_tag, c_str_val)
        }
    }

    /// Get the value of a tag as a string, potentially formatted for user-visible display.
    ///
    /// Only safe if the tag is really of a string type.
    pub fn get_tag_interpreted_string(&self, tag: &str) -> Result<String, str::Utf8Error> {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            let c_str_val = gexiv2::gexiv2_metadata_get_tag_interpreted_string(self.raw, c_str_tag);
            let value = str::from_utf8(ffi::CStr::from_ptr(c_str_val).to_bytes());
            match value {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(e)
            }
        }
    }

    /// Retrieve the list of string values of the given tag.
    ///
    /// Only safe if the tag is in fact of a string type.
    pub fn get_tag_multiple_strings(&self, tag: &str) -> Result<Vec<String>, str::Utf8Error> {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            let mut vals = vec![];
            let c_vals = gexiv2::gexiv2_metadata_get_tag_multiple(self.raw, c_str_tag);
            let mut cur_offset = 0;
            while !(*c_vals.offset(cur_offset) as i8 == 0) {
                let value = str::from_utf8(
                    ffi::CStr::from_ptr((*c_vals.offset(cur_offset))).to_bytes());
                match value {
                    Ok(v) => { vals.push(v.to_string()); }
                    Err(e) => { return Err(e); }
                }
                cur_offset += 1;
            }
            Ok(vals)
        }
    }

    /// Store the given strings as the values of a tag.
    #[allow(unused)]
    pub fn set_tag_multiple_strings(&self, tag: &str, values: &[&str]) -> bool {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            let c_strs: Result<Vec<_>, _> = values.iter().map(|&s| ffi::CString::new(s)).collect();
            let c_strs = c_strs.unwrap();
            let mut ptrs: Vec<_> = c_strs.iter().map(|c| c.as_ptr()).collect();
            ptrs.push(ptr::null());
            gexiv2::gexiv2_metadata_set_tag_multiple(self.raw, c_str_tag, ptrs.as_ptr())
        }
    }

    /// Get the value of a tag as a long.
    ///
    /// Only safe if the tag is really of a numeric type.
    pub fn get_tag_long(&self, tag: &str) -> i64 {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            gexiv2::gexiv2_metadata_get_tag_long(self.raw, c_str_tag)
        }
    }

    /// Set the value of a tag to the given number.
    ///
    /// Only safe if the tag is really of a numeric type.
    pub fn set_tag_long(&self, tag: &str, value: i64) -> bool {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            gexiv2::gexiv2_metadata_set_tag_long(self.raw, c_str_tag, value)
        }
    }

    /// Get the value of an Exif tag as a Rational.
    ///
    /// Only safe if the tag is in fact of a rational type.
    pub fn get_exif_tag_rational(&self, tag: &str) -> Option<num::rational::Ratio<i32>> {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            let ref mut num = 0;
            let ref mut den = 0;
            let ok = gexiv2::gexiv2_metadata_get_exif_tag_rational(self.raw, c_str_tag, num, den);
            if !ok {
                return None
            }
            Some(num::rational::Ratio::new(*num, *den))
        }
    }

    /// Set the value of an Exif tag to a Rational.
    ///
    /// Only safe if the tag is in fact of a rational type.
    pub fn set_exif_tag_rational(&self, tag: &str, value: &num::rational::Ratio<i32>) -> bool {
        unsafe {
            let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
            gexiv2::gexiv2_metadata_set_exif_tag_rational(
                self.raw, c_str_tag, *value.numer(), *value.denom())
        }
    }


    //
    // Helper & convenience getters/setters.
    //

    /// Find out the orientation the image should have, according to the metadata tag.
    pub fn get_orientation(&self) -> Orientation {
        unsafe { gexiv2::gexiv2_metadata_get_orientation(self.raw) }
    }

    /// Set the intended orientation for the image.
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe { gexiv2::gexiv2_metadata_set_orientation(self.raw, orientation) }
    }

    /// Returns the camera exposure time of the photograph.
    pub fn get_exposure_time(&self) -> Option<num::rational::Ratio<i32>> {
        unsafe {
            let ref mut num = 0;
            let ref mut den = 0;
            let ok = gexiv2::gexiv2_metadata_get_exposure_time(self.raw, num, den);
            if !ok {
                return None
            }
            Some(num::rational::Ratio::new(*num, *den))
        }
    }

    /// Returns the f-number used by the camera taking the photograph.
    pub fn get_fnumber(&self) -> Option<f64> {
        unsafe {
            let fnumber = gexiv2::gexiv2_metadata_get_fnumber(self.raw);
            if fnumber == -1.0 {
                return None;
            }
            Some(fnumber)
        }
    }

    /// Returns the focal length used by the camera taking the photograph.
    pub fn get_focal_length(&self) -> Option<f64> {
        unsafe {
            let focal = gexiv2::gexiv2_metadata_get_focal_length(self.raw);
            if focal == -1.0 {
                return None;
            }
            Some(focal)
        }
    }

    /// Returns the ISO speed used by the camera taking the photograph.
    pub fn get_iso_speed(&self) -> Option<i32> {
        unsafe {
            let speed = gexiv2::gexiv2_metadata_get_iso_speed(self.raw);
            if speed == 0 {
                return None;
            }
            Some(speed)
        }
    }


    //
    // GPS-related methods.
    //

    /// Retrieve the stored GPS information from the loaded file.
    pub fn get_gps_info(&self) -> Option<GpsInfo> {
        unsafe {
            let ref mut lon = 0.0;
            let ref mut lat = 0.0;
            let ref mut alt = 0.0;
            let ok = gexiv2::gexiv2_metadata_get_gps_info(self.raw, lon, lat, alt);
            if !ok {
                return None
            }
            Some(GpsInfo { longitude: *lon, latitude: *lat, altitude: *alt })
        }
    }

    /// Save the specified GPS values to the metadata.
    pub fn set_gps_info(&self, gps: &GpsInfo) -> bool {
        unsafe {
            gexiv2::gexiv2_metadata_set_gps_info(
                self.raw, gps.longitude, gps.latitude, gps.altitude)
        }
    }

    /// Remove all saved GPS information from the metadata.
    pub fn delete_gps_info(&self) {
        unsafe { gexiv2::gexiv2_metadata_delete_gps_info(self.raw) }
    }
}

impl Drop for Metadata {
    fn drop(&mut self) {
        unsafe { gexiv2::gexiv2_metadata_free(self.raw); }
    }
}

//
// Tag information.
//

/// Indicates whether the given tag is from the Exif domain.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::is_exif_tag("Exif.Photo.FocalLength"), true);
/// assert_eq!(rexiv2::is_exif_tag("Iptc.Application2.Subject"), false);
/// ```
pub fn is_exif_tag(tag: &str) -> bool {
    let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
    unsafe { gexiv2::gexiv2_metadata_is_exif_tag(c_str_tag) }
}

/// Indicates whether the given tag is part of the IPTC domain.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::is_iptc_tag("Iptc.Application2.Subject"), true);
/// assert_eq!(rexiv2::is_iptc_tag("Xmp.dc.Title"), false);
/// ```
pub fn is_iptc_tag(tag: &str) -> bool {
    let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
    unsafe { gexiv2::gexiv2_metadata_is_iptc_tag(c_str_tag) }
}

/// Indicates whether the given tag is from the XMP domain.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::is_xmp_tag("Xmp.dc.Title"), true);
/// assert_eq!(rexiv2::is_xmp_tag("Exif.Photo.FocalLength"), false);
/// ```
pub fn is_xmp_tag(tag: &str) -> bool {
    let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
    unsafe { gexiv2::gexiv2_metadata_is_xmp_tag(c_str_tag) }
}

/// Get a short label for a tag.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_tag_label("Iptc.Application2.Subject"), Ok("Subject".to_string()));
/// ```
pub fn get_tag_label(tag: &str) -> Result<String, str::Utf8Error> {
    unsafe {
        let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
        let c_str_label = gexiv2::gexiv2_metadata_get_tag_label(c_str_tag);
        let label = str::from_utf8(ffi::CStr::from_ptr(c_str_label).to_bytes());
        match label {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(e)
        }
    }
}

/// Get the long-form description of a tag.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_tag_description("Iptc.Application2.Subject"),
///     Ok("The Subject Reference is a structured definition of the subject matter.".to_string()))
/// ```
pub fn get_tag_description(tag: &str) -> Result<String, str::Utf8Error> {
    unsafe {
        let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
        let c_str_desc = gexiv2::gexiv2_metadata_get_tag_description(c_str_tag);
        let desc = str::from_utf8(ffi::CStr::from_ptr(c_str_desc).to_bytes());
        match desc {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(e)
        }
    }
}

/// Determine the type of the given tag.
///
/// # Examples
/// ```
/// assert_eq!(rexiv2::get_tag_type("Iptc.Application2.Subject"), Ok(rexiv2::TagType::String));
/// assert_eq!(rexiv2::get_tag_type("Iptc.Application2.DateCreated"), Ok(rexiv2::TagType::Date));
/// ```
pub fn get_tag_type(tag: &str) -> Result<TagType, str::Utf8Error> {
    let c_str_tag = ffi::CString::new(tag).unwrap().as_ptr();
    unsafe {
        let c_str_type = gexiv2::gexiv2_metadata_get_tag_type(c_str_tag);
        let tag_type = str::from_utf8(ffi::CStr::from_ptr(c_str_type).to_bytes());
        match tag_type {
            Ok(v) => match v {
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
                _ => Ok(TagType::Unknown)
            },
            Err(e) => Err(e)
        }
    }
}

//
// XMP namespace management.
//

/// Add a new XMP namespace for tags to exist under.
pub fn register_xmp_namespace(name: &str, prefix: &str) -> bool {
    let c_str_name = ffi::CString::new(name).unwrap().as_ptr();
    let c_str_prefix = ffi::CString::new(prefix).unwrap().as_ptr();
    unsafe { gexiv2::gexiv2_metadata_register_xmp_namespace(c_str_name, c_str_prefix) }
}

/// Remove an XMP namespace from the set of known ones.
pub fn unregister_xmp_namespace(name: &str) -> bool {
    let c_str_name = ffi::CString::new(name).unwrap().as_ptr();
    unsafe { gexiv2::gexiv2_metadata_unregister_xmp_namespace(c_str_name) }
}

/// Forget all known XMP namespaces.
pub fn unregister_all_xmp_namespaces() {
    unsafe { gexiv2::gexiv2_metadata_unregister_all_xmp_namespaces() }
}
