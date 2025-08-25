//! Parse FHIR R5 specifications JSON file.
//!
//! For an example see the sibling file of JSON.

use crate::r5::parse::all::*;
use crate::r5::parse::profiles_others::*;
use ::serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Bundle {
    /// # resourceType
    /// 
    /// ## Description
    /// 
    /// The `resourceType` attribute specifies the type of FHIR resource being represented. It is a mandatory element that identifies which resource schema and constraints apply to the JSON document in FHIR R5.
    /// 
    /// ## Purpose
    /// 
    /// The `resourceType` serves several critical functions:
    /// - Identifies the specific FHIR resource type for parsers and processors
    /// - Determines which validation rules and constraints apply
    /// - Enables proper routing and processing in FHIR systems
    /// - Provides context for interpreting the resource's data elements
    /// - Supports polymorphism in FHIR resource handling
    /// 
    /// ## Usage
    /// 
    /// The `resourceType` must be included in every FHIR resource as the first element. It should be used:
    /// - At the root level of every FHIR resource JSON document
    /// - When validating resources against their appropriate StructureDefinitions
    /// - In API endpoints to determine resource-specific processing logic
    /// - For content negotiation and resource type filtering
    /// 
    /// ## Data Type
    /// 
    /// **code** - A string that must exactly match one of the defined FHIR resource types. The value is:
    /// - Case-sensitive
    /// - Must be an exact match to a valid FHIR R5 resource type name
    /// - Follows PascalCase naming convention (e.g., "Patient", "Observation", "DiagnosticReport")
    /// 
    /// ## Constraints
    /// 
    /// - **Required**: Yes - Must be present in every FHIR resource
    /// - **Cardinality**: 1..1 (exactly one occurrence)
    /// - **Fixed Position**: Must be the first element in the JSON object
    /// - **Valid Values**: Must be one of the 150+ defined FHIR R5 resource types
    /// - **Case Sensitivity**: Exact case match required
    /// 
    /// ## Examples
    /// 
    /// See the accompanying `example.json` file for a complete Observation resource demonstrating the use of the `resourceType` attribute.
    /// 
    /// ## Related Keys
    /// 
    /// - `meta.profile` - Specifies which profile(s) the resource conforms to
    /// - `id` - Unique identifier for the resource instance
    /// - `meta` - Metadata about the resource
    /// - All resource-specific elements depend on the `resourceType` for their validity
    /// 
    /// ## Specification Reference
    /// 
    /// Based on FHIR R5 specification. For complete details and the full list of valid resource types, refer to the official FHIR R5 documentation for resource definitions.
    /// 
    pub resource_type: String,

    /// # id
    /// 
    /// ## Description
    /// 
    /// The `id` attribute is the logical identifier for a FHIR resource within
    /// a given context. It uniquely identifies the resource and is used for
    /// resource addressing and referencing within FHIR R5.
    /// 
    /// ## Purpose
    /// 
    /// The `id` exists to provide a unique identifier for each FHIR resource
    /// instance. This identifier is essential for:
    /// 
    /// - Resource addressing via RESTful URLs
    /// - Creating references between resources
    /// - Version control and resource tracking
    /// - Enabling resource updates and deletions
    /// 
    /// ## Usage
    /// 
    /// Use the `id` attribute when:
    /// 
    /// - Creating a new resource that needs to be uniquely identifiable
    /// - Referencing a resource from another resource
    /// - Performing CRUD operations on existing resources
    /// - Building RESTful FHIR APIs
    /// 
    /// The `id` is typically assigned by the server when a resource is created,
    /// but can be provided by the client in some scenarios.
    /// 
    /// ## Data Type
    /// 
    /// **string** - A sequence of Unicode characters with the following
    /// constraints:
    /// 
    /// - Must be between 1 and 64 characters in length
    /// - Can contain letters (A-Z, a-z), digits (0-9), hyphens (-), and periods
    ///   (.)
    /// - Must start and end with an alphanumeric character
    /// - Case sensitive
    /// 
    /// ## Constraints
    /// 
    /// - **Required**: No - The `id` is optional for resource creation but
    ///   typically assigned by servers
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Length**: 1-64 characters
    /// - **Pattern**: Must match the regex `[A-Za-z0-9\-\.]{1,64}`
    /// - **Uniqueness**: Must be unique within the context of the resource type
    ///   on a given server
    /// 
    /// ## Examples
    /// 
    /// See the accompanying `example.json` file for a complete Patient resource
    /// demonstrating the use of the `id` attribute.
    /// 
    /// ## Related Keys
    /// 
    /// - `meta.versionId` - Version identifier for the resource instance
    /// - `identifier` - Business identifiers for the resource
    /// - `fullUrl` - Absolute URL when used in bundles
    /// - `reference` - Used to reference this resource from other resources
    /// 
    /// ## Specification Reference
    /// 
    /// Based on FHIR R5 specification. For complete details, refer to the
    /// official FHIR R5 documentation for resource identity and addressing.
    /// 
    pub id: String,

    /// Example: "type" : "collection"
    #[serde(rename = "type")]
    pub r#type: String,

    /// # meta
    /// 
    /// ## Description
    /// 
    /// The `meta` attribute contains metadata about a FHIR resource that is
    /// maintained by the infrastructure. It provides information about the
    /// resource's versioning, last modification, security labels, profiles, and
    /// tags in FHIR R5.
    /// 
    /// ## Purpose
    /// 
    /// The `meta` element serves to:
    /// - Track resource versioning and modification history
    /// - Specify which profiles the resource claims to conform to
    /// - Apply security labels and access control information
    /// - Provide tags for categorization and workflow management
    /// - Enable optimistic locking through version control
    /// - Support provenance and audit requirements
    /// 
    /// ## Usage
    /// 
    /// Use the `meta` attribute to:
    /// - Track when resources were last updated
    /// - Specify profile conformance for validation
    /// - Apply security classifications to resources
    /// - Tag resources for workflow or categorization purposes
    /// - Enable version-aware updates and conflict detection
    /// - Support system-level metadata requirements
    /// 
    /// The `meta` element is typically managed by the server infrastructure,
    /// though clients may provide some elements.
    /// 
    /// ## Data Type
    /// 
    /// **Meta** - A complex data type containing the following optional
    /// sub-elements:
    /// - `versionId`: string - Version identifier for the resource
    /// - `lastUpdated`: instant - When the resource was last updated  
    /// - `source`: uri - Identifies where the resource came from
    /// - `profile`: array of canonical URIs - Profiles this resource claims to
    ///   conform to
    /// - `security`: array of Coding - Security labels applied to the resource
    /// - `tag`: array of Coding - Tags applied to the resource for
    ///   categorization
    /// 
    /// ## Constraints
    /// 
    /// - **Required**: No - The entire `meta` element is optional
    /// - **Cardinality**: 0..1 (zero to one occurrence)
    /// - **Server Managed**: Most sub-elements are controlled by the server
    /// - **versionId**: Must change when resource content changes
    /// - **lastUpdated**: Must be updated when resource content changes
    /// - **profile**: Must reference valid StructureDefinition resources
    /// 
    /// ## Examples
    /// 
    /// See the accompanying `example.json` file for a complete Practitioner
    /// resource demonstrating comprehensive use of the `meta` attribute.
    /// 
    /// ## Related Keys
    /// 
    /// - `id` - Resource identifier that the meta information describes
    /// - `resourceType` - Resource type that determines applicable profiles
    /// - `extension` - May contain additional metadata not covered by meta
    /// - Bundle entries use `meta` for version control during transactions
    /// 
    /// ## Specification Reference
    /// 
    /// Based on FHIR R5 specification. For complete details on metadata
    /// management, versioning, and security labeling, refer to the official
    /// FHIR R5 documentation.    
    ///     
    pub meta: Meta,

    /// Example "entry": [{"fullUrl": …, "resource": …}]
    pub entry: Vec<Entry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Bundle;

    #[test]
    fn test_serde_json_from_reader() {
        let path = crate::r5::parse::profiles_others::DIR
            .join("bundle")
            .join("bundle.json");
        let file = std::fs::File::open(path).expect("open");
        let reader = std::io::BufReader::new(file);
        let actual: T = ::serde_json::from_reader(reader).unwrap();
        assert_eq!(actual.id, "types");
    }
}
