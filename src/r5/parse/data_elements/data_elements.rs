//! Parse FHIR R5 specifications JSON file `dataelements.json`
//!
//! **Purpose**: Reusable data field definitions with validation rules
//!
//! ```json
//! {
//!   "resourceType": "Bundle",
//!   "entry": [{
//!     "resource": {
//!       "resourceType": "DataElement",
//!       "id": "blood-pressure-systolic",
//!       "element": [{
//!         "path": "systolic",
//!         "type": [{"code": "integer"}],
//!         "min": 1,
//!         "max": "1",
//!         "minValueInteger": 0,
//!         "maxValueInteger": 300
//!       }]
//!     }
//!   }]
//! }
//! ```
//!
//! **Use**: Standardized data dictionary for forms, questionnaires, and data collection
//!

//TODO
