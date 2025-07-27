//! Timing
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Timing
//!
//! Version: 5.0.0
//!
//! Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.
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
pub struct Timing {
    /// A timing schedule that specifies an event that may occur multiple times
    Timing: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Extensions that cannot be ignored even if unrecognized
    modifierExtension: ? // Extension

    /// When the event occurs
    event: ? // dateTime

    /// When the event is to occur
    repeat: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Length/Range of lengths, or (Start and/or end) limits
    : ? // Duration

    /// Number of times to repeat
    count: ? // positiveInt

    /// Maximum number of times to repeat
    countMax: ? // positiveInt

    /// How long when it happens
    duration: ? // decimal

    /// How long when it happens (Max)
    durationMax: ? // decimal

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    durationUnit: ? // code

    /// Indicates the number of repetitions that should occur within a period. I.e. Event occurs frequency times per period
    frequency: ? // positiveInt

    /// Event occurs up to frequencyMax times per period
    frequencyMax: ? // positiveInt

    /// The duration to which the frequency applies. I.e. Event occurs frequency times per period
    period: ? // decimal

    /// Upper limit of period (3-4 hours)
    periodMax: ? // decimal

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    periodUnit: ? // code

    /// mon | tue | wed | thu | fri | sat | sun
    dayOfWeek: ? // code

    /// Time of day for action
    timeOfDay: ? // time

    /// Code for time period of occurrence
    when: ? // code

    /// Minutes from event (before or after)
    offset: ? // unsignedInt

    /// C | BID | TID | QID | AM | PM | QD | QOD | +
    code: ? // CodeableConcept

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Timing;

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
