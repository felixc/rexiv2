// example image from http://www.gaia-gis.it/spatialite-2.3.1/resources.html

extern crate rexiv2;
extern crate chrono;

use rexiv2::Metadata;
use chrono::NaiveDateTime;

fn main() {
    if let Ok(meta) = Metadata::new_from_path("examples/DSCN0010.jpg") {
        if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
            if let Ok(timestamp) = NaiveDateTime::parse_from_str(&time, "%Y:%m:%d %H:%M:%S") {
                println!("Time: {:?}", timestamp);
            }
        }
        if meta.set_tag_string("Exif.Image.DateTime", "2008:11:01 21:15:07").is_ok() {
            meta.save_to_file("examples/DSCN0010.jpg").expect("Couldn't save metadata to file");
        }
    }
}