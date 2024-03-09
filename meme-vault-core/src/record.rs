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
