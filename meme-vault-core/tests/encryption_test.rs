use meme_vault_core::{Record, encryption::{encrypt_record, decrypt_record}};

#[test]
pub fn test_encryption_to_decryption() {
    let record = Record::new("Tester", "Just a simple testing record", "password");

    let password = "Passw0rd";

    let encrypted_record = encrypt_record(&record, password).expect("Failed to encrypt");

    // Checking to make sure an encryption happened
    assert_ne!(encrypted_record, serde_json::to_string(&record).unwrap().as_bytes());

    let decrypted_record: Record =
        decrypt_record(&encrypted_record, password).expect("Failed to decrypt");

    assert_eq!(decrypted_record, record);
}
