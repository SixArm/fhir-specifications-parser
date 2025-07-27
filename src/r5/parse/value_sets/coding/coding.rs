//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.
//!
//! ```sh
//! ```

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Coding {
    /// TODO
    pub system: Option<String>,

    /// TODO
    pub code: Option<String>,

    /// TODO
    pub display: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coding;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("coding")
            .join("coding.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
