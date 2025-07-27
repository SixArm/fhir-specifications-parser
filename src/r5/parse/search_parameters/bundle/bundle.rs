//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::search_parameters::*;
use crate::r5::parse::aa::Meta;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {

    /// Example: "resourceType": "Bundle"
    pub resource_type: String,

    /// Example: "id" : "types"
    pub id: String,

    /// Example: "type" : "collection"
    #[serde(rename = "type")]
    pub r#type: String,

    /// Example: "meta" : { "lastUpdated" : "…" }
    pub meta: Meta,

    /// Example "entry": [{"fullUrl": …, "resource": …}]
    pub entry: Vec<Entry>,

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
            id: String::default(),
            r#type: String::default(),
            meta: Meta::default(),
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
                    "id": "",
                    "type": "",
                    "meta": {
                        "lastUpdated": ""
                    },
                    "entry": []
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
                    "resourceType": "",
                    "id": "",
                    "type": "",
                    "meta": {
                        "lastUpdated": ""
                    },
                    "entry": []
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::search_parameters::DIR
                .join("bundle")
                .join("bundle.json");    
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_eq!(actual.id, "searchParams");
        }
    }
}
