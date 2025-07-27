//! Parse FHIR R5 specifications JSON file `search-parameters.json``
//!
//! **Purpose**: Defines searchable fields and how to query them
//!
//! ```json
//! {
//!   "resourceType": "Bundle",
//!   "entry": [{
//!     "resource": {
//!       "resourceType": "SearchParameter",
//!       "id": "patient-birthdate",
//!       "code": "birthdate",
//!       "base": ["Patient"],
//!       "type": "date",
//!       "expression": "Patient.birthDate",
//!       "comparator": ["eq", "ne", "gt", "lt", "ge", "le"]
//!     }
//!   }]
//! }
//! ```
//!
//! **Use**: Enables REST API queries like `GET /Patient?birthdate=1990-01-01`
//!

//TODO
