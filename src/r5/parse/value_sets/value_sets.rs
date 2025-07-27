//! Parse FHIR specificstions JSON file `valuesets.json``
//!
//! **Purpose**: Defines allowed code sets for coded fields
//!
//! ```json
//! {
//!   "resourceType": "Bundle",
//!   "entry": [{
//!     "resource": {
//!       "resourceType": "ValueSet",
//!       "id": "marital-status",
//!       "url": "http://hl7.org/fhir/ValueSet/marital-status",
//!       "compose": {
//!         "include": [{
//!           "system": "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus",
//!           "concept": [
//!             {"code": "M", "display": "Married"},
//!             {"code": "S", "display": "Single"},
//!             {"code": "D", "display": "Divorced"}
//!           ]
//!         }]
//!       }
//!     }
//!   }]
//! }
//! ```
//!
//! **Use**: Validation of coded fields, dropdown lists in UIs
//!

//TODO
