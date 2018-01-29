
use std::mem;
use std::ffi::CStr;
use rpi_mmal_sys::*;

//enum ComponentDefault {
//    AudioDecoder = MMAL_COMPONENT_DEFAULT_AUDIO_DECODER,
//    AudioRender = MMAL_COMPONENT_DEFAULT_AUDIO_RENDERER,
//    Camera = MMAL_COMPONENT_DEFAULT_CAMERA,
//    CameraInfo = MMAL_COMPONENT_DEFAULT_CAMERA_INFO,
//    Clock = MMAL_COMPONENT_DEFAULT_CLOCK,
//    ContainerReader = MMAL_COMPONENT_DEFAULT_CONTAINER_READER,
//    ContainerWriter = MMAL_COMPONENT_DEFAULT_CONTAINER_WRITER,
//    ImageDecoder = MMAL_COMPONENT_DEFAULT_IMAGE_DECODER,
//    ImageEncoder = MMAL_COMPONENT_DEFAULT_IMAGE_ENCODER,
//    Miracast = MMAL_COMPONENT_DEFAULT_MIRACAST,
//    Scheduler = MMAL_COMPONENT_DEFAULT_SCHEDULER,
//    Splitter = MMAL_COMPONENT_DEFAULT_SPLITTER,
//    VideoConverter = MMAL_COMPONENT_DEFAULT_VIDEO_CONVERTER,
//    VideoDecoder = MMAL_COMPONENT_DEFAULT_VIDEO_DECODER,
//    VideoEncoder = MMAL_COMPONENT_DEFAULT_VIDEO_ENCODER,
//    VideoInjecter = MMAL_COMPONENT_DEFAULT_VIDEO_INJECTER,
//    VideoRenderer = MMAL_COMPONENT_DEFAULT_VIDEO_RENDERER,
//    VideoSplitter = MMAL_COMPONENT_DEFAULT_VIDEO_SPLITTER,
//}


#[derive(Debug)]
pub struct Component(Box<MMAL_COMPONENT_T>);

impl Component {
    pub fn new(name: &'static [u8]) -> Result<Component, MMAL_STATUS_T> {

        let mmal_component: Box<MMAL_COMPONENT_T> = unsafe { Box::new(mem::uninitialized()) };

        // FIXME
        // Handle unwrap correctly
        // Should really find better way to get name
        let name_cstr = CStr::from_bytes_with_nul(name).unwrap();

        let (status, mmal_component) = unsafe {
            let mut mmal_component_pointer = Box::into_raw(mmal_component);
            let status = mmal_component_create(name_cstr.as_ptr(), &mut mmal_component_pointer as *mut _);

            if mmal_component_pointer.is_null() {
                return Err(status);
            }

            let mmal_component = Box::from_raw(mmal_component_pointer);

            (status, mmal_component)
        };

        println!("status: {:?}", status);

        if status == MMAL_STATUS_T::MMAL_SUCCESS {
            println!("mmal_component: {:?}", mmal_component);
            Ok(Component(mmal_component))
        } else {
            Err(status)
        }
    }
}

impl Drop for Component {
    fn drop(&mut self) {

        println!("Destroying component: {:?}", self.0);

        let status = unsafe {
            let mut mmal_component_pointer = Box::into_raw(&self.0);
            mmal_component_destroy(mmal_component_pointer)
        };

        if status != MMAL_STATUS_T::MMAL_SUCCESS {
            panic!("Destroying mmal component failed with status {:?}", status);
        }
    }
}
