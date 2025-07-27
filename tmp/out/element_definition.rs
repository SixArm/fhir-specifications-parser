//! ElementDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ElementDefinition
//!
//! Version: 5.0.0
//!
//! ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension.
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
pub struct ElementDefinition {
    /// Definition of an element in a resource or extension
    ElementDefinition: ? // ?

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Extensions that cannot be ignored even if unrecognized
    modifierExtension: ? // Extension

    /// Path of the element in the hierarchy of elements
    path: ? // string

    /// xmlAttr | xmlText | typeAttr | cdaText | xhtml
    representation: ? // code

    /// Name for this particular element (in a set of slices)
    sliceName: ? // string

    /// If this slice definition constrains an inherited slice definition (or not)
    sliceIsConstraining: ? // boolean

    /// Name for element to display with or prompt for element
    label: ? // string

    /// Corresponding codes in terminologies
    code: ? // Coding

    /// This element is sliced - slices follow
    slicing: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Element values that are used to distinguish the slices
    discriminator: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// value | exists | type | profile | position
    type: ? // code

    /// Path to element value
    path: ? // string

    /// Text description of how slicing works (or not)
    description: ? // string

    /// If elements must be in same order as slices
    ordered: ? // boolean

    /// closed | open | openAtEnd
    rules: ? // code

    /// Concise definition for space-constrained presentation
    short: ? // string

    /// Full formal definition as narrative text
    definition: ? // markdown

    /// Comments about the use of this element
    comment: ? // markdown

    /// Why this resource has been created
    requirements: ? // markdown

    /// Other names
    alias: ? // string

    /// Minimum Cardinality
    min: ? // unsignedInt

    /// Maximum Cardinality (a number or *)
    max: ? // string

    /// Base definition information for tools
    base: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Path that identifies the base element
    path: ? // string

    /// Min cardinality of the base element
    min: ? // unsignedInt

    /// Max cardinality of the base element
    max: ? // string

    /// Reference to definition of content for the element
    contentReference: ? // uri

    /// Data type and Profile for this element
    type: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Data type or Resource (reference to definition)
    code: ? // uri

    /// Profiles (StructureDefinition or IG) - one must apply
    profile: ? // canonical

    /// Profile (StructureDefinition or IG) on the Reference/canonical target - one must apply
    targetProfile: ? // canonical

    /// contained | referenced | bundled - how aggregated
    aggregation: ? // code

    /// either | independent | specific
    versioning: ? // code

    /// Specified value if missing from instance
    : ? // base64Binary

    /// Implicit meaning when this element is missing
    meaningWhenMissing: ? // markdown

    /// What the order of the elements means
    orderMeaning: ? // string

    /// Value must be exactly this
    : ? // base64Binary

    /// Value must have at least these property values
    : ? // base64Binary

    /// Example value (as defined for type)
    example: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Describes the purpose of this example
    label: ? // string

    /// Value of Example (one of allowed types)
    : ? // base64Binary

    /// Minimum Allowed Value (for some types)
    : ? // date

    /// Maximum Allowed Value (for some types)
    : ? // date

    /// Max length for string type data
    maxLength: ? // integer

    /// Reference to invariant about presence
    condition: ? // id

    /// Condition that must evaluate to true
    constraint: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Target of 'condition' reference above
    key: ? // id

    /// Why this constraint is necessary or appropriate
    requirements: ? // markdown

    /// error | warning
    severity: ? // code

    /// Suppress warning or hint in profile
    suppress: ? // boolean

    /// Human description of constraint
    human: ? // string

    /// FHIRPath expression of constraint
    expression: ? // string

    /// Reference to original source of constraint
    source: ? // canonical

    /// For primitives, that a value must be present - not replaced by an extension
    mustHaveValue: ? // boolean

    /// Extensions that are allowed to replace a primitive value
    valueAlternatives: ? // canonical

    /// If the element must be supported (discouraged - see obligations)
    mustSupport: ? // boolean

    /// If this modifies the meaning of other elements
    isModifier: ? // boolean

    /// Reason that this element is marked as a modifier
    isModifierReason: ? // string

    /// Include when _summary = true?
    isSummary: ? // boolean

    /// ValueSet details if this is coded
    binding: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// required | extensible | preferred | example
    strength: ? // code

    /// Intended use of codes in the bound value set
    description: ? // markdown

    /// Source of value set
    valueSet: ? // canonical

    /// Additional Bindings - more rules about the binding
    additional: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// maximum | minimum | required | extensible | candidate | current | preferred | ui | starter | component
    purpose: ? // code

    /// The value set for the additional binding
    valueSet: ? // canonical

    /// Documentation of the purpose of use of the binding
    documentation: ? // markdown

    /// Concise documentation - for summary tables
    shortDoco: ? // string

    /// Qualifies the usage - jurisdiction, gender, workflow status etc.
    usage: ? // UsageContext

    /// Whether binding can applies to all repeats, or just one
    any: ? // boolean

    /// Map element to another set of definitions
    mapping: ? // Element

    /// Unique id for inter-element referencing
    id: ? // http://hl7.org/fhirpath/System.String

    /// Additional content defined by implementations
    extension: ? // Extension

    /// Reference to mapping declaration
    identity: ? // id

    /// Computable language of mapping
    language: ? // code

    /// Details of the mapping
    map: ? // string

    /// Comments about the mapping or its use
    comment: ? // markdown

}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ElementDefinition;

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
