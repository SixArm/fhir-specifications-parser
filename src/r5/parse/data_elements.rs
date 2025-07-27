//// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::r5::parse::DIR.join("data_elements"));

pub static DEFINITIONS_FILE: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| crate::DEFINITIONS_DIR.join("dataelements.json"));

//// Submodules

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

pub mod coding {
    pub mod coding;
}
pub use coding::coding::Coding;

pub mod constraint {
    pub mod constraint;
}
pub use constraint::constraint::Constraint;

pub mod contact {
    pub mod contact;
}
pub use contact::contact::Contact;

pub mod differential {
    pub mod differential;
}
pub use differential::differential::Differential;

pub mod element {
    pub mod element;
}
pub use element::element::Element;

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

pub mod identifier {
    pub mod identifier;
}
pub use identifier::identifier::Identifier;

pub mod jurisdiction {
    pub mod jurisdiction;
}
pub use jurisdiction::jurisdiction::Jurisdiction;

pub mod meta {
    pub mod meta;
}
pub use meta::meta::Meta;

pub mod resource {
    pub mod resource;
}
pub use resource::resource::Resource;

pub mod snapshot {
    pub mod snapshot;
}
pub use snapshot::snapshot::Snapshot;

pub mod r#type {
    pub mod r#type;
}
pub use r#type::r#type::r#Type;

#[cfg(test)]
mod tests {
    use super::*;
    type T = crate::r5::parse::data_elements::Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let file = std::fs::File::open(&*DEFINITIONS_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_ne!(actual, T::default());
    }
}
