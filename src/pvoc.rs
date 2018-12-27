use std::ptr;

use super::ffi::{self};

#[derive(Debug)]
pub struct Pvoc {
  ptr: *mut ffi::Pvoc,
}

unsafe impl Send for Pvoc {}

unsafe impl Sync for Pvoc {}

impl Pvoc {
  pub fn new(window_size: usize, hop_size: usize) -> Result<Self, ()> {
    let ptr = unsafe {
      ffi::new_aubio_pvoc(
        ffi::uint(window_size),
        ffi::uint(hop_size),
      )
    };

    if ptr == ptr::null_mut() {
      Err(())
    } else {
      Ok(Pvoc { ptr })
    }
  }
}
