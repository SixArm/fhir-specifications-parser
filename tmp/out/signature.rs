//! Signature
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Signature
//!
//! Version: 5.0.0
//!
//! Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other signature may be as simple as a graphical image representing a hand-written signature, or a signature ceremony Different signature approaches have different utilities.
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
pub struct Signature {
    /// A Signature - XML DigSig, JWS, Graphical image of signature, etc.
    Signature: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Indication of the reason the entity signed the object(s)
    type: ? // Coding

    /// When the signature was created
    when: ? // instant

    /// Who signed
    who: ? // Reference

    /// The party represented
    onBehalfOf: ? // Reference

    /// The technical format of the signed resources
    targetFormat: ? // code

    /// The technical format of the signature
    sigFormat: ? // code

    /// The actual signature content (XML DigSig. JWS, picture, etc.)
    data: ? // base64Binary

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Signature;

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
