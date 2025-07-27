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
pub struct Identifier {
    /// Example: "system" : "urn:ietf:rfc:3986"
    system: String,

    /// Example: "value" : "urn:oid:2.16.840.1.113883.4.642.4.1827"
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Identifier;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::search_parameters::DIR
            .join("identifier")
            .join("identifier.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
