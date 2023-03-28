mod utils;
use crate::utils::{
    get_ast_definitions, 
    DefinitionResult, 
    generate_entities_page
};

use graphql_parser::schema::parse_schema;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>  {
    // Read testing-schema.graphql
    let contents = std::fs::read_to_string("./testing-schema.graphql")
    .expect("Should have been able to read the file");

    // Parse schema
    let ast = parse_schema::<String>(&contents)?.into_static();

    let results: &Vec<DefinitionResult<'static>> = &get_ast_definitions(&ast)?;

    // Generate markdown from parsed entities
    generate_entities_page(&results)?;

    println!("Subgraph documentation is ready");

    Ok(())
}

