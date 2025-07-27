//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::profiles_types::*;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub element: Vec<Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Snapshot;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            element: vec![],
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
                    "element": []
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
                    "element": []
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::profiles_types::DIR
                .join("snapshot")
                .join("snapshot.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_ne!(actual, T::default());
        }
    }
}
