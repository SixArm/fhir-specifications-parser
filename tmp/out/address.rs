//! Address
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Address
//!
//! Version: 5.0.0
//!
//! Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.
The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.
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
pub struct Address {
    /// An address expressed using postal conventions (as opposed to GPS or other location definition formats)
    Address: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// home | work | temp | old | billing - purpose of this address
    use: ? // code

    /// postal | physical | both
    type: ? // code

    /// Text representation of the address
    text: ? // string

    /// Street name, number, direction & P.O. Box etc.
    line: ? // string

    /// Name of city, town etc.
    city: ? // string

    /// District name (aka county)
    district: ? // string

    /// Sub-unit of country (abbreviations ok)
    state: ? // string

    /// Postal code for area
    postalCode: ? // string

    /// Country (e.g. may be ISO 3166 2 or 3 letter code)
    country: ? // string

    /// Time period when address was/is in use
    period: ? // Period

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Address;

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
