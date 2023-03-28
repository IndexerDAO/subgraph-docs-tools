use std::string::String;
use std::fs::File;
use std::io::Write;
use std::error::Error;


use graphql_parser::schema::TypeDefinition::{Scalar, Object, Interface, Union, Enum, InputObject};
use graphql_parser::schema::Definition::{SchemaDefinition, TypeDefinition, TypeExtension, DirectiveDefinition};
use graphql_parser::query::Type;
use graphql_parser::schema::Document;


#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DefinitionResult<'a> {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<DefinitionField<'a>>,
    pub values: Vec<DefinitionValue>,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DefinitionField<'a> {
    pub name: String,
    pub description: Option<String>,
    pub field_type: Type<'a, String>,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DefinitionValue {
    pub name: String,
    pub description: Option<String>,
}


pub fn generate_entities_page(schema_definitions: &Vec<DefinitionResult<'static>>) -> Result<(), Box<dyn std::error::Error>> {

    let path = "entities.md";

    let mut markdown = String::from("# Subgraph Entities\n");

    for def in schema_definitions {
        markdown.push_str(&format!("* [{}](#{})\n", def.name, def.name.to_lowercase()));
    }

    markdown.push_str("\n");

    for def in schema_definitions {
        markdown.push_str(&format!("## {}\n", def.name));
        match &def.description {
            Some(d) => markdown.push_str(&format!("{}\n", d)),
            None => (),
        }
        markdown.push_str("\n");

        markdown.push_str("| Field/Value | Type | Description | \n");
        markdown.push_str("| --- | --- | --- | \n");
    

    // if fields not the null placeholder
        // add fields to entities table
        for field in &def.fields {
            if field.name != "None" {

                match &field.description {
                    Some(d) => markdown.push_str(&format!("| {} | {} | {} | \n", field.name, field.field_type, d)),
                    None => markdown.push_str(&format!("| {} | {} | | \n", field.name, field.field_type)),
                }
            }
        }

        // if values not the null placeholder
        // add values to entities table
        for value in &def.values {
            if value.name != "None" {
                match &value.description {
                    Some(d) => markdown.push_str(&format!("| {} | | {} | \n", value.name, d)),
                    None => markdown.push_str(&format!("| {} | | | \n", value.name)),
                }
            }
        }

        markdown.push_str("\n\n\n");

    }

    let mut output = File::create(path)?;
    write!(output, "{}", &markdown)?;
    Ok(())
}

pub fn get_ast_definitions<'a>(ast: &Document<'static, String>) -> Result<Vec<DefinitionResult<'static>>, Box<dyn Error>> {
    let mut results: Vec<DefinitionResult> = Vec::new();
    for definition in &ast.definitions {
        match definition {
            TypeDefinition(Object(o)) => {
                let mut tmp_fields: Vec<DefinitionField> = Vec::new();
                for field in &o.fields {
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
                        name: o.name.clone(),
                        description: o.description.clone(),
                        fields: tmp_fields,
                        values: tmp_values,
                    }
                );
            },
            TypeDefinition(Enum(e)) => {
                let mut tmp_values: Vec<DefinitionValue> = Vec::new();
                for value in &e.values {
                    tmp_values.push(
                        DefinitionValue {
                            name: value.name.clone(),
                            description: value.description.clone(),
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
                        name: e.name.clone(),
                        description: e.description.clone(),
                        fields: tmp_fields,
                        values: tmp_values,
                    }
                );
            },
            // Work on interface next
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
                println!("Scalar Coming soon");
            },
            TypeDefinition(Union(u)) => {
                println!("Union Coming soon");
            },
            TypeDefinition(InputObject(io)) => {
                println!("InputObject Coming soon");
            },
            SchemaDefinition(_) | TypeExtension(_) | DirectiveDefinition(_) => todo!(),
        }
    }
    
    // Code to delete entry equal to _Schema_ from results
    results.retain(|r| *r.name != "_Schema_".to_string());

    Ok(results)
}