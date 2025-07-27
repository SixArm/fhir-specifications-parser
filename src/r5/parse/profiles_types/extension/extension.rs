//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Serialize, Deserialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    /// Example: "url" : "http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type"
    pub url: String,

    /// Example:  "valueBoolean" : "id" 
    pub value_boolean: Option<bool>,

    /// Example:  "valueCanonical" : "…" 
    pub value_canonical: Option<String>,

    /// Example:  "valueCode" : "…" 
    pub value_code: Option<String>,

    /// Example:  "valueInteger" : "…" 
    pub value_integer: Option<i32>,

    /// Example:  "valueString" : "…" 
    pub value_string: Option<String>,

    /// Example:  "valueUri" : "…" 
    pub value_uri: Option<String>,

    /// Example:  "valueUrl" : "…" 
    pub value_url: Option<String>,
    
    /// Handle other value[x] types
    // TODO
    pub value: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Extension;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            url: String::default(),
            value_boolean: None,
            value_canonical: None,
            value_code: None,
            value_integer: None,
            value_string: None,
            value_uri: None,
            value_url: None,
            value: None, //TODO? std::collections::HashMap::default(),
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
                    "url": ""
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
                    "url": ""
                }
            );
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_from_reader() {
            let path = crate::parse::profiles_types::DIR
                .join("extension")
                .join("extension.json");
            let file = std::fs::File::open(path).expect("open");
            let reader = std::io::BufReader::new(file);
            let actual: T = ::serde_json::from_reader(reader).unwrap();
            assert_ne!(actual, T::default());
        }
    }
}
