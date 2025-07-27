//! Parse FHIR R5 specifications JSON file `profiles-types.json` resource inner data.
//!
//! Example:
//! 
//! ```json
//! {
//!     "resourceType": "StructureDefinition",
//!     "id": "Narrative",
//! }
//! ```

use crate::r5::parse::profiles_types::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Example: "resourceType": "StructureDefinition"
    pub resource_type: String,

    /// Example: "id" : "Narrative"
    pub id: String,

    /// Example: { "lastUpdated" : "2023-03-26T15:21:02.749+11:00" }
    pub meta: Option<::serde_json::Value>,

    /// Example: { "status" : "generated", … }
    pub text: Option<::serde_json::Value>, 

    /// Example: { "url" : "…", "valueCode" : "normative" }
    pub extension: Option<::serde_json::Value>,

    /// Example: "url" : "http://hl7.org/fhir/StructureDefinition/Narrative"
    pub url: Option<String>,

    /// Example: "version" : "5.0.0"
    pub version: Option<String>,

    /// Example: "name" : "Narrative"
    pub name: Option<String>,

    /// Example: "status" : "active"
    pub status: Option<String>,

    /// Example: "experimental" : false
    pub experimental: Option<bool>,

    /// Example: "date" : "2023-03-26T15:21:02+11:00"
    pub date: Option<String>,

    /// Example: "publisher" : "HL7 FHIR Standard"
    pub publisher: Option<String>,

    /// Example: { "contact" : [{ "telecom" : [{ … }] }] }
    pub contact: Option<::serde_json::Value>, 

    /// Example: "description" : "Narrative Type: A human-readable summary…",
    pub description: Option<String>,

    /// Example: "jurisdiction": "coding" : [{…}]
    pub jurisdiction: Option<::serde_json::Value>,

    /// Example: "fhirVersion": "5.0.0"
    pub fhir_version: Option<String>,

    /// Example: "mapping": [{ "identity" : "rim", … ]}
    pub mapping: Option<::serde_json::Value>,

    /// Example: "kind": "complex-type"
    pub kind: Option<String>,

    /// Example: "abstract": true
    #[serde(rename = "abstract")]
    pub abstract_x: Option<bool>,

    /// Example: "type": "Narrative"
    #[serde(rename = "type")]
    pub type_x: Option<String>,

    /// Example: "baseDefinition" : "http://hl7.org/fhir/StructureDefinition/DataType"
    pub base_definition: Option<String>,
      
    /// Example: "snapshot": { "element": [...] }
    pub snapshot: Option<Snapshot>,

    /// Example: "differential": { "element": [...] }
    pub differential: Option<Differential>,

    /// Example: "derivation" : "specialization"
    pub derivation: Option<String>,

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Resource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            resource_type: String::default(),
            id: String::default(),
            meta: None,
            text: None,
            extension: None,
            url: None,
            version: None,
            name: None,
            status: None,
            experimental: None,
            date: None,
            publisher: None,
            contact: None,
            description: None,
            jurisdiction: None,
            fhir_version: None,
            mapping: None,
            kind: None,
            abstract_x: None,
            type_x: None,
            snapshot: None,
            differential: None,
            base_definition: None,
            derivation: None,
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "resourceType": "",
                    "id": "",
                    "meta": null,
                    "text": null,
                    "extension": null,
                    "url": null,
                    "version": null,
                    "name": null,
                    "status": null,
                    "experimental": null,
                    "date": null,
                    "publisher": null,
                    "contact": null,
                    "description": null,
                    "jurisdiction": null,
                    "fhirVersion": null,
                    "mapping": null,
                    "kind": null,
                    "abstract": null,
                    "type": null,
                    "snapshot": null,
                    "differential": null,
                    "baseDefinition": null,
                    "derivation": null,
                }
            );
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!(
                {
                    "resourceType": "",
                    "id": ""
                }
            );
            assert_eq!(actual, expect);
        }

        use std::path::PathBuf;
        use std::sync::LazyLock;

        pub static FILE: LazyLock<PathBuf> = LazyLock::new(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("r5")
                .join("parse")
                .join("profiles_types")
                .join("resource.json")
        });

        #[test]
        fn serde_json_from_reader() {
            let file = std::fs::File::open(&*FILE).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T =
                ::serde_json::from_reader(reader).expect("from_reader");
            assert_eq!(actual.id, "Narrative");
        }
    }
}
