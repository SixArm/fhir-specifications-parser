//! ContactPoint
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ContactPoint
//!
//! Version: 5.0.0
//!
//! ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.
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
pub struct ContactPoint {
    /// Details of a Technology mediated contact point (phone, fax, email, etc.)
    ContactPoint: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// phone | fax | email | pager | url | sms | other
    system: ? // code

    /// The actual contact point details
    value: ? // string

    /// home | work | temp | old | mobile - purpose of this contact point
    use: ? // code

    /// Specify preferred order of use (1 = highest)
    rank: ? // positiveInt

    /// Time period when the contact point was/is in use
    period: ? // Period

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ContactPoint;

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
