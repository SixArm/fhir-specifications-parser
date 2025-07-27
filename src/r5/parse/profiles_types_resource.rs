//! Parse FHIR R5 specifications JSON file `profiles-types.json` entry.
//!
//! **Purpose**: Profiles on FHIR data types and primitive types
//!
//! ```json
//! {
//!   "resourceType": "Bundle",
//!   "entry": [{
//!     "resource": {
//!       "resourceType": "StructureDefinition",
//!       "id": "us-core-birthsex",
//!       "type": "code",
//!       "kind": "primitive-type",
//!       "derivation": "constraint",
//!       "differential": {
//!         "element": [{
//!           "id": "code",
//!           "binding": {
//!             "strength": "required",
//!             "valueSet": "http://hl7.org/fhir/us/core/ValueSet/birthsex"
//!           }
//!         }]
//!       }
//!     }
//!   }]
//! }
//! ```
//!
//! **Contains**:
//!
//! - Constrained primitive types (special string formats, code patterns)
//! - Complex type profiles (special Address, Identifier formats)
//! - Custom data type definitions
//!

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {

    /// Example: "resourceType": "StructureDefinition"
    resource_type: String,

    /// Example: "id": "us-core-birthsex"
    id: String,

    /// Example: "type": "code"
    #[serde(rename = "type")]
    type_x: String,

    /// Example: "kind": "primitive-type"
    kind: String,

    /// Example: "derivation": "constraint"
    derivation: String,

    /// Example: "differential": { "element": [...] }
    differential: ::serde_json::Value,

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {};
        assert_eq!(profiles_types_resource, Resource {
            resource_type: String::default(),
            id: String::default(),
            type_x: String::default(),
            kind: String::default(),
            derivation: String::default(),
            differential: ::serde_json::Value::default(),
        });
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
                    "type": "",
                    "kind": "",
                    "derivation": "",
                    "differential": null
                }
            );
            let actual: Resource = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value = ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value =  json!(
                {
                    "resourceType": "",
                    "id": "",
                    "type": "",
                    "kind": "",
                    "derivation": "",
                    "differential": null
                }
            );
            assert_eq!(actual, expect);
        }

    }

}