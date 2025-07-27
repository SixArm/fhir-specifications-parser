//! Parse FHIR R5 specifications JSON file `conceptmaps.json`
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

//TODO

use crate::r5::parse::concept_maps::resource::Resource;
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// Example: "resourceType": "Bundle"
    resource_type: String,

    /// Example "version": "5.0.1"
    entry: Vec<Resource>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Bundle;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            resource_type: String::default(),
            entry: vec![],
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
                    "entry": []
                }
            );
            let actual: Bundle = ::serde_json::from_value(json).expect("from_value");
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
                    "entry": []
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
