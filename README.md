# Version 1
## What 
Web app that let's a user generate Markdown-based Subgraph documentation from a GraphQL schema

## Why
The current process to write Subgraph documentation is a manual and tedious process, which explains why many development teams don't bother to document their Sugraphs. This is a non-ideal situation for dApp developers looking to leverage Subgraphs quickly, without spending too much time manually reading various GraphQL schemas. This tool automates the schema-to-documentation process, unblocking an important bottleneck in dApp development. 

## How
### Business logic
Rust
* Parse `schema.graphql` file for entities
* Generate Markdown-based documentation from user input and parsed entities

### UI
React
* As a user I can input the URL or raw text for a `schema.graphql`
* As a user I can download the Markdown-based documentation generated from the `schema.graphql` input

### Deployment
* As an app I can rate limit myself against overuse from an IP address
* As an app I can monitor the status code, IP, Lat, Long, and User Agent of users submitting requests to me

## Who
* Paka
* Dylan?

## When
Delivery by end of August 2023
