//// Include all the structs in the subdirectory.

pub mod base {
    pub mod base;
}
pub use base::base::Base;

pub mod binding {
    pub mod binding;
}
pub use binding::binding::Binding;

pub mod bundle {
    pub mod bundle;
}
pub use bundle::bundle::Bundle;

pub mod constraint {
    pub mod constraint;
}
pub use constraint::constraint::Constraint;

pub mod differential {
    pub mod differential;
}
pub use differential::differential::Differential;

pub mod element {
    pub mod element;
    pub mod element_into_rust_struct_attribute;
}
pub use element::element::Element;
pub use element::element_into_rust_struct_attribute::element_into_rust_struct_attribute;

pub mod entry {
    pub mod entry;
}
pub use entry::entry::Entry;

pub mod example {
    pub mod example;
}
pub use example::example::Example;

pub mod extension {
    pub mod extension;
}
pub use extension::extension::Extension;

pub mod resource {
    pub mod resource;
    pub mod resource_into_rust;
}
pub use resource::resource::Resource;
pub use resource::resource_into_rust::resource_into_rust;

pub mod r#type { 
    pub mod r#type; 
} 
pub use r#type::r#type::r#Type;

pub mod snapshot {
    pub mod snapshot;
}
pub use snapshot::snapshot::Snapshot;

//// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> = std::sync::LazyLock::new(|| {
    super::DIR.join("profiles_types")
});

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> = std::sync::LazyLock::new(|| {
    crate::DEFINITIONS_DIR.join("profiles-types.json")
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

