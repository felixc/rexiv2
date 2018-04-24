extern crate rexiv2;

use rexiv2::Metadata;

fn main() {
    if let Ok(meta) = Metadata::new_from_path("examples/example.jpg") {
        if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
            println!("Time: {:?}", time);
        }
        if meta.set_tag_string("Exif.Image.DateTime", "2008:11:01 21:15:07").is_ok() {
            meta.save_to_file("examples/example.jpg").expect("Couldn't save metadata to file");
        }
    }
}
