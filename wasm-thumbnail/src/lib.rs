use std::error::Error;
use std::io::Cursor;
use std::mem;
use std::os::raw::c_void;

use image;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImage;
use image::ImageOutputFormat;

#[cfg(not(feature = "wasm-bindgen"))]
mod hook;
#[cfg(not(feature = "wasm-bindgen"))]
use hook::register_panic_hook;

/// Resize the input image specified by pointer and length to nwidth by nheight,
/// returns a pointer to nsize bytes that containing a u32 length followed
/// by the thumbnail bytes and padding
#[no_mangle]
#[cfg(not(feature = "wasm-bindgen"))]
pub extern "C" fn resize_and_pad(
    pointer: *mut u8,
    length: usize,
    nwidth: u32,
    nheight: u32,
    nsize: usize,
    nquality: u8,
) -> *const u8 {
    use std::io::Write;

    register_panic_hook();

    let slice: &[u8] = unsafe { std::slice::from_raw_parts(pointer, length) };

    let mut out: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(nsize));
    // Reserve space at the start for length header
    out.write_all(&[0,0,0,0]);    

    let mut mquality = nquality;

    loop {
        match _resize_and_pad(slice, &mut out, nwidth, nheight, nsize, mquality) {
            Ok(thumbnail_len) => {
                out.get_mut().splice(..4, thumbnail_len.to_be_bytes().iter().cloned());
                break;
            }
            _ => {
                mquality -= 10;
                
                // Reallocate the cursor since the previous attempt to write an image has failed
                out = Cursor::new(Vec::with_capacity(nsize));
                // Reserve space at the start for length header
                out.write_all(&[0,0,0,0]);

                if mquality <= 15 {
                    panic!("Image too large")
                }
            },
    
        }  
    }

    out.get_mut().resize(nsize, 0);

    let pointer = out.get_mut().as_mut_ptr();
    mem::forget(out);
    pointer
}

pub fn _resize_and_pad(
    slice: &[u8],
    out: &mut Cursor<Vec<u8>>,
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

    if out.get_ref().len() > nsize {
        return Err("size is too large".into());
    }

    Ok(out.get_ref().len() as u32 - 4)
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