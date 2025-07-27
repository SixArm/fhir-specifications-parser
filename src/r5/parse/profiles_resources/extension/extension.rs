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
pub struct Extension {
    /// Example: "url" : "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
    pub url: String,

    /// Example:  "valueBoolean" : "id"
    pub value_boolean: Option<bool>,

    /// Example:  "valueCanonical" : "…"
    pub value_canonical: Option<String>,

    /// Example:  "valueCode" : "…"
    pub value_code: Option<String>,

    /// Example:  "valueInteger" : "…"
    pub value_integer: Option<i32>,

    /// Example:  "valueString" : "…"
    pub value_string: Option<String>,

    /// Example:  "valueUri" : "…"
    pub value_uri: Option<String>,

    /// Example:  "valueUrl" : "…"
    pub value_url: Option<String>,

    /// Handle other value[x] types
    // TODO
    pub value: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Extension;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_resources::DIR
            .join("extension")
            .join("extension.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
