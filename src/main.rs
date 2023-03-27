mod utils;
use crate::utils::{DefinitionResult, DefinitionField, DefinitionValue, generate_entities_page};
use graphql_parser::schema::parse_schema;
use graphql_parser::schema::TypeDefinition::{Scalar, Object, Interface, Union, Enum, InputObject};
use graphql_parser::schema::Definition::{SchemaDefinition, TypeDefinition, TypeExtension, DirectiveDefinition};
use graphql_parser::query::Type;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>  {
    // Read testing-schema.graphql
    let contents = std::fs::read_to_string("./testing-schema.graphql")
    .expect("Should have been able to read the file");

    // Parse schema
    let ast = parse_schema::<String>(&contents)?.into_static();

    // Traverse type definition options with match
    let mut results: Vec<DefinitionResult> = Vec::new();

    for definition in ast.definitions {
        match definition {
            TypeDefinition(Object(o)) => {
                let mut tmp_fields: Vec<DefinitionField> = Vec::new();
                for field in o.fields {
                    tmp_fields.push(
                        DefinitionField {
                            name: field.name,
                            description: field.description,
                            field_type: field.field_type,
                        }
                    );
                }
                let mut tmp_values: Vec<DefinitionValue> = Vec::new();
                tmp_values.push(
                    DefinitionValue {
                        name: "None".to_string(),
                        description: None,
                    }
                );
                results.push(
                    DefinitionResult {
                        name: o.name,
                        description: o.description,
                        fields: tmp_fields,
                        values: tmp_values,
                    }
                );
            },
            TypeDefinition(Enum(e)) => {
                let mut tmp_values: Vec<DefinitionValue> = Vec::new();
                for value in e.values {
                    tmp_values.push(
                        DefinitionValue {
                            name: value.name,
                            description: value.description,
                        }
                    );
                }
                let mut tmp_fields: Vec<DefinitionField> = Vec::new();
                tmp_fields.push(
                    DefinitionField {
                        name: "None".to_string(),
                        description: None,
                        field_type: Type::NamedType("None".to_string()),
                    }
                );
                results.push(
                    DefinitionResult {
                        name: e.name,
                        description: e.description,
                        fields: tmp_fields,
                        values: tmp_values,
                    }
                );
            },
            TypeDefinition(Interface(i)) => {
                let mut tmp_fields: Vec<DefinitionField> = Vec::new();
                for field in &i.fields {
                    tmp_fields.push(
                        DefinitionField {
                            name: field.name.clone(),
                            description: field.description.clone(),
                            field_type: field.field_type.clone(),
                        }
                    );
                }
                let mut tmp_values: Vec<DefinitionValue> = Vec::new();
                tmp_values.push(
                    DefinitionValue {
                        name: "None".to_string(),
                        description: None,
                    }
                );
                results.push(
                    DefinitionResult {
                        name: i.name.clone(),
                        description: i.description.clone(),
                        fields: tmp_fields,
                        values: tmp_values,
                    }
                );
            },
            TypeDefinition(Scalar(s)) => {
                println!("Coming soon");
            },
            TypeDefinition(Union(u)) => {
                println!("Coming soon");
            },
            TypeDefinition(InputObject(io)) => {
                println!("Coming soon");
            },
            SchemaDefinition(_) | TypeExtension(_) | DirectiveDefinition(_) => todo!(),
        }
    }


    // Generate markdown from parsed entities
    generate_entities_page(&results)?;

    println!("Subgraph documentation is ready");

    // Print results
    // for res in results {
    //     println!("{:?}", res);
    //     println!("");
    // }

    Ok(())
}

