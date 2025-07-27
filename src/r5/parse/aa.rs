//// Include all the structs in the subdirectory.

pub mod coding {
    pub mod coding;
}
pub use coding::coding::Coding;

pub mod contact {
    pub mod contact;
}
pub use contact::contact::Contact;

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

//// Namespace conveniences

pub static DIR: std::sync::LazyLock<std::path::PathBuf> = std::sync::LazyLock::new(|| {
    super::DIR.join("aa")
});
