use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use indexmap::IndexMap;
/// Records are the "model"s of the MIR world. model is a crazy overloaded word though.
use openapiv3::{
    ObjectType, OpenAPI, RefOrMap, ReferenceOr, Schema, SchemaData, SchemaKind, SchemaReference,
    StringType, Type,
};
use tracing::warn;

use hir::{HirField, HirSpec, NewType, Record, StrEnum, Struct};
use mir::Doc;

use crate::child_schemas::ChildSchemas;
use crate::extractor;
use crate::extractor::{schema_ref_to_ty_already_resolved, schema_to_ty};

fn properties_to_fields(
    properties: &RefOrMap<Schema>,
    schema: &Schema,
    spec: &OpenAPI,
) -> BTreeMap<String, HirField> {
    properties
        .iter()
        .map(|(name, field_schema_ref)| {
            let field_schema = field_schema_ref.resolve(spec);
            println!(
                "trying to resolve ty for: {}, ref: {:?}",
                name, field_schema_ref
            );
            let ty = schema_ref_to_ty_already_resolved(field_schema_ref, spec, field_schema);
            let optional = extractor::is_optional(name, field_schema, schema);
            (
                name.clone(),
                HirField {
                    ty,
                    optional,
                    doc: extractor::extract_schema_docs(field_schema),
                    example: None,
                    flatten: false,
                },
            )
        })
        .collect()
}

pub fn effective_length(all_of: &[ReferenceOr<Schema>]) -> usize {
    let mut length = 0;
    for schema_ref in all_of {
        length += schema_ref.as_ref_str().map(|_s| 1).unwrap_or_default();
        length += schema_ref
            .as_item()
            .map(|s| s.properties())
            .map(|s| s.iter().len())
            .unwrap_or_default();
    }
    length
}

pub fn create_record(name: &str, schema: &Schema, spec: &OpenAPI) -> Record {
    let name = name.to_string();
    match &schema.kind {
        // The base case, a regular object
        SchemaKind::Type(Type::Object(ObjectType { properties, .. })) => {
            let fields = properties_to_fields(properties, schema, spec);
            Record::Struct(Struct {
                name,
                fields,
                nullable: schema.nullable,
                docs: schema
                    .description
                    .as_ref()
                    .map(|d| Doc(d.trim().to_string())),
            })
        }
        // An enum
        SchemaKind::Type(Type::String(StringType { enumeration, .. }))
            if !enumeration.is_empty() =>
        {
            Record::Enum(StrEnum {
                name,
                variants: enumeration.iter().map(|s| s.to_string()).collect(),
                docs: schema.description.as_ref().map(|d| Doc(d.clone())),
            })
        }
        // A newtype with multiple fields
        SchemaKind::AllOf { all_of } => {
            let all_of = all_of.as_slice();
            if effective_length(all_of) == 1 {
                Record::TypeAlias(
                    name,
                    HirField {
                        ty: schema_ref_to_ty_already_resolved(&all_of[0], spec, schema),
                        optional: schema.nullable,
                        ..HirField::default()
                    },
                )
            } else {
                create_record_from_all_of(&name, all_of, &schema.data, spec)
            }
        }
        // Default case, a newtype with a single field
        _ => Record::NewType(NewType {
            name,
            fields: vec![HirField {
                ty: schema_to_ty(schema, spec),
                optional: schema.nullable,
                doc: None,
                example: None,
                flatten: false,
            }],
            docs: schema.description.as_ref().map(|d| Doc(d.clone())),
        }),
    }
}

fn create_field(field_schema_ref: &ReferenceOr<Schema>, spec: &OpenAPI) -> HirField {
    let field_schema = field_schema_ref.resolve(spec);
    let ty = schema_ref_to_ty_already_resolved(field_schema_ref, spec, field_schema);
    let optional = field_schema.nullable;
    let example = field_schema.example.clone();
    let doc = field_schema.description.clone().map(Doc);
    HirField {
        ty,
        optional,
        doc,
        example,
        flatten: false,
    }
}

fn create_record_from_all_of(
    name: &str,
    all_of: &[ReferenceOr<Schema>],
    schema_data: &SchemaData,
    spec: &OpenAPI,
) -> Record {
    let mut fields = BTreeMap::new();
    for schema in all_of {
        match &schema {
            ReferenceOr::Reference { reference } => {
                let schema_ref = SchemaReference::from_str(reference);
                let name = extractor::get_name(schema_ref);
                let mut field = create_field(schema, spec);
                field.flatten = true;
                fields.insert(name, field);
            }
            ReferenceOr::Item(item) => {
                let props = item.properties();
                for (name, schema) in props {
                    let mut field = create_field(schema, spec);
                    if !field.ty.is_iterable() && !item.required().iter().any(|s| s == name) {
                        field.optional = true;
                    }
                    fields.insert(name.to_string(), field);
                }
            }
        }
    }
    Record::Struct(Struct {
        nullable: schema_data.nullable,
        name: name.to_string(),
        fields,
        docs: schema_data.description.as_ref().map(|d| Doc(d.clone())),
    })
}

// records are data types: structs, newtypes
pub fn extract_records(spec: &OpenAPI, result: &mut HirSpec) -> Result<()> {
    let mut schema_lookup = HashMap::new();

    spec.add_child_schemas(&mut schema_lookup);
    for (mut name, schema) in schema_lookup {
        println!("creating record for name: {}, ", name);
        let rec = create_record(&name, schema, spec);
        let name = rec.name().to_string();
        result.schemas.insert(name, rec);
    }

    for (name, schema_ref) in &spec.schemas {
        let Some(reference) = schema_ref.as_ref_str() else {
            continue;
        };
        result.schemas.insert(
            name.clone(),
            Record::TypeAlias(name.clone(), create_field(&schema_ref, spec)),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use openapiv3::{OpenAPI, Schema, SchemaData, SchemaKind};

    use crate::extractor::record::create_record_from_all_of;

    #[test]
    fn test_all_of_required_set_correctly() {
        let mut additional_props: Schema =
            serde_yaml::from_str(include_str!("./pet_tag.yaml")).unwrap();
        let SchemaKind::AllOf { all_of } = &additional_props.kind else {
            panic!()
        };
        let spec = OpenAPI::default();
        let rec = create_record_from_all_of("PetTag", &all_of, &SchemaData::default(), &spec);
        let mut fields = rec.fields();
        let eye_color = fields.next().unwrap();
        let weight = fields.next().unwrap();
        assert_eq!(eye_color.optional, false);
        assert_eq!(weight.optional, true);
    }
}
