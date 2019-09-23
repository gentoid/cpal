extern crate asio_sys as sys;

use crate::{
    BuildStreamError,
    DefaultFormatError,
    DeviceNameError,
    DevicesError,
    Format,
    // PauseStreamError,
    // PlayStreamError,
    SupportedFormatsError,
    StreamData,
    StreamError,
};
use traits::{
    DeviceTrait,
    HostTrait,
};

// use self::stream::{
//     build_stream_err,
//     InterleavedSample,
//     AsioSample,
//     cast_slice_mut,
//     asio_channel_slice,
//     StreamInner,
// };

// use std::sync::atomic::Ordering;

pub use self::device::{Device, Devices, SupportedInputFormats, SupportedOutputFormats};
pub use self::stream::{Stream};
use std::sync::Arc;

mod device;
mod stream;

/// The host for ASIO.
#[derive(Debug)]
pub struct Host {
    asio: Arc<sys::Asio>,
}

impl Host {
    pub fn new() -> Result<Self, crate::HostUnavailable> {
        let asio = Arc::new(sys::Asio::new());
        let host = Host { asio };
        Ok(host)
    }
}

impl HostTrait for Host {
    type Devices = Devices;
    type Device = Device;

    fn is_available() -> bool {
        true
        //unimplemented!("check how to do this using asio-sys")
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        Devices::new(self.asio.clone())
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        // ASIO has no concept of a default device, so just use the first.
        self.input_devices().ok().and_then(|mut ds| ds.next())
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        // ASIO has no concept of a default device, so just use the first.
        self.output_devices().ok().and_then(|mut ds| ds.next())
    }
}

impl DeviceTrait for Device {
    type SupportedInputFormats = SupportedInputFormats;
    type SupportedOutputFormats = SupportedOutputFormats;
    type Stream = Stream;

    fn name(&self) -> Result<String, DeviceNameError> {
        Device::name(self)
    }

    fn supported_input_formats(&self) -> Result<Self::SupportedInputFormats, SupportedFormatsError> {
        Device::supported_input_formats(self)
    }

    fn supported_output_formats(&self) -> Result<Self::SupportedOutputFormats, SupportedFormatsError> {
        Device::supported_output_formats(self)
    }

    fn default_input_format(&self) -> Result<Format, DefaultFormatError> {
        Device::default_input_format(self)
    }

    fn default_output_format(&self) -> Result<Format, DefaultFormatError> {
        Device::default_output_format(self)
    }

    // TODO: implement
    fn build_input_stream<D, E>(&self, format: &Format, data_callback: D, error_callback: E)
    -> Result<Self::Stream, BuildStreamError>
    where D: FnMut(StreamData) + Send + 'static, E: FnMut(StreamError) + Send + 'static {
        Ok(Stream::new(
            self.build_input_stream_inner(format, data_callback, error_callback)?,
        ))}

    // TODO: implement
    fn build_output_stream<D, E>(&self, format: &Format, data_callback: D, error_callback: E) 
    -> Result<Self::Stream, BuildStreamError>
    where D: FnMut(StreamData) + Send + 'static, E: FnMut(StreamError) + Send + 'static {
        Ok(Stream::new(
            self.build_output_stream_inner(format, data_callback, error_callback)?,
        ))
    }
}
