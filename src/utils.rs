use std::string::String;

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