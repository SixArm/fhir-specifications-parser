//! DataRequirement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DataRequirement
//!
//! Version: 5.0.0
//!
//! DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.
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
pub struct DataRequirement {
    /// Describes a required data item
    DataRequirement: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// The type of the required data
    type: ? // code

    /// The profile of the required data
    profile: ? // canonical

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    : ? // CodeableConcept

    /// Indicates specific structure elements that are referenced by the knowledge module
    mustSupport: ? // string

    /// What codes are expected
    codeFilter: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// A code-valued attribute to filter on
    path: ? // string

    /// A coded (token) parameter to search on
    searchParam: ? // string

    /// ValueSet for the filter
    valueSet: ? // canonical

    /// What code is expected
    code: ? // Coding

    /// What dates/date ranges are expected
    dateFilter: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// A date-valued attribute to filter on
    path: ? // string

    /// A date valued parameter to search on
    searchParam: ? // string

    /// The value of the filter, as a Period, DateTime, or Duration value
    : ? // dateTime

    /// What values are expected
    valueFilter: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// An attribute to filter on
    path: ? // string

    /// A parameter to search on
    searchParam: ? // string

    /// eq | gt | lt | ge | le | sa | eb
    comparator: ? // code

    /// The value of the filter, as a Period, DateTime, or Duration value
    : ? // dateTime

    /// Number of results
    limit: ? // positiveInt

    /// Order of the results
    sort: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// The name of the attribute to perform the sort
    path: ? // string

    /// ascending | descending
    direction: ? // code

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DataRequirement;

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
