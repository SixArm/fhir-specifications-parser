//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! To list the keys using jq:
//!
//! ```sh
//! cat conceptmaps.json | jq -r 'keys_unsorted[]'
//! resourceType
//! id
//! meta
//! type
//! entry
//! ```
use crate::r5::parse::concept_maps::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// Example: "resourceType": "Bundle"
    pub resource_type: String,

    /// Example: "id" : "types"
    pub id: String,

    /// Example: "type" : "collection"
    #[serde(rename = "type")]
    pub r#type: String,

    /// Example: "meta" : { "lastUpdated" : "…" }
    pub meta: Meta,

    /// Example "entry": [{"fullUrl": …, "resource": …}]
    pub entry: Vec<Entry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("bundle")
            .join("bundle.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.id, "conceptmaps");
    }
}
