//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! To list the keys using jq:
//!
//! ```sh
//! cat conceptmaps.json | jq -r '.entry[0] | .resource | keys_unsorted[]'
//! resourceType
//! id
//! text
//! url
//! version
//! name
//! title
//! status
//! experimental
//! date
//! publisher
//! jurisdiction
//! sourceScopeCanonical
//! targetScopeCanonical
//! group
//! ```
//!
//! To list all the keys and sort by count descending:
//!
//! ```sh
//! cat conceptmaps.json | jq -r '.entry[] | .resource | keys_unsorted[]' | sort | uniq -c | sort -nr
//! ```

use crate::r5::parse::concept_maps::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Example: "resourceType": "StructureDefinition"
    pub resource_type: String,

    /// Example: "id" : "Narrative"
    pub id: String,

    /// Example: { "status" : "generated", … }
    pub text: Option<::serde_json::Value>,

    /// Example: "url" : "http://hl7.org/fhir/StructureDefinition/Narrative"
    pub url: String,

    /// Example: "version" : "5.0.0"
    pub version: String,

    /// Example: "name" : "Narrative"
    pub name: Option<String>,

    /// Example: "title" : "v2 map for AdministrativeGender"
    pub title: String,

    /// Example: "status" : "active"
    pub status: String,

    /// Example: "experimental" : false
    pub experimental: bool,

    /// Example: "date" : "2023-03-26T15:21:02+11:00"
    pub date: Option<String>,

    /// Example: "publisher" : "HL7 FHIR Standard"
    pub publisher: Option<String>,

    /// Example: "jurisdiction": [{"coding" : [{…}] }]
    pub jurisdiction: Option<Vec<Jurisdiction>>,

    /// Example: { "contact" : [{ "telecom" : [{ … }] }] }
    pub contact: Option<::serde_json::Value>,

    /// Example: "description" : "Narrative Type: A human-readable summary…",
    pub description: Option<String>,

    // Example: { "url" : "…", "valueCode" : "normative" }
    pub extension: Option<::serde_json::Value>,

    /// Example: { "profile" : "…" }
    pub meta: Option<Meta>,

    /// TODO
    pub identifier: Option<Vec<Identifier>>,

    /// Example: "sourceScopeCanonical" : "http://hl7.org/fhir/ValueSet/administrative-gender",
    pub source_scope_canonical: Option<String>,

    /// Example: "sourceScopeUri" : "http://hl7.org/fhir/ValueSet/address-use",
    pub source_scope_uri: Option<String>,

    /// Example: "targetScopeCanonical" : "http://terminology.hl7.org/ValueSet/v2-0001",
    pub target_scope_canonical: Option<String>,

    /// Example: "targetScopeUri" : "http://terminology.hl7.org/ValueSet/v3-AddressUse",
    pub target_scope_uri: Option<String>,

    /// Example: { "source" : "…", … }
    pub group: ::serde_json::Value,

    /// TODO
    pub copyright: Option<String>,

    /// TODO
    pub purpose: Option<String>,

    /// TODO
    pub topic: Option<Vec<Topic>>,

    /// TODO
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// TODO
    pub property: Option<Vec<Property>>,

    /// TODO
    pub approval_date: Option<String>,

    /// TODO
    pub last_review_date: Option<String>,

    /// The author list of contact items for the work.
    pub author: Option<Vec<Contact>>,

    /// The editor list of contact items for the work.
    pub editor: Option<Vec<Contact>>,

    /// The endorser list of contact items for the work.
    pub endorser: Option<Vec<Contact>>,

    /// The reviewer list of contact items for the work.
    pub reviewer: Option<Vec<Contact>>,

    /// TODO
    pub effective_period: Option<Range>,

    /// Example: TODO
    pub additional_attribute: Option<Vec<AdditionalAttribute>>,

    // Example: TODO
    pub use_context: Option<Vec<UseContext>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Resource;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::concept_maps::DIR
            .join("resource")
            .join("resource.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.id, "cm-administrative-gender-v2");
    }
}
