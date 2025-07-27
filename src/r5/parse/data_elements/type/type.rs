//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! ```sh
//! ```
use crate::r5::parse::data_elements::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct r#Type {
    /// Example: "extension": [{"url": …, "valueUrl": … }],
    pub extension: Option<Vec<Extension>>,

    /// Example: "code": "http://hl7.org/fhirpath/System.String"
    pub code: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = r#Type;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::data_elements::DIR
            .join("type")
            .join("type.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
