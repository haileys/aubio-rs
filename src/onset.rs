use std::ptr;

use super::ffi::{self, c_char};
use super::types;

#[derive(Debug)]
pub struct Onset {
    ptr: *mut ffi::Onset,
    hop_size: usize,
}

unsafe impl Send for Onset {}

// all non-Sync methods take &mut self:
unsafe impl Sync for Onset {}

impl Onset {
    pub fn new(buffer_size: usize, hop_size: usize, sample_rate: usize) -> Result<Self, ()> {
        const DEFAULT: *const c_char = b"default\0" as *const u8 as *const i8;

        let ptr = unsafe {
            ffi::new_aubio_onset(DEFAULT,
                ffi::uint(buffer_size),
                ffi::uint(hop_size),
                ffi::uint(sample_rate))
        };

        if ptr == ptr::null_mut() {
            Err(())
        } else {
            Ok(Onset { ptr, hop_size })
        }
    }

    pub fn get_last(&mut self) -> f32 {
        let last = unsafe { ffi::aubio_onset_get_last(self.ptr) };
        last as f32
    }

    /// input_buffer length must equal hop_size!
    pub fn execute(&mut self, input_buffer: &[types::Sample]) {
        assert!(input_buffer.len() == self.hop_size);

        let mut onset = vec![0f32; 2];
        let mut onset_fvec = ffi::fvec_mut(&mut onset);
        let input_fvec = ffi::fvec(input_buffer);

        unsafe {
            ffi::aubio_onset_do(self.ptr,
                &input_fvec as *const ffi::FVec,
                &mut onset_fvec as *mut ffi::FVecMut);
        }
    }
}

impl Drop for Onset {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_onset(self.ptr) }
    }
}
