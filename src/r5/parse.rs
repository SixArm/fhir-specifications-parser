//! Parse FHIR R5 specifications JSON files.
//!
//! Files:
//!
//! - conceptmaps.json
//! - dataelements.json
//! - profiles-others.json
//! - profiles-resources.json
//! - profiles-types.json
//! - valuesets.json
//!
//! And the version information file:
//!
//! - version.info

pub mod concept_maps;
pub mod data_elements;
pub mod profiles_others;
pub mod profiles_resources;
pub mod profiles_types;
pub mod value_sets;
pub mod version_info;
