use wasm_bindgen::prelude::*;

use wasm_thumbnail::_resize_and_pad;

/// Resize the input image specified by pointer and length to nwidth by nheight,
/// returns a pointer to nsize bytes that containing a u32 length followed
/// by the thumbnail bytes and padding
#[wasm_bindgen]
pub fn resize_and_pad(
    src: &[u8],
    nwidth: u32,
    nheight: u32,
    nsize: usize,
    nquality: u8,
) -> Result<Vec<u8>, JsValue> {
    let mut out: Vec<u8> = Vec::with_capacity(nsize);
    // Reserve space at the start for length header
    out.extend_from_slice(&[0, 0, 0, 0]);

    let thumbnail_len =
        _resize_and_pad(src, &mut out, nwidth, nheight, nsize, nquality).map_err(|e| e.to_string())?;
    out.splice(..4, thumbnail_len.to_be_bytes());
    out.resize(nsize, 0);
    
    Ok(out)
}
