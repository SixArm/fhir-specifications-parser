//! Annotation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Annotation
//! 
//! Version: 5.0.0
//! 
//! Annotation Type: A  text note which also  contains information about who made the statement and when.
//! 
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub author: types::String, // DataType [0..1] // « Reference( Practitioner | PractitionerRole | Patient | RelatedPerson |Organization )| string »
    pub time: Option<types::DateTime>,
    pub text: types::Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Annotation;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            author: types::String::default(),
            time: None,
            text: types::Markdown::default(),
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
                    "author": {},
                    "text": {}
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
                    "author": {},
                    "text": {}
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
