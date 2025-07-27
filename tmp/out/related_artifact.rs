//! RelatedArtifact
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RelatedArtifact
//!
//! Version: 5.0.0
//!
//! RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.
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
pub struct RelatedArtifact {
    /// Related artifacts for a knowledge resource
    RelatedArtifact: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comments-on | comment-in | contains | contained-in | corrects | correction-in | replaces | replaced-with | retracts | retracted-by | signs | similar-to | supports | supported-with | transforms | transformed-into | transformed-with | documents | specification-of | created-with | cite-as
    type: ? // code

    /// Additional classifiers
    classifier: ? // CodeableConcept

    /// Short label
    label: ? // string

    /// Brief description of the related artifact
    display: ? // string

    /// Bibliographic citation for the artifact
    citation: ? // markdown

    /// What document is being referenced
    document: ? // Attachment

    /// What artifact is being referenced
    resource: ? // canonical

    /// What artifact, if not a conformance resource
    resourceReference: ? // Reference

    /// draft | active | retired | unknown
    publicationStatus: ? // code

    /// Date of publication of the artifact being referred to
    publicationDate: ? // date

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RelatedArtifact;

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
