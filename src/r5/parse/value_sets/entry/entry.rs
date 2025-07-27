//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::value_sets::*;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {

    /// Example: "fullUrl" : "http://hl7.org/fhir/StructureDefinition/Narrative"
    pub full_url: String,

    /// Example: "resource" : { "resourceType": "StructureDefinition", "id" : "Narrative",  }
    pub resource: Resource,

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Entry;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            full_url: String::default(),
            resource: Resource::default(),
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
                    "fullUrl": "",
                    "resource": {
                        "resourceType": "",
                        "id": "",
                        "meta": {},
                        "url": "",
                        "version": "",
                        "title": "",
                        "status": "",
                        "experimental": false,
                        "description": ""
                    }
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
            let expect: ::serde_json::Value = json!(
                {
                    "fullUrl": "",
                    "resource": {
                        "resourceType": "",
                        "id": "",
                        "meta": {},
                        "url": "",
                        "version": "",
                        "title": "",
                        "status": "",
                        "experimental": false,
                        "description": ""
                    }
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::value_sets::DIR
                .join("entry")
                .join("entry.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_eq!(actual.full_url, "http://hl7.org/fhir/CodeSystem/example");
        }

    }
}
