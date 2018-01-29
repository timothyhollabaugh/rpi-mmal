extern crate rpi_mmal;

use rpi_mmal::component::Component;
//use rpi_mmal_sys::vc_gencmd_number_property;

fn main() {
    println!("Hello, world!");

    let component = Component::new(b"vc.camera_info\x00");
    //let component = Component::new(b"none\x00");
    println!("Component: {:?}", component);

}
