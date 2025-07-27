//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! ```sh
//! ```
use crate::r5::parse::profiles_types::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// Example: "name" : "FHIR project team"
    name: Option<String>,

    /// Example: "telecom" : [{"system" : "…", "value" : "…"}]
    telecom: Vec<Identifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Contact;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_types::DIR
            .join("contact")
            .join("contact.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
