//! Parse FHIR R5 specifications JSON file `profiles-types.json` bundle.
//!
//! Example:
//! 
//! ```json
//! {
//!     "resourceType": "Bundle",
//!     "id" : "types",
//!     "type" : "collection",
//!     "entry": [{
//!         "fullUrl" : "http://hl7.org/fhir/StructureDefinition/Narrative"
//!         "resource": {
//!             "resourceType": "StructureDefinition",
//!             "id" : "Narrative",
//!         }
//!     }]
//! }
//! ```

use crate::r5::parse::profiles_types::*;
use ::serde::{Deserialize, Serialize};

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
    pub type_x: String,

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
            type_x: String::default(),
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
                    "entry": []
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
                .join("bundle.json")
        });

        #[test]
        fn serde_json_from_reader() {
            let file = std::fs::File::open(&*FILE).expect("open");
            let reader = std::io::BufReader::new(file);
            let bundle: T =
                ::serde_json::from_reader(reader).expect("from_reader");
            println!("{:#?}", bundle);
        }

        pub static PROFILE_TYPES_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("doc")
                .join("fhir-specifications")
                .join("r5")
                .join("fhir-definitions-json")
                .join("profiles-types.json")
        });

        #[test]
        fn serde_json_from_reader_with_profile_types_file() {
            let file = std::fs::File::open(&*PROFILE_TYPES_FILE).expect("open");
            let reader = std::io::BufReader::new(file);
            let bundle: T =
                ::serde_json::from_reader(reader).expect("from_reader");
            println!("{:#?}", bundle);
        }
    }
}
