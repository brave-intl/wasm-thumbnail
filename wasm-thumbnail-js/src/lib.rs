use std::error::Error;

use wasm_bindgen::prelude::*;

use image;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::ImageOutputFormat;

/// Resize the input image specified by pointer and length to nwidth by nheight,
/// returns a pointer to nsize bytes that containing a u32 length followed
/// by the thumbnail bytes and padding
#[wasm_bindgen]
pub fn resize_and_pad(
    src: &[u8],
    nwidth: u32,
    nheight: u32,
    nsize: usize,
) -> Result<Box<[u8]>, JsValue> {
    let mut out: Vec<u8> = Vec::with_capacity(nsize);
    // Reserve space at the start for length header
    out.extend_from_slice(&[0, 0, 0, 0]);

    if let Ok(thumbnail_len) = _resize_and_pad(src, &mut out, nwidth, nheight, nsize) {
        out.splice(..4, thumbnail_len.to_be_bytes().iter().cloned());
        return Ok(out.into_boxed_slice());
    } else {
        return Err(JsValue::from_str("bad"));
    }
}

pub fn _resize_and_pad(
    slice: &[u8],
    out: &mut Vec<u8>,
    nwidth: u32,
    nheight: u32,
    nsize: usize,
) -> Result<u32, Box<dyn Error>> {
    let img = image::load_from_memory(slice)?;

    // Resize preserves aspect ratio
    let img = img.resize(nwidth, nheight, FilterType::Lanczos3);

    // Copy pixels only
    let mut result = DynamicImage::new_rgba8(img.width(), img.height());
    result.copy_from(&img, 0, 0)?;

    result.write_to(out, ImageOutputFormat::Png)?;

    if out.len() > nsize {
        return Err("size is too large".into());
    }

    Ok(out.len() as u32 - 4)
}
