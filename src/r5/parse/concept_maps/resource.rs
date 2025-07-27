//! Parse FHIR R5 specifications JSON file `profiles-types.json` entry.
//!
//! **Purpose**: Define mappings between different code systems
//!
//! ```json
//! {
//!   "resourceType": "Bundle",
//!   "entry": [{
//!     "resource": {
//!       "resourceType": "ConceptMap",
//!       "id": "gender-to-v2",
//!       "source": "http://hl7.org/fhir/ValueSet/administrative-gender",
//!       "target": "http://terminology.hl7.org/ValueSet/v2-0001",
//!       "group": [{
//!         "element": [{
//!           "code": "male",
//!           "target": [{
//!             "code": "M",
//!             "equivalence": "equivalent"
//!           }]
//!         }]
//!       }]
//!     }
//!   }]
//! }
//! ```
//!
//! **Use**: Translation tables between coding systems (SNOMED↔ICD-10, LOINC↔local codes)
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
    type T = Resource;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            resource_type: String::default(),
            id: String::default(),
            type_x: String::default(),
            kind: String::default(),
            derivation: String::default(),
            differential: ::serde_json::Value::default(),
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
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!(
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
