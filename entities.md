# Subgraph Entities
* [Project](#project)
* [Category](#category)
* [Challenge](#challenge)
* [Everest](#everest)
* [Vote](#vote)
* [Choice](#choice)
* [User](#user)

## Project
Projects are the member type which the Everest list is curated for


| Field/Value | Type | Description | 
| --- | --- | --- | 
| id | ID! | Required ID | 
| ipfsHash | String | The IPFS hash where all off-chain data is stored | 
| name | String | Project name | 
| description | String | Project description | 
| website | String | Project website | 
| twitter | String | Project twitter handle | 
| github | String | Project github URL | 
| avatar | String | Project avatar | 
| image | String | Project image | 
| categories | [Category!]! | List of project categories | 
| isRepresentative | Boolean! | True if a representative of the company owns this project | 
| createdAt | Int! | Time it was created at on the blockchain | 
| updatedAt | Int! | Time it was updated at on the blockchain | 
| owner | User | Owner of this project | 
| currentChallenge | Challenge | Current challenge against this project | 
| pastChallenges | [Challenge!] | Past challenges agaisnt this project | 
| createdChallenges | [Challenge!] | Challenges this project has created against other projects | 
| membershipStartTime | Int! | Time this project joined Everest | 
| delegates | [User!] | List of all delegates of this project | 
| totalVotes | Int! | Total vote count | 
| votes | [Vote!] | All votes a project has made | 



## Category
Project Categories


| Field/Value | Type | Description | 
| --- | --- | --- | 
| id | ID! | The ID is a lowercased name | 
| description | String! | The category description | 
| imageHash | String! | The IPFS hash of the category image | 
| imageUrl | String! | The Url of the category image | 
| name | String! | The name of the category, case insensitive | 
| slug | String! | The name used for the front end | 
| projects | [Project!]! | Projects with this category designation | 
| subcategories | [Category!] | The subcategories of this Category | 
| parentCategory | Category | Parent category of this category. Null if it is a top level category | 
| createdAt | Int! | Time it was created in the Subgraph | 
| projectCount | Int! | Number of projects in this category and all of its subcategories | 



## Challenge

| Field/Value | Type | Description | 
| --- | --- | --- | 
| id | ID! | Challenge ID | 
| ipfsHash | String! | IPFS hash where the description is stored | 
| description | String | Challenge description | 
| endTime | Int! | End time of the challenge | 
| removeVotes | Int! | Votes yes to a challenge for removal of the project (in weight) | 
| keepVotes | Int! | Voting no to a challenge for keeping the project  (in weight) | 
| project | Project | Project that is being challenged | 
| owner | Project | Owner of the challenge, which is a project | 
| votes | [Vote!] | List of all created votes | 
| resolved | Boolean! | True if the challenge has been resolved | 
| createdAt | Int! | Time challenge was created on the blockchain | 



## Everest
Everest holds the global variables relevant to the dapp


| Field/Value | Type | Description | 
| --- | --- | --- | 
| id | ID! | | 
| owner | Bytes! | Owner of the Everest contract | 
| approvedToken | Bytes! | Approved token for Everest fees | 
| votingPeriodDuration | Int! | Voting period for challenges | 
| challengeDeposit | BigInt! | Challege deposit in DAI | 
| applicationFee | BigInt! | Fee to apply to be in Everest | 
| everestAddress | Bytes! | Address of everest | 
| reserveBankAddress | Bytes! | Address of the reserve bank | 
| reserveBankBalance | BigInt! | Balance of the reserve bank (DAI) | 
| categories | Bytes! | IPFS hash pointing to the categories | 
| charter | Bytes! | IPFS hash pointing to the charter | 
| createdAt | Int! | Time it was created on the blockchain | 
| projectCount | Int! | Total count of projects created on Everest | 
| claimedProjects | Int! | Projects that are currently in control by a representative | 
| challengedProjects | Int! | Projects that are currently under challenge | 
| categoriesCount | Int! | The amount of categories in Everest | 



## Vote
A challenge vote


| Field/Value | Type | Description | 
| --- | --- | --- | 
| id | ID! | Concatenation of challenge ID and voter address | 
| voter | Project | Project that voted on the challenge | 
| challenge | Challenge! | Challenge the vote is for | 
| choice | Choice! | Vote choice | 
| weight | Int! | Vote weight based on project reputation | 
| createdAt | Int! | Time that vote was created on the blockchain | 



## Choice
The Vote choice enum


| Field/Value | Type | Description | 
| --- | --- | --- | 
| Null | | | 
| Yes | | | 
| No | | | 



## User
A User of the Everest Dapp. A User is the owner or delegate of Projects. User info
is obtained from 3box


| Field/Value | Type | Description | 
| --- | --- | --- | 
| id | ID! | User ethereum address | 
| projects | [Project!] | Projects the user owns | 
| delegatorProjects | [Project!] | Projects the user is a delegate of | 
| createdAt | Int! | The time the user was created in the Subgraph (not the blockchain) | 



