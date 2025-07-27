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
pub struct Constraint {
    /// TODO
    pub key: String,

    /// TODO
    pub severity: String,

    /// TODO
    pub human: String,

    /// TODO
    pub expression: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Constraint;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("constraint")
            .join("constraint.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
