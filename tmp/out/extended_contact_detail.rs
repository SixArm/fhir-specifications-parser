//! ExtendedContactDetail
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExtendedContactDetail
//!
//! Version: 5.0.0
//!
//! ExtendedContactDetail Type: Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.
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
pub struct ExtendedContactDetail {
    /// Contact information
    ExtendedContactDetail: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// The type of contact
    purpose: ? // CodeableConcept

    /// Name of an individual to contact
    name: ? // HumanName

    /// Contact details (e.g.phone/fax/url)
    telecom: ? // ContactPoint

    /// Address for the contact
    address: ? // Address

    /// This contact detail is handled/monitored by a specific organization
    organization: ? // Reference

    /// Period that this contact was valid for usage
    period: ? // Period

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExtendedContactDetail;

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
