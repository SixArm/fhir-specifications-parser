//! Attachment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Attachment
//!
//! Version: 5.0.0
//!
//! Attachment Type: For referring to data content defined in other formats.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Content in a format defined elsewhere
    Attachment: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Mime type of the content, with charset etc.
    contentType: ? // code

    /// Human language of the content (BCP-47)
    language: ? // code

    /// Data inline, base64ed
    data: ? // base64Binary

    /// Uri where the data can be found
    url: ? // url

    /// Number of bytes of content (if url provided)
    size: ? // integer64

    /// Hash of the data (sha-1, base64ed)
    hash: ? // base64Binary

    /// Label to display in place of the data
    title: ? // string

    /// Date attachment was first created
    creation: ? // dateTime

    /// Height of the image in pixels (photo/video)
    height: ? // positiveInt

    /// Width of the image in pixels (photo/video)
    width: ? // positiveInt

    /// Number of frames if > 1 (photo)
    frames: ? // positiveInt

    /// Length in seconds (audio / video)
    duration: ? // decimal

    /// Number of printed pages
    pages: ? // positiveInt

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Attachment;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {};
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
