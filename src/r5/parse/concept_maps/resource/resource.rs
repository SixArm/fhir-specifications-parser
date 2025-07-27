//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

//use crate::r5::parse::concept_maps::*;
use crate::r5::parse::aa::Jurisdiction;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Example: "resourceType": "ConceptMap"
    pub resource_type: String,

    /// Example: "id" : "cm-administrative-gender-v2"
    pub id: String,

    /// Example: { "status" : "extensions", … }
    pub text: Option<::serde_json::Value>, 

    /// Example: "url" : "http://hl7.org/fhir/ConceptMap/cm-administrative-gender-v2"
    pub url: String,

    /// Example: "version" : "5.0.0"
    pub version: String,

    /// Example: "name" : "v2.AdministrativeGender"
    pub name: Option<String>,

    /// Example: "title" : "v2 map for AdministrativeGender"
    pub title: String,

    /// Example: "status" : "draft"
    pub status: String,

    /// Example: "experimental" : false
    pub experimental: bool,

    /// Example: "date" : "2023-03-26T15:21:02+11:00"
    pub date: Option<String>,

    /// Example: "publisher" : "HL7 FHIR Standard"
    pub publisher: Option<String>,

    /// Example: "jurisdiction": [{"coding" : [{…}] }]
    pub jurisdiction: Option<Vec<Jurisdiction>>,

    /// Example: "sourceScopeCanonical" : "http://hl7.org/fhir/ValueSet/administrative-gender",
    pub source_scope_canonical: Option<String>,
    
    /// Example: "sourceScopeUri" : "http://hl7.org/fhir/ValueSet/address-use",
    pub source_scope_uri: Option<String>,

    /// Example: "targetScopeCanonical" : "http://terminology.hl7.org/ValueSet/v2-0001",
    pub target_scope_canonical: Option<String>,

    /// Example: "targetScopeUri" : "http://terminology.hl7.org/ValueSet/v3-AddressUse",
    pub target_scope_uri: Option<String>,

    /// Example: { "source" : "…", … }
    pub group: Option<::serde_json::Value>, 

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
            text: None,
            url: String::default(),
            version: String::default(),
            name: None,
            title: String::default(),
            status: String::default(),
            experimental: false,
            date: None,
            publisher: None,
            jurisdiction: None,
            source_scope_canonical: None,
            source_scope_uri: None,
            target_scope_canonical: None,
            target_scope_uri: None,
            group: None,
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
                    "url": "",
                    "version": "",
                    "title": "",
                    "status": "",
                    "experimental": false
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
                    "title": "",
                    "status": "",
                    "experimental": false
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::concept_maps::DIR
                .join("resource")
                .join("resource.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_eq!(actual.id, "cm-administrative-gender-v2");
        }
    }
}
