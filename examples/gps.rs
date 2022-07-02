extern crate rexiv2;

use rexiv2::Metadata;

fn main() {
    rexiv2::initialize().expect("Unable to initialize rexiv2");

    if let Ok(meta) = Metadata::new_from_path("examples/example.jpg") {
        if let Some(location) = meta.get_gps_info() {
            println!("Location: {:?}", location);
        }
    }
}
