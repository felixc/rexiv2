extern crate rexiv2;

use rexiv2::Metadata;

fn main() {
    if let Ok(meta) = Metadata::new_from_path("examples/example.jpg") {
        if let Some(lat) = meta.get_gps_latitude() {
            println!("Latitude: {:?}", lat);
        } else {
            println!("No latitude found.");
        }
        if let Some(lon) = meta.get_gps_longitude() {
            println!("Longitude: {:?}", lon);
        } else {
            println!("No longitude found.");
        }
        if let Some(alt) = meta.get_gps_altitude() {
            println!("Altitude: {:?}", alt);
        } else {
            println!("No altitude found.");
        }
    }
}
