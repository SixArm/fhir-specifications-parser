//! Parse FHIR R5 specifications JSON file `profiles-resources.json`
//!
//! **Purpose**: Profiles (constraints) on FHIR base resources
//!
//! ```json
//! {
//!   "resourceType": "Bundle",
//!   "entry": [{
//!     "resource": {
//!       "resourceType": "StructureDefinition",
//!       "id": "us-core-patient",
//!       "type": "Patient",
//!       "derivation": "constraint",
//!       "differential": {
//!         "element": [
//!           {
//!             "id": "Patient.identifier",
//!             "min": 1,
//!             "mustSupport": true
//!           },
//!           {
//!             "id": "Patient.name",
//!             "min": 1,
//!             "mustSupport": true
//!           }
//!         ]
//!       }
//!     }
//!   }]
//! }
//! ```
//!
//! **Use**: Country/organization-specific requirements on resources (US Core, UK Core)
//!

//TODO
