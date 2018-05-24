pub use std::os::raw::{c_char, c_uint};

use super::types;

pub enum Source {}

#[repr(C)]
pub struct FVec {
    len: c_uint,
    data: *mut types::Sample,
}

pub fn with_fvec<T>(slice: &mut [types::Sample], f: impl FnOnce(&mut FVec) -> T) -> T {
    let mut fvec = FVec {
        len: slice.len() as u32,
        data: slice.as_mut_ptr(),
    };

    f(&mut fvec)
}

#[link(name="aubio")]
extern "C" {
    pub fn new_aubio_source(uri: *const c_char, sample_rate: c_uint, hop_size: c_uint) -> *mut Source;
    pub fn del_aubio_source(source: *mut Source);
    pub fn aubio_source_do(source: *mut Source, fvec: *mut FVec, read: *mut c_uint);
}
