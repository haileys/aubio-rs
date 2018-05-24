use std::ffi::{CString, NulError};
use std::ptr;

use super::ffi::{self, c_uint};
use super::types;

pub struct Source {
    ptr: *mut ffi::Source,
    hop_size: usize,
}

pub enum SourceError {
    NulInUri(NulError),
    BadSource,
}

impl Source {
    pub fn new(uri: &str, sample_rate: c_uint, hop_size: c_uint) -> Result<Self, SourceError> {
        let uri_cstr = CString::new(uri).map_err(SourceError::NulInUri)?;

        let ptr = unsafe { ffi::new_aubio_source(uri_cstr.as_ptr(), sample_rate, hop_size) };

        if ptr == ptr::null_mut() {
            Err(SourceError::BadSource)
        } else {
            Ok(Source { ptr, hop_size: hop_size as usize })
        }
    }

    pub fn read_into(&mut self, samples: &mut Vec<types::Sample>) {
        if samples.capacity() < self.hop_size {
            let needed = self.hop_size - samples.len();
            samples.reserve_exact(needed);
        }

        unsafe {
            let mut read: c_uint = 0;

            ffi::with_fvec(samples, |fvec| {
                ffi::aubio_source_do(self.ptr, fvec as *mut ffi::FVec, &mut read as *mut c_uint);
            });

            samples.set_len(read as usize);
        }
    }

    pub fn read(&mut self) -> Vec<types::Sample> {
        let mut samples = Vec::new();
        self.read_into(&mut samples);
        samples
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_source(self.ptr) }
    }
}
