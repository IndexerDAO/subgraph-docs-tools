use std::string::String;
use std::fs::File;
use std::io::Write;

use graphql_parser::query::Type;

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