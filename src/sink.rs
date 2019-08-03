use std::ffi::{CString, NulError};
use std::ptr;

use super::ffi::{self, c_uint};
use super::types;

#[derive(Debug)]
pub struct Sink {
    ptr: *mut ffi::Sink
}

unsafe impl Send for Sink {}

// all non-Sync methods take &mut self:
unsafe impl Sync for Sink {}

#[derive(Debug)]
pub enum SinkError {
    NulInUri(NulError),
    BadSink,
}

impl Sink {
    pub fn open(uri: &str, sample_rate: usize) -> Result<Self, SinkError> {
        let uri_cstr = CString::new(uri).map_err(SinkError::NulInUri)?;

        let ptr = unsafe {
            ffi::new_aubio_sink(uri_cstr.as_ptr(),
                                ffi::uint(sample_rate))
        };

        if ptr == ptr::null_mut() {
            Err(SinkError::BadSink)
        } else {
            Ok(Sink { ptr })
        }
    }

    pub fn write(&mut self, write_data: &[types::Sample]) {
        unsafe {
            let input_fvec = ffi::fvec(write_data);
            let write: c_uint = write_data.len() as u32;
            ffi::aubio_sink_do(self.ptr, &input_fvec as *const ffi::FVec, write);
        }
    }

    pub fn sample_rate(&self) -> usize {
        unsafe { ffi::aubio_sink_get_samplerate(self.ptr) as usize }
    }
}

impl Drop for Sink {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_sink(self.ptr) }
    }
}
