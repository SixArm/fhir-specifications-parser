//! HumanName
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HumanName
//!
//! Version: 5.0.0
//!
//! HumanName Type: A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name parts or the need for usage information.
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
pub struct HumanName {
    /// Name of a human or other living entity - parts and usage
    HumanName: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// usual | official | temp | nickname | anonymous | old | maiden
    use: ? // code

    /// Text representation of the full name
    text: ? // string

    /// Family name (often called 'Surname')
    family: ? // string

    /// Given names (not always 'first'). Includes middle names
    given: ? // string

    /// Parts that come before the name
    prefix: ? // string

    /// Parts that come after the name
    suffix: ? // string

    /// Time period when name was/is in use
    period: ? // Period

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = HumanName;

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
