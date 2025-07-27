//! Parse FHIR R5 specifications JSON file.
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

/// Share this common code with all the below specific parse modules.
pub mod aa;

//// Parse modules, one per FHIR definitions JSON file.
pub mod concept_maps;
pub mod data_elements;
pub mod profiles_others;
pub mod profiles_resources;
pub mod profiles_types;
pub mod value_sets;
pub mod version_info;

use std::path::PathBuf;
use std::sync::LazyLock;

pub static DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("r5")
        .join("parse")
});

pub static DEFINITIONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("doc")
        .join("fhir-specifications")
        .join("r5")
        .join("fhir-definitions-json")
});
