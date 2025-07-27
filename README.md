# FHIR specifications parser

<https://build.fhir.org/versions.html#std-process>

Fast Healthcare Interoperability Resources (FHIR) parser for FHIR Release 5 (R5).

This is a work in progress. This is aiming to be usable before end of 2025.

This crate has these notable areas:

```txt
- bin: Binaries e.g. a shell script that creates files
- src: All the source code
  - r5: FHIR Release 5
   - parse: Parse each FHIR Release 5 specification JSON file
   - abstract_types: Examples to verify the code is working
   - primitive_types: Examples to verify the code is working
   - complex_types: Examples to verify the code is working
   - properties: Work in progress for JSON parsing
```

## FHIR R5 element URL

Any element defined in any version of FHIR is automatically assigned an
extension URL that uniquely identifies the element and can be used in the
relevant FHIR version. The extension URL for an element can automatically be
derived:

<http://hl7.org/fhir/[version]/StructureDefinition/extension-[Path]>

## FHIR R5 Datatypes

The FHIR R5 datatypes live in FHIR specifications JSON file `profiles-types.json`.

The file specifies primitive types, complex types, and more.

- Primitive types always start with lowercase.

- Complex types always start with uppercase.

### Datatypes that are primitive types

Parse example for primitive-type id list:

```sh
<profiles-types.json |
jq -r '.entry | map(select(.resource.kind == "primitive-type")) | map(.resource.id)[]'
```

Primitive type id list:

- base64Binary
- boolean
- canonical
- code
- date
- dateTime
- decimal
- id
- instant
- integer
- integer64
- markdown
- oid
- positiveInt
- string
- time
- unsignedInt
- uri
- url
- uuid

### Datatypes that are complex types

Parse example for complex-type id list:

```sh
<profiles-types.json |
jq -r '.entry | map(select(.resource.kind == "complex-type")) | map(.resource.id)[]'
```

General-purpose data types:

- Address
- Age
- Annotation
- Attachment
- BackboneType
- CodeableConcept
- Coding
- ContactPoint
- Count
- Distance
- Duration
- HumanName
- Identifier
- Money
- MoneyQuantity
- Period
- Quantity
- Range
- Ratio
- RatioRange
- RelativeTime
- SampledData
- Signature
- SimpleQuantity
- Timing

Meta data types:

- Availability
- ContactDetail
- Contributor
- DataRequirement
- Expression
- ExtendedContactDetail
- MonetaryComponent
- ParameterDefinition
- RelatedArtifact
- TriggerDefinition
- UsageContext
- VirtualServiceDetail

Special purpose data types:

- BackboneType
- CodeableReference
- Dosage
- ElementDefinition
- Extension
- Meta
- Narrative
- Reference
- xhtml

Hierarchy view:

```json
{
    "Element": {
        "BackboneElement": {},
        "xhtml": {},
        "DataType": {
            "Address": {},
            "Annotation": {},
            "Attachment": {},
            "BackboneType": {
                "Dosage": {},
                "ElementDefinition": {},
                "MarketingStatus": {},
                "OrderedDistribution": {},
                "Population": {},
                "ProdCharacteristic": {},
                "ProductShelfLife": {},
                "Statistic": {},
                "SubstanceAmount": {},
                "Timing": {}
            },
            "CodeableConcept": {},
            "CodeableReference": {},
            "Coding": {},
            "ContactDetail": {},
            "ContactPoint": {},
            "Contributor": {},
            "DataRequirement": {},
            "Expression": {},
            "Extension": {},
            "HumanName": {},
            "Identifier": {},
            "Meta": {},
            "Money": {},
            "Narrative": {},
            "ParameterDefinition": {},
            "Period": {},
            "PrimitiveType": {
                "base64Binary": {},
                "boolean": {},
                "date": {},
                "dateTime": {},
                "decimal": {},
                "instant": {},
                "integer": {
                    "positiveInt": {},
                    "unsignedInt": {}
                },
                "integer64": {},
                "string": {
                    "code": {},
                    "id": {},
                    "markdown": {}
                },
                "time": {},
                "uri": {
                    "canonical": {},
                    "oid": {},
                    "url": {},
                    "uuid": {}
                }
            },
            "Quantity": {
                "Age": {},
                "Count": {},
                "Distance": {},
                "Duration": {},
                "Quantity": {}
            },
            "Range": {},
            "Ratio": {},
            "Reference": {},
            "RelatedArtifact": {},
            "SampledData": {},
            "Signature": {},
            "TriggerDefinition": {},
            "UsageContext": {}
        }
    }
}
```

### Parse into snake case

