//! Identifier
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Identifier
//!
//! Version: 5.0.0
//!
//! Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
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
pub struct Identifier {
    /// An identifier intended for computation
    Identifier: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// usual | official | temp | secondary | old (If known)
    use: ? // code

    /// Description of identifier
    type: ? // CodeableConcept

    /// The namespace for the identifier value
    system: ? // uri

    /// The value that is unique
    value: ? // string

    /// Time period when id is/was valid for use
    period: ? // Period

    /// Organization that issued id (may be just text)
    assigner: ? // Reference

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Identifier;

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
