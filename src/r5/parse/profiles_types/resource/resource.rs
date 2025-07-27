//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! ```sh
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
    /// Note profile_types is required; concept_maps is option.
    pub name: Option<String>,

    /// Example: "title" : "v2 map for AdministrativeGender"
    pub title: Option<String>,

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

    /// Example: "jurisdiction": [{"coding" : [{…}] }]
    pub jurisdiction: Option<Vec<Jurisdiction>>,

    //// profiles_types
    /// Example: "purpose": "Data Elements are defined for each element to assist in questionnaire construction etc",
    pub purpose: Option<String>,

    /// Example: "kind": "logical"
    pub kind: Option<String>,

    /// Example: "abstract": false,
    pub r#abstract: Option<bool>,

    /// Example: "type": "date.id"
    pub r#type: Option<String>,

    /// Example: "derivation": "specialization"
    pub derivation: Option<String>,

    /// Example: "fhirVersion": "5.0.0"
    pub fhir_version: Option<String>,

    /// Example: "snapshot": { "element": [...] }
    pub snapshot: Option<Snapshot>,

    /// Example: "differential": { "element": [...] }
    pub differential: Option<Differential>,

    /// Example: "baseDefinition" : "http://hl7.org/fhir/StructureDefinition/DataType"
    pub base_definition: Option<String>,

    /// Example: "mapping": [{ "identity" : "rim", … ]}
    pub mapping: Option<Vec<Mapping>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Resource;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_types::DIR
            .join("resource")
            .join("resource.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.id, "Narrative");
    }
}
