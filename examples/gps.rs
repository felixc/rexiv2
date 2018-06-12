// example image from https://github.com/drewnoakes/metadata-extractor-images

extern crate rexiv2;

use rexiv2::Metadata;

fn main() {
    if let Ok(meta) = Metadata::new_from_path("examples/FujiFilm FinePixS1Pro (1).jpg") {
        if let Some(location) = meta.get_gps_info() {
            println!("Location: {:?}", location);
        }
    }
}
