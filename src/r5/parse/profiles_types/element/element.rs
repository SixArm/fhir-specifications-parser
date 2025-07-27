//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::profiles_types::*;
use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// Example: "id": "Narrative.id"
    pub id: String,

    /// Example: "path": "Narrative.id"
    pub path: String,

    /// Example: "representation": ["xmlAttr"]
    pub representation: Option<Vec<String>>,

    /// Example: "short": "Unique id for inter-element referencing"
    pub short: Option<String>,

    /// Example: "definition": "Unique id for inter-element referencing …"
    pub definition: Option<String>,

    /// Example: "min": 0
    pub min: Option<u32>,

    /// Example: "max": "1"
    pub max: Option<String>,

    /// Example: "base": { "path": "Element.id", "min": 0, "max": "1" }
    pub base: Option<Base>,

    /// Example: "type": [{ "extension": … }]
    #[serde(rename = "type")]
    pub r#type: Option<Vec<r#Type>>,

    /// TODO
    pub constraint: Option<Vec<Constraint>>,

    /// TODO
    pub binding: Option<Binding>,

    /// TODO
    pub extension: Option<Vec<Extension>>,

    /// TODO common properties
    pub comment: Option<String>,
    pub requirements: Option<String>,
    pub alias: Option<Vec<String>>,
    pub fixed_value: Option<serde_json::Value>,
    pub pattern_value: Option<serde_json::Value>,
    pub example: Option<Vec<Example>>,
    
    // Additional properties
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,

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
            path: String::default(),
            representation: None,
            short: None,
            definition: None,
            min: None,
            max: None,
            base: None,
            r#type: None,
            constraint: None,
            binding: None,
            extension: None,
            comment: None,
            requirements: None,
            alias: None,
            fixed_value: None,
            pattern_value: None,
            example: None,
            additional: std::collections::HashMap::default(),
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
                    "id": "",
                    "path": ""
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
                    "id": "",
                    "path": ""
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::profiles_types::DIR
                .join("element")
                .join("element.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_ne!(actual, T::default());
        }
    }
}
