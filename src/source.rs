use std::ffi::{CString, NulError};
use std::ptr;

use super::ffi::{self, c_uint};
use super::types;

#[derive(Debug)]
pub struct Source {
    ptr: *mut ffi::Source,
    hop_size: usize,
}

#[derive(Debug)]
pub enum SourceError {
    NulInUri(NulError),
    BadSource,
}

impl Source {
    pub fn open(uri: &str, sample_rate: usize, hop_size: usize) -> Result<Self, SourceError> {
        let uri_cstr = CString::new(uri).map_err(SourceError::NulInUri)?;

        let ptr = unsafe {
            ffi::new_aubio_source(uri_cstr.as_ptr(),
                ffi::uint(sample_rate),
                ffi::uint(hop_size))
        };

        if ptr == ptr::null_mut() {
            Err(SourceError::BadSource)
        } else {
            Ok(Source { ptr, hop_size })
        }
    }

    pub fn read_into(&mut self, samples: &mut Vec<types::Sample>) {
        if samples.capacity() < self.hop_size {
            let needed = self.hop_size - samples.len();
            samples.reserve_exact(needed);
        }

        unsafe {
            let mut read: c_uint = 0;

            {
                let mut fvec = ffi::fvec_mut(samples);
                ffi::aubio_source_do(self.ptr, &mut fvec as *mut ffi::FVecMut, &mut read as *mut c_uint);
            }

            samples.set_len(read as usize);
        }
    }

    pub fn read(&mut self) -> Option<Vec<types::Sample>> {
        let mut samples = Vec::new();
        self.read_into(&mut samples);

        if samples.len() == 0 {
            None
        } else {
            Some(samples)
        }
    }

    pub fn sample_rate(&self) -> usize {
        unsafe { ffi::aubio_source_get_samplerate(self.ptr) as usize }
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_source(self.ptr) }
    }
}
