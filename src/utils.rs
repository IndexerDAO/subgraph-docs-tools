use std::collections::HashMap;
use std::string::String;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SubgraphManifest {
    pub dataSources: Vec<DataSource>,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub specVersion: String,
    pub schema: SchemaAddress,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SchemaAddress {
    pub file: HashMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct DataSource {
    pub kind: String,
    pub mapping: Mapping,
    pub name: String,
    pub network: String,
    pub source: Source,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Mapping {
    pub abis: serde_yaml::Sequence,
    pub apiVersion: String,
    pub entities: serde_yaml::Sequence,
    pub eventHandlers: serde_yaml::Sequence,
    pub file: HashMap<String, String>,
    pub kind: String,
    pub language: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Source {
    pub abi: String,
    pub address: String,
    pub startBlock: u32,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DefintionResult<'a> {
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