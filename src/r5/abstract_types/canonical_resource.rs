//! CanonicalResource
//!
//! ## UML
//!
//! <https://build.fhir.org/uml.html>
//!
//! CanonicalResource «Interface»:
//!
//! - url : uri [0..1]
//! - identifier : Identifier [0..*]
//! - version: Option<types::string::String>,
//! - versionAlgorithm[x] : DataType [0..1] « string|Coding;
//! - VersionAlgorithm+ »
//! - name: Option<types::string::String>, « C »
//! - title: Option<types::string::String>,
//! - status : code [1..1] « PublicationStatus! »
//! - experimental : boolean [0..1]
//! - date : dateTime [0..1]
//! - publisher: Option<types::string::String>,
//! - contact : ContactDetail [0..*]
//! - description : markdown [0..1]
//! - useContext : UsageContext [0..*]
//! - jurisdiction : CodeableConcept [0..*] « JurisdictionValueSet+ »
//! - purpose : markdown [0..1]
//! - copyright : markdown [0..1]
//! - copyrightLabel: Option<types::string::String>,

#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CanonicalResource {
    pub url: Option<types::Uri>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    pub version: Option<types::String>,

    pub version_algorithm: types::String, // « string | Coding; VersionAlgorithm+ »

    pub name: Option<types::String>,

    pub title: Option<types::string::String>,

    pub status: types::code::Code, // « PublicationStatus! »

    pub experimental: Option<types::Boolean>,

    pub date: Option<types::DateTime>,

    pub publisher: Option<types::String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    pub description: Option<types::Markdown>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    pub purpose: Option<types::Markdown>,

    pub copyright: Option<types::Markdown>,

    pub copyright_label: Option<types::String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CanonicalResource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            url: None,
            identifier: vec![],
            version: None,
            version_algorithm: types::String::default(),
            name: None,
            title: None,
            status: types::Code::default(),
            experimental: None,
            date: None,
            publisher: None,
            contact: vec![],
            description: None,
            use_context: vec![],
            jurisdiction: vec![],
            purpose: None,
            copyright: None,
            copyright_label: None,
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
                    "identifier": [],
                    "versionAlgorithm": {},
                    "status": {},
                    "contact": [],
                    "useContext": [],
                    "jurisdiction": [],
                }
            );
            let actual: CanonicalResource = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!(
                {
                    "versionAlgorithm": {},
                    "status": {},
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
