use std::ptr;

use super::ffi;
use super::types;

#[derive(Debug)]
pub struct Pvoc {
  ptr: *mut ffi::Pvoc,
  hop_size: usize,
}

unsafe impl Send for Pvoc {}

unsafe impl Sync for Pvoc {}

impl Pvoc {
  pub fn new(window_size: usize, hop_size: usize) -> Result<Self, ()> {
    let ptr = unsafe { ffi::new_aubio_pvoc(ffi::uint(window_size), ffi::uint(hop_size)) };

    if ptr == ptr::null_mut() {
      Err(())
    } else {
      Ok(Pvoc { ptr, hop_size })
    }
  }

  // from signal
  pub fn from_signal(
    &mut self,
    input_buffer: &[types::Sample],
    mut norm: &mut Vec<f32>,
    mut phas: &mut Vec<f32>,
  ) {
    assert!(input_buffer.len() == self.hop_size);
    assert!(norm.len() == self.hop_size);
    assert!(phas.len() == self.hop_size);

    // convert input buffer
    let input_fvec = ffi::fvec(input_buffer);

    // create complex output
    let mut fftgrain = ffi::cvec_mut(&mut norm, &mut phas);

    unsafe {
      ffi::aubio_pvoc_do(
        self.ptr,
        &input_fvec as *const ffi::FVec,
        &mut fftgrain as *mut ffi::CVecMut,
      );
    }
  }

  pub fn to_signal(
    &mut self,
    norm: &[types::Sample],
    phas: &[types::Sample],
    output_buffer: &mut Vec<f32>,
  ) {
    assert!(output_buffer.len() == self.hop_size);
    assert!(norm.len() == self.hop_size);
    assert!(phas.len() == self.hop_size);

    // convert output buffer
    let mut output_fvec = ffi::fvec_mut(output_buffer);

    // create complex input
    let fftgrain = ffi::cvec(norm, phas);

    unsafe {
      ffi::aubio_pvoc_rdo(
        self.ptr,
        &fftgrain as *const ffi::CVec,
        &mut output_fvec as *mut ffi::FVecMut,
      );
    }
  }  
}

impl Drop for Pvoc {
  fn drop(&mut self) {
    unsafe { ffi::del_aubio_pvoc(self.ptr) }
  }
}
