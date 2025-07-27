//! Parse FHIR R5 specifications JSON file `profiles-types.json` element inner data.
//!
//! See file `element.json` for example.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// Example: "id": "Narrative.id"
    pub id: String,

    /// Example: "path": "Narrative.id"
    pub path: Option<String>,

    /// Example: "representation": ["xmlAttr"]
    pub representation: Option<Vec<String>>,

    /// Example: "short": "Unique id for inter-element referencing"
    pub short: Option<String>,

    /// Example: "definition": "Unique id for inter-element referencing …"
    pub definition: Option<String>,

    /// Example: "min": 0
    pub min: Option<i32>,

    /// Example: "max": "1"
    pub max: Option<String>,

    /// Example: "base": { "path": "Element.id", "min": 0, "max": "1" }
    pub base: Option<::serde_json::Value>,

    /// Example: "type": [{ "extension": … }]
    #[serde(rename = "type")]
    pub type_x: Option<::serde_json::Value>,

    /// Example: "condition": ["ele-1"]
    pub condition: Option<Vec<String>>,

    /// Example: "isModifier": false
    pub is_modifier: Option<bool>, 

    /// Example: "isSummary": false
    pub is_summary: Option<bool>,

    /// Example: "mapping": [{"identity": "rim", "map": "n/a"}]
    pub mapping: Option<::serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Element;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            id: String::default(),
            path: None,
            representation: None,
            short: None,
            definition: None,
            min: None,
            max: None,
            base: None,
            type_x: None,
            condition: None,
            is_modifier: None,
            is_summary: None,
            mapping: None,
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
                    "id": ""
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
                    "id": ""
                }
            );
            assert_eq!(actual, expect);
        }

        use std::path::PathBuf;
        use std::sync::LazyLock;

        pub static FILE: LazyLock<PathBuf> = LazyLock::new(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("r5")
                .join("parse")
                .join("profiles_types")
                .join("element.json")
        });

        #[test]
        fn serde_json_from_reader() {
            let file = std::fs::File::open(&*FILE).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T =
                ::serde_json::from_reader(reader).expect("from_reader");
            assert_ne!(actual, T::default());
        }
    }
}
