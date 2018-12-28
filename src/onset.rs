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
        // const DEFAULT: *const c_char = b"default\0" as *const u8 as *const i8;
        const SPECFLUX: *const c_char = b"specflux\0" as *const u8 as *const i8;
        // ...

        let ptr = unsafe {
            ffi::new_aubio_onset(
                SPECFLUX,
                ffi::uint(buffer_size),
                ffi::uint(hop_size),
                ffi::uint(sample_rate),
            )
        };

        if ptr == ptr::null_mut() {
            Err(())
        } else {
            Ok(Onset { ptr, hop_size })
        }
    }

    pub fn execute(&mut self, input_buffer: &[types::Sample]) {
        assert!(input_buffer.len() == self.hop_size);

        let mut position = vec![0f32; 2];
        let mut tempo_fvec = ffi::fvec_mut(&mut position);
        let input_fvec = ffi::fvec(input_buffer);

        unsafe {
            ffi::aubio_onset_do(
                self.ptr,
                &input_fvec as *const ffi::FVec,
                &mut tempo_fvec as *mut ffi::FVecMut,
            );
        }
    }

    pub fn last_onset(&self) -> u32 {
        unsafe { ffi::aubio_onset_get_last(self.ptr) }
    }

    pub fn set_threshold(&self, threshold: types::Sample) {
        unsafe {
            ffi::aubio_onset_set_threshold(self.ptr, threshold);
        }
    }

    pub fn set_silence(&self, silence: types::Sample) {
        unsafe {
            ffi::aubio_onset_set_silence(self.ptr, silence);
        }
    }

    pub fn set_minioi(&mut self, minioi: types::Sample) {
        unsafe {
            ffi::aubio_onset_set_minioi_s(self.ptr, minioi);
        }
    }
}

impl Drop for Onset {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_onset(self.ptr) }
    }
}
