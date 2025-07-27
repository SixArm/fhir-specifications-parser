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
pub struct AdditionalAttribute {
    /// Example: "code": "spec-src"
    pub code: String,

    /// Example: "uri": "http://snomed.info/id/3006873018"
    pub uri: String,
            
    /// Example: "description" : "Specimen source"
    pub description: Option<String>,

    /// Example: "type" : "code"
    pub r#type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AdditionalAttribute;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("additional_attribute")
            .join("additional_attribute.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
