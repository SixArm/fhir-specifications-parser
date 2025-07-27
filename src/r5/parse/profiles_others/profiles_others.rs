//! Parse FHIR R5 specifications JSON file `profiles-others.json`
//!
//! **Purpose**: Profiles for non-resource types (extensions, operations, naming systems)
//!
//! ```json
//! {
//!   "resourceType": "Bundle",
//!   "entry": [
//!     {
//!       "resource": {
//!         "resourceType": "StructureDefinition",
//!         "type": "Extension",
//!         "id": "patient-race",
//!         "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race",
//!         "context": [{
//!           "type": "element",
//!           "expression": "Patient"
//!         }]
//!       }
//!     },
//!     {
//!       "resource": {
//!         "resourceType": "OperationDefinition",
//!         "id": "patient-everything",
//!         "code": "everything",
//!         "resource": ["Patient"],
//!         "system": false,
//!         "type": false,
//!         "instance": true
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! **Contains**:
//!
//! - Extension definitions
//! - Operation definitions
//! - Search parameter definitions
//! - Naming system definitions
//! - Capability statements
//!

//TODO
