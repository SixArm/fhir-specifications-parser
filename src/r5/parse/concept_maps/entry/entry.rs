//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! To list the keys using jq:
//!
//! ```sh
//! cat conceptmaps.json | jq -r '.entry[0] | keys_unsorted[]'
//! fullUrl
//! resource
//! ```

use crate::r5::parse::concept_maps::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// Example: "fullUrl" : "http://hl7.org/fhir/StructureDefinition/Narrative"
    pub full_url: String,

    /// Example: "resource" : { "resourceType": "StructureDefinition", "id" : "Narrative",  }
    pub resource: Resource,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Entry;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("entry")
            .join("entry.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(
            actual.full_url,
            "http://hl7.org/fhir/ConceptMap/cm-administrative-gender-v2"
        );
    }
}
