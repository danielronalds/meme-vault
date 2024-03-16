use std::{
    error::Error,
    fs,
    io::{Read, Write},
};

use steganography::{
    decoder::Decoder,
    encoder::Encoder,
    util::{file_as_dynamic_image, file_as_image_buffer, save_image_buffer, bytes_to_str},
};

use crate::{record, Record};

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

    let image = file_as_dynamic_image(png_path.to_string());

    let encoder = Encoder::new(encrypted_record, image);

    let encoded_image = encoder.encode_alpha();

    save_image_buffer(encoded_image, png_path.to_string());

    Ok(())
}

/// Dislodges an encrypted record from the given image
///
/// # Parametesr
///
/// - `png_path` The path to the PNG that contains the encrypted record
pub fn dislodge_record(png_path: impl AsRef<str>) -> Result<String, Box<dyn Error>> {
    let png_path = png_path.as_ref();

    let encoded_image = file_as_image_buffer(png_path.to_string());

    let decoder = Decoder::new(encoded_image);

    let out_buffer = decoder.decode_alpha();

    //If there is no alpha, it's set to 255 by default so we filter those out
    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();

    Ok(bytes_to_str(&clean_buffer).to_string())
}
