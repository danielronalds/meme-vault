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
fn image_encryption_to_decryption() {
    let png_path = format!(
        "./{}.png",
        download_random_meme(meme_vault_fetcher::MemeSubreddit::WholesomeMemes, ".")
            .expect("Failed to download meme")
            .replace(" ", "_"),
    );

    let record = "Testing 123";

    embed_record(record.as_bytes(), &png_path).expect("Failed to embed record");

    let dislodged_record = dislodge_record(png_path).expect("Failed to dislodge record");

    assert_eq!(record, dislodged_record);
}
