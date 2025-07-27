//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

//use crate::r5::parse::concept_maps::*;
use crate::r5::parse::aa::{Contact, Identifier, Meta};
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Example: "resourceType": "ConceptMap"
    pub resource_type: String,

    /// Example: "id": "cm-administrative-gender-v2"
    pub id: String,

    /// Example: "meta": { "lastUpdated": "…", "profile": ["…"] }
    pub meta: Meta,

    /// Example: { "status": "extensions", … }
    pub text: Option<::serde_json::Value>, 

    /// Example: [{ "url": "…", … }]
    pub extension: Option<::serde_json::Value>, 

    /// Example: "url" : "http://hl7.org/fhir/ConceptMap/cm-administrative-gender-v2"
    pub url: String,

    /// Example: [{ "system": …", "value": "…" }]
    pub identifier: Option<Vec<Identifier>>, 

    /// Example: "version": "5.0.0"
    pub version: String,

    /// Example: "name": "ACMECholCodesBlood"
    pub name: Option<String>,

    /// Example: "title": "ACME Codes for Cholesterol in Serum/Plasma"
    pub title: String,

    /// Example: "status": "draft"
    pub status: String,

    /// Example: "experimental": false
    pub experimental: bool,

    /// Example: "date": "2023-03-26T15:21:02+11:00"
    pub date: Option<String>,

    /// Example: "publisher": "HL7 FHIR Standard"
    pub publisher: Option<String>,

    /// Example: "contact" : [{"name" : "…", "telecom" : [{…}]}]
    pub contact: Option<Vec<Contact>>,

    /// Example: "description": "This is an example code system that includes all the ACME codes for serum/plasma cholesterol from v2.36.",
    pub description: String,

    /// Example: "caseSensitive" : true,
    pub case_sensitive: Option<bool>,

    /// Example: "content": "complete",
    pub content: Option<String>,

    /// Example: TODO
    pub filter: Option<::serde_json::Value>, 

    /// Example: TODO
    pub concept: Option<::serde_json::Value>, 
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
            identifier: None,
            version: String::default(),
            name: None,
            title: String::default(),
            status: String::default(),
            experimental: false,
            date: None,
            publisher: None,
            contact: None,
            description: String::default(),
            case_sensitive: None,
            content: None,
            filter: None,
            concept: None,
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
                    "meta": {},
                    "url": "",
                    "version": "",
                    "title": "",
                    "status": "",
                    "experimental": false,
                    "description": ""
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
                    "meta": {},
                    "url": "",
                    "version": "",
                    "title": "",
                    "status": "",
                    "experimental": false,
                    "description": ""
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::value_sets::DIR
                .join("resource")
                .join("resource.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_eq!(actual.id, "example");
        }
    }
}
