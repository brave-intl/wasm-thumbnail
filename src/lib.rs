use std::mem;
use std::os::raw::c_void;

use image;
use image::imageops::FilterType;
use image::ImageOutputFormat;

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

    let mut out: Vec<u8> = Vec::with_capacity(nsize);

    // Reserve space at the start for length header
    out.extend_from_slice(&[0, 0, 0, 0]);

    result
        .write_to(&mut out, ImageOutputFormat::Jpeg(80))
        .expect("can save as jpeg");

    assert!(out.len() <= nsize);

    let thumbnail_len: u32 = out.len() as u32 - 4;
    out.splice(..4, thumbnail_len.to_be_bytes().iter().cloned());

    out.resize(nsize, 0);

    let pointer = out.as_slice().as_ptr();
    mem::forget(out);
    pointer
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
