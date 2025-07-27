//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// Example: "lastUpdated" : "2023-03-26T15:21:02.749+11:00"
    pub last_updated: Option<String>,

    /// Example: "profile" : ["http://hl7.org/fhir/StructureDefinition/shareablecodesystem"]
    pub profile: Option<Vec<String>>,

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Meta;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            last_updated: None,
            profile: None,
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
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn serde_json_from_reader() {
            let path =  crate::r5::parse::aa::DIR
                .join("meta")
                .join("meta.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).expect("from_reader");
            assert_eq!(actual.last_updated, Some(String::from("2023-03-26T15:21:02.749+11:00")));
        }
    }
}
