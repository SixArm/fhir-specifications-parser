//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::profiles_types::*;
use crate::r5::parse::aa::Meta;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Example: "resourceType": "StructureDefinition"
    pub resource_type: String,

    /// Example: "id" : "Narrative"
    pub id: String,

    /// Example: { "profile" : "…" }
    pub meta: Meta,

    /// Example: { "status" : "generated", … }
    pub text: Option<::serde_json::Value>, 

    /// Example: { "url" : "…", "valueCode" : "normative" }
    pub extension: Option<::serde_json::Value>,

    /// Example: "url" : "http://hl7.org/fhir/StructureDefinition/Narrative"
    pub url: String,

    /// Example: "version" : "5.0.0"
    pub version: String,

    /// Example: "name" : "Narrative"
    pub name: String,

    /// Example: "status" : "active"
    pub status: String,

    /// Example: "experimental" : false
    pub experimental: bool,

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
    pub r#abstract: Option<bool>,

    /// Example: "type": "Narrative"
    pub r#type: Option<String>,

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
            meta: Meta::default(),
            text: None,
            extension: None,
            url: String::default(),
            version: String::default(),
            name: String::default(),
            status: String::default(),
            experimental: false,
            date: None,
            publisher: None,
            contact: None,
            description: None,
            jurisdiction: None,
            fhir_version: None,
            mapping: None,
            kind: None,
            r#abstract: None,
            r#type: None,
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
                    "url": "",
                    "version": "",
                    "name": "",
                    "status": "",
                    "experimental": false,
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
                    "id": "",
                    "url": "",
                    "version": "",
                    "name": "",
                    "status": "",
                    "experimental": false
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::profiles_types::DIR
                .join("resource")
                .join("resource.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_eq!(actual.id, "Narrative");
        }
    }
}
