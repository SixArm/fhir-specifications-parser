use crate::r5::parse::profiles_types::*;
use crate::util::*;
use ::convert_case::{Case, Casing};
use ::indoc::formatdoc;
use std::path::PathBuf;

/// Literate programming.
type SourceCodeString = String;

/// Indent the start of each Rust attribute line of source code
const RUST_ATTRIBUTE_INDENT: &str = "        ";

/// FHIR resource => Rust source code file.
/// 
/// Example:
/// 
/// ```no_run
/// let resource = … // e.g. resource id AlfaBravo.
/// let result = resource_into_rust(&resource);
/// ```
/// 
/// Outcome: a Rust source code file at `./tmp/out/alfa_bravo.rs`.
/// 
#[allow(dead_code)]
pub fn resource_into_rust(resource: &Resource) -> std::io::Result<()> {
    println!(
        "{} {} {}",
        resource.id,
        resource.id.to_case(Case::Pascal),
        resource.id.to_case(Case::Snake),
    );
    std::fs::write(
        resource_into_rust_struct_path(&resource),
        resource_into_rust_struct_block(&resource),
    )
}

/// FHIR resource => Rust struct file path.
/// 
/// Example:
/// 
/// ```no_run
/// let resource = … // e.g. resource id AlfaBravo.
/// let path_buf = resource_into_rust_struct_path(&resource);
/// ```
/// 
/// Output: `./tmp/out/alfa_bravo.rs`.
/// 
#[allow(dead_code)]
pub fn resource_into_rust_struct_path(resource: &Resource) -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("tmp")
        .join("out")
        .join(format!(
            "{}.rs",
            resource.id.from_case(Case::Pascal).to_case(Case::Snake)
        ))
}

/// FHIR resource => Rust struct block of source code.
/// 
/// Example:
/// 
/// ```no_run
/// let resource = … // e.g. resource id AlfaBravo.
/// let source_code_string = resource_into_rust_struct(&resource);
/// ```
/// 
/// Output: all the Rust struct source code, including comments,
/// mods, attributes, etc. such that the source code can compile.
/// 
#[allow(dead_code)]
pub fn resource_into_rust_struct_block(resource: &Resource) -> SourceCodeString {
    let question = String::from("?");
    formatdoc!(
        r#"//! {id_pascal_case}
        //!
        //! URL: {url}
        //! 
        //! Version: {version}
        //! 
        //! {description}
        //! 
        //! FHIR: <https://build.fhir.org/>
        //!
        //! UML: <https://build.fhir.org/uml.html>

        // Allow unused crate::r5::types as types;
        #![allow(unused_imports)]

        use crate::r5::types;
        use ::serde::{{Deserialize, Serialize}};

        #[serde_with::skip_serializing_none]
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct {id_pascal_case} {{
            {attribute_block}
        }}

        #[cfg(test)]
        mod tests {{
            use super::*;
            type T = {id_pascal_case};

            #[test]
            fn test_default() {{
                let actual = T::default();
                let expect = T {{}};
                assert_eq!(actual, expect);
            }}

            mod serde_json {{
                use super::*;
                use ::serde_json::json;

                #[test]
                fn test_serde_json_from_value() {{
                    let json = json!({{}});
                    let actual: T = ::serde_json::from_value(json).expect("from_value");
                    let expect: T = T::default();
                    assert_eq!(actual, expect);
                }}

                #[test]
                fn test_serde_json_to_value() {{
                    let actual: ::serde_json::Value =
                        ::serde_json::to_value(T::default()).expect("to_value");
                    let expect: ::serde_json::Value = json!({{}});
                    assert_eq!(actual, expect);
                }}
            }}
        }}
        "#,
        id_pascal_case = resource.id.to_case(Case::Pascal),
        url = resource.url.as_ref().unwrap_or(&question),
        version = resource.version.as_ref().unwrap_or(&question),
        description = resource.description.as_ref().unwrap_or(&question),
        attribute_block = resource_into_rust_struct_attribute_block(resource),
    )
}

/// FHIR resource => Rust struct attribute block of source code.
///
/// Example:
/// 
/// ```no_run
/// let resource = … // e.g. resource id AlfaBravo.
/// let attribute_block = resource_into_into_rust_struct_attribute_block(&resource);
/// ```
/// 
/// Output is approximately like this also with indentation:
/// 
/// ```no_run
/// alfa: int,
/// bravo: int,
/// ```
/// 
#[allow(dead_code)]
pub fn resource_into_rust_struct_attribute_block(resource: &Resource) -> SourceCodeString {
    if resource.id == "Narrative" {
        println!("{:?}", resource);    
    }
    match resource.snapshot.as_ref() {
        Some(snapshot) =>
            snapshot
            .element
            .iter()
            .map(element_into_rust_struct_attribute)
            .collect::<Vec<String>>()
            .join("\n"),
        None => String::from("")
    }
}

/// Given one element, generate Rust struct attribute source code.
#[allow(dead_code)]
pub fn element_into_rust_struct_attribute(element: &Element) -> SourceCodeString {
    let name = last_word(&element.id);
    format!(
        "{}/// {}\n{}{}\n", 
        RUST_ATTRIBUTE_INDENT,
        element.short.as_ref().unwrap_or(&String::from("Short description goes here.")),
        RUST_ATTRIBUTE_INDENT, 
        name,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_into_rust() {
        let resource = Resource::default();
        let actual = resource_into_rust(&resource);
        assert!(actual.is_ok());
    }

    #[test]
    fn test_resource_into_rust_struct_attribute_block() {
        let resource = Resource {
            snapshot: Some(
                Snapshot {
                    element: vec![
                        Element {
                            id: "Foo.alfa".into(),
                            short: Some(String::from("Short comment 0")),
                            ..Element::default()
                        },
                        Element {
                            id: "Foo.bravo".into(),
                            short: Some(String::from("Short comment 1")),
                            ..Element::default()
                        }
                    ]
                }
            ),
            ..Resource::default()
        };
        let actual = resource_into_rust_struct_attribute_block(&resource);
        let expect = concat!(
            "        /// Short comment 0\n",
            "        alfa\n",
            "\n",
            "        /// Short comment 1\n",
            "        bravo\n",
        );
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_element_into_rust_struct_attribute() {
        let element = Element { 
            id: "Alfa.bravo".into(),
            short: Some(String::from("Short comment")),
            ..Element::default()
        };
        let actual = element_into_rust_struct_attribute(&element);
        let expect = concat!(
            "        /// Short comment\n",
            "        bravo\n",
        );
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_resource_into_rust_struct_path() {
        let resource = Resource {
            id: "AlfaBravoCharlie".into(),
            ..Resource::default()
        };
        let actual = resource_into_rust_struct_path(&resource);
        assert!(
            actual
                .to_string_lossy()
                .ends_with("/tmp/out/alfa_bravo_charlie.rs"),
            "{:?}",
            actual
        );
    }

    #[test]
    fn test_resource_into_rust_struct_block() {
        let resource = Resource {
            id: "AlfaBravoCharlie".into(),
            ..Resource::default()
        };
        let actual = resource_into_rust_struct_block(&resource);
        assert!(actual.contains("//! AlfaBravoCharlie\n"), "{}", actual);
    }
}
