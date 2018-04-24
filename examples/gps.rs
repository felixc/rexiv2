// example image from http://www.gaia-gis.it/spatialite-2.3.1/resources.html

extern crate rexiv2;

use rexiv2::Metadata;

fn main() {
    if let Ok(meta) = Metadata::new_from_path("examples/DSCN0010.jpg") {
        if let Some(location) = meta.get_gps_info() {
            println!("Long: {} Lat: {}", location.longitude, location.latitude);
        }
    }
}