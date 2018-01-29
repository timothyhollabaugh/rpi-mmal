
use rpi_mmal_sys::*;

trait Paramater {
    fn get_header() -> *mut MMAL_PARAMETER_HEADER_T;
}
