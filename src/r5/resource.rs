//! Resource
//!
//! <https://build.fhir.org/json.html>
//!
//! JSON Representation of Resources
//!
//! ```json```
//! {
//!   "resourceType" : "[Resource Type]",
//!   "resourceDefinition" : "(see below)",
//!   // from Source: property0
//!   "property1" : "<[primitive]>", // short description
//!   "property2" : { [Datatype] }, // short description
//!   "property3" : { // Short Description
//!     "propertyA" : { CodeableConcept }, // Short Description (Example)
//!   },
//!   "property4" : [{ // Short Description
//!     "propertyB" : { Reference(ResourceType) } // R!  Short Description
//!   }]
//! }
//! ```
//!

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub resource_type: String,
    pub resource_definition: String,
    pub properties: Vec<crate::r5::todo::property::Property>,
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
            resource_definition: String::default(),
            properties: vec![],
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
                    "resourceDefinition": "",
                    "properties": []
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
                    "resourceDefinition": "",
                    "properties": []
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
