//! Parse FHIR R5 specifications JSON file `version.info`.
//!
//! **Purpose**: Package metadata and versioning information
//!
//! ```json
//! {
//!   "packageId": "hl7.fhir.us.core",
//!   "version": "5.0.1",
//!   "fhir-version": "4.0.1",
//!   "title": "US Core Implementation Guide",
//!   "canonical": "http://hl7.org/fhir/us/core",
//!   "date": "2023-01-15",
//!   "publisher": "HL7 International",
//!   "dependencies": {
//!     "hl7.fhir.r4.core": "4.0.1",
//!     "hl7.terminology.r4": "3.1.0"
//!   }
//! }
//! ```
//! **Use**: Metadata for FHIR packages, version control, and dependencies
//!

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    /// Example: "package-id": "hl7.fhir.us.core"
    package_id: String,

    /// Example "version": "5.0.1"
    version: String,

    /// Example: "fhir-version": "4.0.1"
    fhir_version: String,

    /// Example: "title": "US Core Implementation Guide",
    title: String,

    /// Example: "canonical": "http://hl7.org/fhir/us/core",
    canonical: String,

    /// Example: "date": "2023-01-15",
    date: String,

    /// Example: "publisher": "HL7 International"
    publisher: String,

    /// Example:
    ///   "dependencies": {
    ///   "hl7.fhir.r4.core": "4.0.1",
    ///   "hl7.terminology.r4": "3.1.0"
    /// }
    ///
    /// TODO: build a struct for dependencies
    dependencies: ::serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = VersionInfo;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            package_id: String::default(),
            version: String::default(),
            fhir_version: String::default(),
            title: String::default(),
            canonical: String::default(),
            date: String::default(),
            publisher: String::default(),
            dependencies: ::serde_json::Value::Null,
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
                    "packageId": "",
                    "version": "",
                    "fhirVersion": "",
                    "title": "",
                    "canonical": "",
                    "date": "",
                    "publisher": "",
                    "dependencies": null
                }
            );
            let actual: VersionInfo = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!(
                {
                    "packageId": "",
                    "version": "",
                    "fhirVersion": "",
                    "title": "",
                    "canonical": "",
                    "date": "",
                    "publisher": "",
                    "dependencies": null
                }
            );
            assert_eq!(actual, expect);
        }
    }
}
