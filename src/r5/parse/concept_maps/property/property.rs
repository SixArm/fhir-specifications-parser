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
pub struct Property {
    /// Example: "code": "priority"
    pub code: String,

    /// Example: "description": "The priority with which to choose this mapping over the other mappings for the same code"
    pub description: Option<String>,

    /// Example: "type": "string"
    pub r#type: String,

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Property;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("property")
            .join("property.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
