use std::error::Error;

mod utils;
use crate::utils::{SubgraphManifest, DefinitionResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let manifest_response = reqwest::get("https://ipfs.io/ipfs/QmbW34MGRyp7LWkpDyKXDLWsKrN8iqZrNAMjGTYHN2zHa1")
    .await?
    .text()
    .await?;

    let manifest_data: SubgraphManifest = serde_yaml::from_str(&manifest_response).unwrap();

    println!("{:?}", manifest_data);

    Ok(())
}
