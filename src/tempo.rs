use std::ptr;

use super::ffi::{self, c_char};
use super::types;

#[derive(Debug)]
pub struct Tempo {
    ptr: *mut ffi::Tempo,
    hop_size: usize,
}

impl Tempo {
    pub fn new(buffer_size: usize, hop_size: usize, sample_rate: usize) -> Result<Self, ()> {
        const DEFAULT: *const c_char = b"default\0" as *const u8 as *const i8;

        let ptr = unsafe {
            ffi::new_aubio_tempo(DEFAULT,
                ffi::uint(buffer_size),
                ffi::uint(hop_size),
                ffi::uint(sample_rate))
        };

        if ptr == ptr::null_mut() {
            Err(())
        } else {
            Ok(Tempo { ptr, hop_size })
        }
    }

    pub fn execute(&mut self, input_buffer: &[types::Sample]) {
        let mut tempo = vec![0f32; 2];
        let mut tempo_fvec = ffi::fvec_mut(&mut tempo);
        let input_fvec = ffi::fvec(input_buffer);

        unsafe {
            ffi::aubio_tempo_do(self.ptr,
                &input_fvec as *const ffi::FVec,
                &mut tempo_fvec as *mut ffi::FVecMut);
        }
    }

    pub fn bpm(&self) -> Option<f32> {
        let bpm = unsafe { ffi::aubio_tempo_get_bpm(self.ptr) };

        if bpm == 0.0 {
            None
        } else {
            Some(bpm as f32)
        }
    }
}

impl Drop for Tempo {
    fn drop(&mut self) {
        unsafe { ffi::del_aubio_tempo(self.ptr) }
    }
}
