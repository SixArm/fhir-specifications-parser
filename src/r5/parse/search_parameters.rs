//// Include all the structs in the subdirectory.

pub mod bundle {
    pub mod bundle;
}

pub mod entry {
    pub mod entry;
}

//// Namespace conveniences

pub use bundle::bundle::Bundle;
pub use meta::meta::Meta;
pub use entry::entry::Entry;

pub static DIR: std::sync::LazyLock<std::path::PathBuf> = std::sync::LazyLock::new(|| {
    super::DIR.join("profiles_resources")
});

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> = std::sync::LazyLock::new(|| {
    crate::DEFINITIONS_DIR.join("search-parameters.json")
});

#[cfg(test)]
mod tests {
    use super::*;
    type T = Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual,  T::default());
    }

}

