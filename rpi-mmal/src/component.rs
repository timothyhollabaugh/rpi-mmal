
use std::mem;
use std::ffi::CStr;
use rpi_mmal_sys::*;

/// The names of default MMAL components
pub enum ComponentName {
    AudioDecoder,
    AudioRender,
    Camera,
    CameraInfo,
    Clock,
    ContainerReader,
    ContainerWriter,
    ImageDecoder,
    ImageEncoder,
    Miracast,
    Scheduler,
    Splitter,
    VideoConverter,
    VideoDecoder,
    VideoEncoder,
    VideoInjecter,
    VideoRenderer,
    VideoSplitter,
    Other(&'static [u8]),
}

impl Into<&'static [u8]> for ComponentName {
    fn into(self) -> &'static [u8] {
        match self {
            ComponentName::AudioDecoder => MMAL_COMPONENT_DEFAULT_AUDIO_DECODER,
            ComponentName::AudioRender => MMAL_COMPONENT_DEFAULT_AUDIO_RENDERER,
            ComponentName::Camera => MMAL_COMPONENT_DEFAULT_CAMERA,
            ComponentName::CameraInfo => MMAL_COMPONENT_DEFAULT_CAMERA_INFO,
            ComponentName::Clock => MMAL_COMPONENT_DEFAULT_CLOCK,
            ComponentName::ContainerReader => MMAL_COMPONENT_DEFAULT_CONTAINER_READER,
            ComponentName::ContainerWriter => MMAL_COMPONENT_DEFAULT_CONTAINER_WRITER,
            ComponentName::ImageDecoder => MMAL_COMPONENT_DEFAULT_IMAGE_DECODER,
            ComponentName::ImageEncoder => MMAL_COMPONENT_DEFAULT_IMAGE_ENCODER,
            ComponentName::Miracast => MMAL_COMPONENT_DEFAULT_MIRACAST,
            ComponentName::Scheduler => MMAL_COMPONENT_DEFAULT_SCHEDULER,
            ComponentName::Splitter => MMAL_COMPONENT_DEFAULT_SPLITTER,
            ComponentName::VideoConverter => MMAL_COMPONENT_DEFAULT_VIDEO_CONVERTER,
            ComponentName::VideoDecoder => MMAL_COMPONENT_DEFAULT_VIDEO_DECODER,
            ComponentName::VideoEncoder => MMAL_COMPONENT_DEFAULT_VIDEO_ENCODER,
            ComponentName::VideoInjecter => MMAL_COMPONENT_DEFAULT_VIDEO_INJECTER,
            ComponentName::VideoRenderer => MMAL_COMPONENT_DEFAULT_VIDEO_RENDERER,
            ComponentName::VideoSplitter => MMAL_COMPONENT_DEFAULT_VIDEO_SPLITTER,
            ComponentName::Other(name) => name,
        }
    }
}

#[derive(Debug)]
pub struct Component(Box<MMAL_COMPONENT_T>);

impl Component {

    /// Create a new component with a ComponentName
    /// The component MUST be destroyed with Component::destroy()
    /// to avoid leaking memory
    pub fn new(name: ComponentName) -> Result<Component, MMAL_STATUS_T> {

        let mmal_component: Box<MMAL_COMPONENT_T> = unsafe { Box::new(mem::uninitialized()) };

        // FIXME
        // Handle unwrap correctly
        // Should really find better way to get name
        let name_cstr = CStr::from_bytes_with_nul(name.into()).unwrap();

        let (status, mmal_component) = unsafe {
            let mut mmal_component_pointer = Box::into_raw(mmal_component);
            let status = mmal_component_create(name_cstr.as_ptr(), &mut mmal_component_pointer as *mut _);

            if mmal_component_pointer.is_null() {
                return Err(status);
            }

            let mmal_component = Box::from_raw(mmal_component_pointer);

            (status, mmal_component)
        };

        //println!("status: {:?}", status);

        if status == MMAL_STATUS_T::MMAL_SUCCESS {
            //println!("mmal_component: {:?}", mmal_component);
            Ok(Component(mmal_component))
        } else {
            Err(status)
        }
    }

    /// Destroy the component by calling mmal_component_destroy
    /// This must be called before the program ends to avoid
    /// leaking memory
    ///
    /// This really should be in a drop trait, but the drop trait
    /// takes an &mut self, and Box::into_raw takes ownership.
    pub fn destroy(self) {
        //println!("Destroying component: {:?}", &self.0);

        let status = unsafe {
            let mut mmal_component_pointer = Box::into_raw(self.0);
            mmal_component_destroy(mmal_component_pointer)
        };

        if status != MMAL_STATUS_T::MMAL_SUCCESS {
            panic!("Destroying mmal component failed with status {:?}", status);
        }
    }
}

