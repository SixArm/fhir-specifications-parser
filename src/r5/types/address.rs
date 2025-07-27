//! Address
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Address
//! 
//! Version: 5.0.0
//! 
//! Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world. The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.
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
    #[serde(rename = "use")]
    pub use_x: Option<types::Code>, // « AddressUse! »

    #[serde(rename = "type")]
    pub type_x: Option<types::Code>, // « AddressType! »

    pub text: Option<types::String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<types::String>,

    pub city: Option<types::String>,

    pub district: Option<types::String>,

    pub state: Option<types::String>,

    pub postal_code: Option<types::String>,

    pub country: Option<types::String>,

    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Address;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            use_x: None,
            type_x: None,
            text: None,
            line: vec![],
            city: None,
            district: None,
            state: None,
            postal_code: None,
            country: None,
            period: None,
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
                    "line": []
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
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
