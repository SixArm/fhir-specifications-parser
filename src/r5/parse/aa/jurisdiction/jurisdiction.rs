//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::aa::Coding;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Jurisdiction {
    /// Example: [{"system": "…", "code" : "…", "display" : "…"}]
    pub coding: Vec<Coding>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Jurisdiction;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            coding: vec![],
        };
        assert_eq!(actual, expect)
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "coding": []
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
                    "coding": []
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::aa::DIR
                .join("jurisdiction")
                .join("jurisdiction.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_eq!(actual, T::default());
        }
    }
}
