//! MarketingStatus
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MarketingStatus
//!
//! Version: 5.0.0
//!
//! MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.
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
pub struct MarketingStatus {
    /// The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available
    MarketingStatus: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Extensions that cannot be ignored even if unrecognized
    modifierExtension: ? // Extension

    /// The country in which the marketing authorization has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements
    country: ? // CodeableConcept

    /// Where a Medicines Regulatory Agency has granted a marketing authorization for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlled terminology The controlled term and the controlled term identifier shall be specified
    jurisdiction: ? // CodeableConcept

    /// This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples
    status: ? // CodeableConcept

    /// The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain
    dateRange: ? // Period

    /// The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain
    restoreDate: ? // dateTime

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MarketingStatus;

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
