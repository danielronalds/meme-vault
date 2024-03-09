use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
/// Struct to represent a record of a password
pub struct Record {
    /// The name of the record, e.g. "YouTube"
    name: String,
    /// The description of the record, e.g. "My password for YouTube"
    description: String,
    /// The password associated with the record, e.g. "FabulousYoutubePassw0rd"
    password: String,
}

impl Record {
    pub fn new<T: Into<String>>(name: T, description: T, password: T) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            password: password.into(),
        }
    }

    /// Returns the records json as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let json = serde_json::to_string(self).expect("Failed to serialise");
        json.to_owned().as_bytes().to_owned()
    }
}

/// Encrypts a record
///
/// # Parameters
///
/// - `record` The record to encrypt
///
/// # Returns
///
/// `None` if the record fails to encrypt, otherwise the encypted bytes
pub fn encrypt_record(record: &Record, password: impl Into<String>) -> Option<Vec<u8>> {
    let password = password.into();
    let password_bytes: &[u8] = password.as_bytes();

    let record_bytes = record.as_bytes();

    // NOTE: We return bytes here, as creating a string loses the encrypted information
    simple_crypt::encrypt(&record_bytes, password_bytes).ok()
}

/// Decrypts a record
///
/// # Parameters
///
/// - `encrypted_record` The encrypted bytes that should contain the record
/// - `password`         The password to decrypt the record with
///
/// # Returns
///
/// `None` if the bytes fail to decypt, otherwise a [`Record`]
pub fn decrypt_record(
    encrypted_record: &[u8],
    password: impl AsRef<str>,
) -> Option<Record> {
    let password: &str = password.as_ref();

    let decrypted_bytes = simple_crypt::decrypt(encrypted_record, password.as_bytes())
        .expect("Failed to decrypt");

    let decypted_json = String::from_utf8_lossy(&decrypted_bytes).to_string();

    serde_json::from_str(decypted_json.as_str()).ok()
}
