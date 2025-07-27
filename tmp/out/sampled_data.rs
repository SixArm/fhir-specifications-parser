//! SampledData
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SampledData
//!
//! Version: 5.0.0
//!
//! SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
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
pub struct SampledData {
    /// A series of measurements taken by a device
    SampledData: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Zero value and units
    origin: ? // Quantity

    /// Number of intervalUnits between samples
    interval: ? // decimal

    /// The measurement unit of the interval between samples
    intervalUnit: ? // code

    /// Multiply data by this before adding to origin
    factor: ? // decimal

    /// Lower limit of detection
    lowerLimit: ? // decimal

    /// Upper limit of detection
    upperLimit: ? // decimal

    /// Number of sample points at each time point
    dimensions: ? // positiveInt

    /// Defines the codes used in the data
    codeMap: ? // canonical

    /// Offsets, typically in time, at which data values were taken
    offsets: ? // string

    /// Decimal values with spaces, or "E" | "U" | "L", or another code
    data: ? // string

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SampledData;

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
