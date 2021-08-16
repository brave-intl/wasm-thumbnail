use std::error::Error;
use std::mem;
use std::os::raw::c_void;

use image;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::ImageOutputFormat;

mod hook;
use hook::register_panic_hook;

/// Resize the input image specified by pointer and length to nwidth by nheight,
/// returns a pointer to nsize bytes that containing a u32 length followed
/// by the thumbnail bytes and padding
#[no_mangle]
pub extern "C" fn resize_and_pad(
    pointer: *mut u8,
    length: usize,
    nwidth: u32,
    nheight: u32,
    nsize: usize,
    nquality: u8,
) -> *const u8 {
    register_panic_hook();

    let slice: &[u8] = unsafe { std::slice::from_raw_parts(pointer, length) };

    let mut out: Vec<u8> = Vec::with_capacity(nsize);
    // Reserve space at the start for length header
    out.extend_from_slice(&[0, 0, 0, 0]);

    if let Ok(thumbnail_len) = _resize_and_pad(slice, &mut out, nwidth, nheight, nsize, nquality) {
        out.splice(..4, thumbnail_len.to_be_bytes().iter().cloned());
    }

    out.resize(nsize, 0);

    let pointer = out.as_mut_ptr();
    mem::forget(out);
    pointer
}

pub fn _resize_and_pad(
    slice: &[u8],
    out: &mut Vec<u8>,
    nwidth: u32,
    nheight: u32,
    nsize: usize,
    nquality: u8,
) -> Result<u32, Box<dyn Error>> {
    let img = image::load_from_memory(slice)?;

    // Resize preserves aspect ratio
    let img = img.resize(nwidth, nheight, FilterType::Lanczos3);

    // Copy pixels only
    let mut result = DynamicImage::new_rgba8(img.width(), img.height());
    result.copy_from(&img, 0, 0)?;

    result.write_to(out, ImageOutputFormat::Jpeg(nquality))?;

    if out.len() > nsize {
        return Err("size is too large".into());
    }

    Ok(out.len() as u32 - 4)
}

/// Allocate a new buffer in the wasm memory space
#[no_mangle]
pub extern "C" fn allocate(capacity: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(capacity);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

/// Deallocate a buffer in the wasm memory space
#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}
