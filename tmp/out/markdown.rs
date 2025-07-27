//! Markdown
//!
//! URL: http://hl7.org/fhir/StructureDefinition/markdown
//!
//! Version: 5.0.0
//!
//! markdown type: A string that may contain Github Flavored Markdown syntax for optional processing by a mark down presentation engine
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
pub struct Markdown {
    /// Primitive Type markdown
    markdown: ? // ?

    /// xml:id (or equivalent in JSON)
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Primitive value for markdown
    value: ? // http://hl7.org/fhirpath/System.String

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Markdown;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {};
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
