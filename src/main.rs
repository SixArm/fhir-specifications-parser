pub mod r5;
pub mod util;

use crate::r5::parse;
use std::fs::File;
use std::io::BufReader;

pub static DEFINITIONS_DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("doc")
            .join("fhir-specifications")
            .join("r5")
            .join("fhir-definitions-json")
    });

fn parse_profiles_types() {
    let path = crate::DEFINITIONS_DIR.join("profiles-types.json");
    let file = File::open(path).expect("open");
    let reader = BufReader::new(file);
    let bundle: parse::profiles_types::Bundle = ::serde_json::from_reader(reader).unwrap();
    bundle
        .entry
        .into_iter()
        .map(|resource_head| resource_head.resource)
        .for_each(|resource| {
            parse::profiles_types::resource_into_rust(&resource).expect("resource_into_rust");
        });
}

/// Literate programming.
pub type SourceCodeString = String;

fn main() {
    parse_profiles_types();
}
