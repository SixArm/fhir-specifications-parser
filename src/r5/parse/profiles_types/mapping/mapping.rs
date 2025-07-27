//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! ```sh
//! ```

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    /// Example: "identity" : "v2"
    pub identity: String,

    /// Example: "uri" : "http://hl7.org/v2"
    pub uri: String,

    /// Example: "name" : "HL7 V2 Mapping"
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Mapping;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_types::DIR
            .join("mapping")
            .join("mapping.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