We also want the output as snake case so we use the toolbox [change-case](https://github.com/sixarm/change-case) and specifically the command `snake-case`:

```sh
<profiles-types.json jq -r '.entry | map(select(.resource.kind == "primitive-type")) | map(.resource.id)[]' | snake-case
<profiles-types.json jq -r '.entry | map(select(.resource.kind == "complex-type")) | map(.resource.id)[]' | snake-case
```

## Types

Each type is represented as a class with a name and an ancestor class (except for Base, which has no ancestor). In addition, types may be marked as abstract, or assigned stereotypes that describe how they used.

Classes also have a zero or more attributes defined, where each attribute has the following properties:

- name: the name of the attribute

- type: the type of the attribute - either another type defined in the speification, or (for primitive types) a type from XML Schema
[cardinality]: [min..max] control over the attribute cardinality. Attributes with Max cardinality >1 (usually *) are ordered, though the meaning of the order might not be known or defined

- « stereotypes »: these provide additional detail about the element - see below

In addition, classes have zero or more associations, which are always aggregations, and have the following properties:

- name: the name of the association (which is the name of the element that represents it in XML/JSON)

- [cardinality]: [min..max] control over the association cardinality. Associations with Max cardinality >1 (usually *) are ordered, though the meaning of the order might not be known or defined

## Documentation links

FHIR datatypes: <https://build.fhir.org/datatypes.html>

- Regex

- XML representation

- JSON representation

More:

- [UML](https://build.fhir.org/uml.html)
- [References](https://build.fhir.org/references.html)
- [Extensibility](https://build.fhir.org/extensibility.html)
- [Narrative](https://build.fhir.org/narrative.html)
- [Resource](https://build.fhir.org/resource.html)

## profiles-types.json

Fast Healthcare Interoperability Resources (FHIR, pronounced "fire") is a standard created by Health Level Seven International (HL7) for the exchange of electronic health records. The FHIR Profiles-Types JSON file is a schema that provides a guide to the structure and data types used in FHIR. This schema offers a blueprint for each type of resource, defining its properties, attributes, and their data types.

### Dataset Structure

The FHIR Profiles-Types file is structured in JSON, a versatile, human-readable data format. Each JSON object corresponds to a FHIR resource type. It describes the properties that the corresponding resources in FHIR should have, as well as the data type for each property.

### Fields Description

The exact properties described in the file vary for each FHIR resource type, but they typically include:

- Id: The unique identifier for the resource type.
- Url: A URI that identifies the type globally.
- Version: The business version of the type.
- Name: The human-friendly name of the type.
- Status: The publication status of the type (draft, active, retired).
- Experimental: A boolean value to indicate if this is an experimental type.
- Date: The date when the type was last changed.
- Publisher: The name of the individual or organization that published the type.
- Contact: Contact details of the publishers.
- Description: A free text natural language description of the type.
- UseContext: A list of usability context for the type.
- Jurisdiction: Indicates the country/region for which the type is defined.
- Purpose: Explains why the type is needed.
- Element: A list defining the structure and data types of properties for the resource.
- Potential Use Cases
- Schema Validation: Use the schema to validate FHIR data and ensure it adheres to the defined structure and data types.
- Interoperability: Aid in the exchange of healthcare information with other FHIR-compatible systems by providing a standard structure.
- Data Mapping: Use the schema to map data from other formats into FHIR format or vice versa.
System Design: Help in the design and development of healthcare systems by providing a template for data structure.

## FHIR profiles: snapshot view versus differential view

In FHIR profiles, snapshot and differential are two different views of the profiled resource. The snapshot represents the complete, final structure of the profile after applying all changes from the differential to the base resource. The differential, on the other hand, shows only the differences or constraints introduced by the profile compared to the base resource. 

### Snapshot View

The snapshot view provides a comprehensive, hierarchical representation of the profiled resource, incorporating all the changes defined in the differential. It essentially shows the fully realized structure of the profile, including inherited elements from the base resource and the specific constraints or additions defined in the profile. This view is useful when you need a complete, self-contained definition of the resource, especially if you don't have direct access to the base resource.

### Differential View

The differential view highlights the specific differences between the profiled resource and its base resource. It focuses on the elements that have been modified, added, or removed by the profile. This view is particularly helpful for understanding how a profile customizes a base resource and for making informed decisions about implementing the profile. It allows data analysts and developers to easily identify the key changes introduced by the profile, simplifying the process of understanding and implementing it. 

### Example

Imagine a FHIR profile for "Laboratory Observation" derived from the general "Observation" resource. The differential view might show that the profile adds a new element for "laboratory test result" or changes the cardinality of an existing element, while the snapshot view would display the complete structure of the "Laboratory Observation" resource, including all the inherited elements from "Observation" and the newly added or modified elements. 

## jq commands

List keys:

```sh
cat profiles-types.json | jq -r 'keys_unsorted[]'
```

Output:

```stdout
resourceType
id
meta
type
entry
```

List keys of a inner area such as a bundle that contains an entry list item that contains a resource item:

```sh
cat profiles-types.json | jq -r '.entry[0] | .resource | keys_unsorted[]'
```

Count  keys of a inner area such as a bundle that contains an entry list item that contains a resource item:

```sh
cat profiles-types.json | jq -r '.entry[] | .resource | keys_unsorted[]' | sort | uniq -c 
```

## FHIR JSON

<https://build.fhir.org/json.html>
