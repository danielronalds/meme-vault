use std::{
    error::Error,
    fs,
    io::{Read, Write},
};

use stegano_core::{
    media::image::{decoder::ImageRgbaColor, encoder::ImageRgbaColorMut},
    universal_decoder::{Decoder, OneBitUnveil},
    universal_encoder::{Encoder, OneBitHide},
    SteganoEncoder,
};

/// Embed an encrypted record into the given image
///
/// # Parameters
///
/// - `encrypted_record` The encrypted bytes of a record
/// - `png_path`         The path to the PNG to embed the record in
pub fn embed_record(
    encrypted_record: &[u8],
    png_path: impl AsRef<str>,
) -> Result<(), Box<dyn Error>> {
    let png_path = png_path.as_ref();

    let mut image = image::open(png_path)?.to_rgba8();

    let mut encoder = Encoder::new(ImageRgbaColorMut::new(&mut image).into_iter(), OneBitHide);
    encoder.write_all(&encrypted_record)?;

    // Overwriting the image file
    fs::remove_file(png_path)?;
    Ok(image.save(png_path)?)
}

/// Dislodges an encrypted record from the given image
///
/// # Parametesr
///
/// - `png_path` The path to the PNG that contains the encrypted record
pub fn dislodge_record(png_path: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
    let png_path = png_path.as_ref();

    let mut image = image::open(png_path)?.to_rgba8();

    let mut record_string = String::new();

    let mut decoder = Decoder::new(ImageRgbaColor::new(&image), OneBitUnveil);

    decoder.read_to_string(&mut record_string)?;

    Ok(record_string)
}
