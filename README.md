# Version 1
## What 
Web app that let's a user generate Markdown-based Subgraph documentation from a GraphQL schema


![image](https://user-images.githubusercontent.com/43630382/222188985-c592caa9-797e-4caf-8cfa-43babdf7ffa4.png)

*Figure 1. Prototype UI*


## Why
The current process to write Subgraph documentation is a manual and tedious process, which explains why many development teams don't bother to document their Sugraphs. This is a non-ideal situation for dApp developers looking to leverage Subgraphs quickly, without spending too much time manually reading various GraphQL schemas. This tool automates the schema-to-documentation process, unblocking an important bottleneck in dApp development. 

## How
### Business logic
Rust
* If URL given, download `schema.graphql` from URL
* Else
    * Parse `schema.graphql` contents for entities
    * Generate documentation (Markdown files) from user input and parsed entities

### UI
React
* As a user I can input the URL or raw text for a `schema.graphql`
* As a user I can download the documentation (Markdown files) generated from my `schema.graphql` input

### Deployment
* Rate limit against overuse from an IP address
* Log status code, IP, Lat, Long, and User Agent of users submitting requests (for 30 days)

## Who
* Paka
* Dylan
* Don

## When
Delivery by end of August 2023
