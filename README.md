Web app that let's a user generate Markdown-based Subgraph documentation from a GraphQL schema

![image](https://user-images.githubusercontent.com/43630382/222188985-c592caa9-797e-4caf-8cfa-43babdf7ffa4.png)

*Figure 1. Prototype UI*


## Overview
The current process to write Subgraph documentation is a manual and tedious process, which explains why many development teams don't bother to document their Sugraphs. This is a non-ideal situation for dApp developers looking to leverage Subgraphs quickly, without spending too much time manually reading various GraphQL schemas. 

Our tool automates the `schema.graphql`-to-documentation process, unblocking an important bottleneck in dApp development. The web app will be hosted on a subdomain of indexerdao.com, i.e. subgraphdocstools.indexerdao.com or something similar. We anticipate completing version 1 of this tool by the end of August 2023.


## Existing alternatives
* https://github.com/brettchalupa/graphql-docs
    * Generates HTML-based documentation from GraphQL Schema

## User stories
* As a subgraph docs team member, I want to input the raw text of a `schema.graphql` file and click a button to automatically generate schema entity documentation that I can copy elsewhere

## Future plans (version 2 and beyond)
Leverage ChatGPT to generate:
* schema and entity descriptions where they are missing
* sample queries of interest for dApp developers

## Project team
* Paka, project manager and back-end engineer 
* Dylan, front-end engineer
* Don, quality assurance engineer
