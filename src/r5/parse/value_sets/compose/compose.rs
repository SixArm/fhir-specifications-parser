//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Compose {
    /// TODO
    pub include: Vec<::serde_json::Value>,

    /// TODO
    pub exclude: Option<Vec<::serde_json::Value>>,

    /// TODO
    pub property: Option<Vec<String>>,

    /// Example: "2012-06-13",
    pub locked_date: Option<String>,

    /// TODO
    pub inactive: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Compose;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::value_sets::DIR
            .join("compose")
            .join("compose.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
