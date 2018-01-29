
use std::os::raw::c_char;
use std::ffi::CString;

use rpi_mmal_sys::{
    MMAL_PORT_T,
    MMAL_PORT_PRIVATE_T,
    MMAL_PORT_TYPE,
    MMAL_ES_FORMAT_T,
    MMAL_COMPONENT_T,
    MMAL_PORT_USERDATA_T,
};

struct Port(MMAL_PORT_T);

impl Default for Port {
    fn Default() -> Port {
//        let mmal_port = MMAL_PORT_T{
//            _priv: &mut MMAL_PORT_PRIVATE_T { _address: 0_u8 },
//            name: CString::new("").as_ptr(),
//
//
//        };

    }
}

