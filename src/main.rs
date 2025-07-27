pub mod r5;
pub mod util;

use crate::r5::parse;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn parse_profiles_types() {
    let profiles_types_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("doc")
        .join("fhir-specifications")
        .join("r5")
        .join("fhir-definitions-json")
        .join("profiles-types.json");
    let file = File::open(profiles_types_path).expect("open");
    let reader = BufReader::new(file);
    let bundle: parse::profiles_types::bundle::Bundle =
        ::serde_json::from_reader(reader).expect("from_reader");
    bundle
        .entry
        .into_iter()
        .map(|resource_head| resource_head.resource)
        .for_each(|resource| {
            parse::profiles_types::resource_into_rust::resource_into_rust(&resource)
                .expect("resource_into_rust");
        });
}

fn main() {
    parse_profiles_types();
}
