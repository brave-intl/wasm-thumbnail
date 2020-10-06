use std::mem;
use std::os::raw::c_void;

use image;
use image::imageops::FilterType;
use image::ImageFormat;

#[no_mangle]
pub extern "C" fn resize_and_pad(
    pointer: *mut u8,
    length: usize,
    nwidth: u32,
    nheight: u32,
    nsize: usize,
) -> *const u8 {
    let slice: &[u8] = unsafe { std::slice::from_raw_parts(pointer, length) };

    let img = image::load_from_memory(slice).expect("must be a valid image");

    // Preserves aspect ratio
    let result = img.resize(nwidth, nheight, FilterType::Lanczos3);

    let mut out = Vec::with_capacity(nsize);
    result
        .write_to(&mut out, ImageFormat::Jpeg)
        .expect("can save as png");

    out.as_slice().as_ptr()
}

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}
