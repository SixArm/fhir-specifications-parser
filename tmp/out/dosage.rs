//! Dosage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Dosage
//!
//! Version: 5.0.0
//!
//! Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.
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
pub struct Dosage {
    /// How the medication is/was taken or should be taken
    Dosage: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Extensions that cannot be ignored even if unrecognized
    modifierExtension: ? // Extension

    /// The order of the dosage instructions
    sequence: ? // integer

    /// Free text dosage instructions e.g. SIG
    text: ? // string

    /// Supplemental instruction or warnings to the patient - e.g. "with meals", "may cause drowsiness"
    additionalInstruction: ? // CodeableConcept

    /// Patient or consumer oriented instructions
    patientInstruction: ? // string

    /// When medication should be administered
    timing: ? // Timing

    /// Take "as needed"
    asNeeded: ? // boolean

    /// Take "as needed" (for x)
    asNeededFor: ? // CodeableConcept

    /// Body site to administer to
    site: ? // CodeableConcept

    /// How drug should enter body
    route: ? // CodeableConcept

    /// Technique for administering medication
    method: ? // CodeableConcept

    /// Amount of medication administered, to be administered or typical amount to be administered
    doseAndRate: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// The kind of dose or rate specified
    type: ? // CodeableConcept

    /// Amount of medication per dose
    : ? // Range

    /// Amount of medication per unit of time
    : ? // Ratio

    /// Upper limit on medication per unit of time
    maxDosePerPeriod: ? // Ratio

    /// Upper limit on medication per administration
    maxDosePerAdministration: ? // Quantity

    /// Upper limit on medication per lifetime of the patient
    maxDosePerLifetime: ? // Quantity

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Dosage;

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
