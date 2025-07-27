//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! ```sh
//! ```
use crate::r5::parse::search_parameters::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Jurisdiction {
    /// Example: [{"system": "…", "code" : "…", "display" : "…"}]
    pub coding: Vec<Coding>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Jurisdiction;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::search_parameters::DIR
            .join("jurisdiction")
            .join("jurisdiction.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual, T::default());
    }
}
