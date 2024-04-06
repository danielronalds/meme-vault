use std::fs;

use meme_vault_core::{
    encryption::{decrypt_record, encrypt_record},
    steganography::{dislodge_record, embed_record},
    Record,
};
use meme_vault_fetcher::download_random_meme;

#[test]
pub fn test_encryption_to_decryption() {
    let record = Record::new("Tester", "Just a simple testing record", "password");

    let password = "Passw0rd";

    let encrypted_record = encrypt_record(&record, password).expect("Failed to encrypt");

    // Checking to make sure an encryption happened
    assert_ne!(
        encrypted_record,
        serde_json::to_string(&record).unwrap().as_bytes()
    );

    let decrypted_record: Record =
        decrypt_record(&encrypted_record, password).expect("Failed to decrypt");

    assert_eq!(decrypted_record, record);
}

#[test]
fn image_embedding_and_dislodging() {
    let png_path = download_random_meme(
        meme_vault_fetcher::MemeSubreddit::ProgrammerHumor,
        ".",
        "test.png",
    )
    .expect("Failed to download meme");

    let record = Record::new("Tester", "Just a simple testing record", "password");

    embed_record(&record.as_bytes(), &png_path).expect("Failed to embed record");

    let dislodged_record =
        Record::try_from(dislodge_record(&png_path).expect("Failed to dislodge record"))
            .expect("Failed to dislodge record");

    assert_eq!(record, dislodged_record);

    fs::remove_file(png_path).expect("Failed to delete file");
}

#[test]
fn image_embedding_and_dislodging_with_encryption() {
    let png_path = download_random_meme(
        meme_vault_fetcher::MemeSubreddit::ProgrammerHumor,
        ".",
        "test.png",
    )
    .expect("Failed to download meme");

    let password = "Passw0rd!";

    let record = Record::new("Tester", "Just a simple testing record", "password");

    let encrypted_record = encrypt_record(&record, password).expect("Failed to encrypt record");

    embed_record(&encrypted_record, &png_path).expect("Failed to embed record");

    let dislodged_record = dislodge_record(&png_path).expect("Failed to dislodge record");

    let decrypted_record =
        decrypt_record(dislodged_record.as_bytes(), password).expect("Failed to decrypt record");

    assert_eq!(record, decrypted_record);

    fs::remove_file(png_path).expect("Failed to delete file");
}
