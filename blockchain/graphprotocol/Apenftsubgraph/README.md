Cheers to Nader Dabit: https://dev.to/dabit3/building-graphql-apis-on-ethereum-4poa

The number of production dapps continues to explode and the demand for developers building with Solidity and other blockchain languages continues to outpace supply, moving wages higher and higher.

As a developer getting into this space, I quickly learned there are a lot of differences in the way that we interact with and build on top of blockchains versus what I was used to in the traditional web. With Ethereum (and other blockchains), data isn’t stored in a format that can efficiently or easily be consumed directly from other applications or front ends. The problem is that you need to have the data indexed and organized for efficient retrieval.

Traditionally, that’s the work that databases do in the centralized tech stack, but that indexing layer was missing in the Web3 stack.

In the traditional web stack, databases, servers, and APIs query, filter, sort, paginate, group, and join data before it is returned to an application usually via some type of http request. These types of data transformations are not possible when reading data directly from Ethereum or other blockchains.

In the past, developers were getting around this by building their own centralized indexing servers - pulling data from blockchains, storing it in a database, and exposing it over an API. This required significant engineering and hardware resources and broke the important security properties required for decentralization.

How can we build an API on top of blockchain data that can easily be deployed to a decentralized web infrastructure? Let's find out.

Decentralizing web infrastructure
The vision of and movement towards a decentralized internet is commonly referred to as Web3. Web3 enhances the internet as we know it today with these added characteristics:

Decentralized
Verifiable
Trustless
Self governing
For a great overview of what Web3 is, check out this video by Juan Benet

To achieve decentralization, protocols define networks that offer a range of digital services such as compute, storage, bandwidth, identity, and other pieces of web infrastructure with no intermediaries. These protocols are usually distributed across multiple nodes (servers) that enable the participation of anyone looking to be part of the network and provide the service.

The network participants are incentivized to provide the highest quality services to anyone consuming them. Rules are also put in place to ensure the security and the integrity of the network itself. This is usually accomplished using a combination of consensus mechanisms programmed into smart contracts implementing various types of game theory and cryptoeconomic design.

What makes a service truly decentralized?
Yaniv Tal profile imageYaniv Tal@yanivgraphtwitter logo
@BernFeitosa 1) It’s operated by a permissionless network of nodes
2) The service is verifiable
3) Trust minimized
4) Market-based pricing
5) Censorship resistance
00:06 AM - 30 Apr 2021
Twitter reply action Twitter retweet action Twitter like action
Building on The Graph
In this post, we will be looking at one such protocol, The Graph, and how to build and deploy our own GraphQL API using data stored in the Ethereum blockchain.

The Graph is an indexing protocol for querying blockchains like Ethereum and networks like IPFS. Anyone can build and publish open APIs, called subgraphs, making data easily accessible.

Subgraphs define the data, data sources, and data access patterns that you would like to have made available via a GraphQL API. You as a developer can choose either to use a subgraph already deployed by another developer, or define and deploy your own subgraph and use it.

Developers can create open APIs by deploying their subgraphs either to the hosted service or to the network, enabling them to earn money based on the amount of usage of their API.

Subgraphs are made up of a few main parts:

1. GraphQL Schema
The GraphQL Schema defines the data types / entities you would like to save and query for. You can also define configuration like relationships and full text search capabilities in your schema.

2. Subgraph Manifest (yaml configuration)
(from the docs) The manifest defines the smart contracts your subgraph indexes, their ABIs, which events from these contracts to pay attention to, and how to map event data to entities that Graph Node stores and allows to query.

3. AssemblyScript Mappings
AssemblyScript Mappings allow you to save data to be indexed using the entity types defined in your schema. The Graph CLI also generates AssemblyScript types using a combination of the subgraph's schema along with a smart contract's ABIs.

Let's start building
Now that we have a good understanding of The Graph and how it works, let's start writing some code.

In this tutorial we'll build a subgraph for querying NTF data from the Zora smart contract, implementing queries for fetching NFTs as well as their owners, and building relationships between them.

Prerequisites
To be successful in this tutorial, you should have Node.js installed on your machine. These days, I recommend using either nvm or fnm to manage Node.js versions.

Creating the Graph project in the Graph Explorer
To get started, open the Graph Explorer and either sign in or create a new account.

Next, go to the dashboard and click on Add Subgraph to create a new subgraph.

Configure your subgraph with the following properties:

Subgraph Name - Zoranftsubgraph
Subtitle - A subgraph for querying NFTs
Optional - Fill the description and GITHUB URL properties
Once the subgraph is created, we will initialize the subgraph locally using the Graph CLI.

Initializing a new subgraph using the Graph CLI
Next, install the Graph CLI:
$ npm install -g @graphprotocol/graph-cli

# or

$ yarn global add @graphprotocol/graph-cli
Once the Graph CLI has been installed you can initialize a new subgraph with the Graph CLI init command.

There are two ways to initialize a new subgraph:

1 - From an example subgraph
$ graph init --from-example <GITHUB_USERNAME>/<SUBGRAPH_NAME> [<DIRECTORY>]
2 - From an existing smart contract

If you already have a smart contract deployed to Ethereum mainnet or one of the testnets, initializing a new subgraph from this contract is an easy way to get up and running.
$ graph init --from-contract <CONTRACT_ADDRESS> \
  [--network <ETHEREUM_NETWORK>] \
  [--abi <FILE>] \
  <GITHUB_USER>/<SUBGRAPH_NAME> [<DIRECTORY>]
In our case we'll be using the Zora Token Contract so we can initilize from that contract address by passing in the contract address using the --from-contract flag:
$ graph init --from-contract 0xabEFBc9fD2F806065b4f3C237d4b59D9A97Bcac7 --network mainnet  \
--contract-name Token --index-events

? Subgraph name › your-username/Zoranftsubgraph
? Directory to create the subgraph in › Zoranftsubgraph
? Ethereum network › Mainnet
? Contract address › 0xabEFBc9fD2F806065b4f3C237d4b59D9A97Bcac7
? Contract Name · Token
This command will generate a basic subgraph based off of the contract address passed in as the argument to --from-contract. By using this contract address, the CLI will initialize a few things in your project to get you started (including fetching the abis and saving them in the abis directory).

By passing in --index-events the CLI will automatically populate some code for us both in schema.graphql as well as src/mapping.ts based on the events emitted from the contract.

The main configuration and definition for the subgraph lives in the subgraph.yaml file. The subgraph codebase consists of a few files:

subgraph.yaml: a YAML file containing the subgraph manifest
schema.graphql: a GraphQL schema that defines what data is stored for your subgraph, and how to query it via GraphQL
AssemblyScript Mappings: AssemblyScript code that translates from the event data in Ethereum to the entities defined in your schema (e.g. mapping.ts in this tutorial)
The entries in subgraph.yaml that we will be working with are:

description (optional): a human-readable description of what the subgraph is. This description is displayed by the Graph Explorer when the subgraph is deployed to the Hosted Service.
repository (optional): the URL of the repository where the subgraph manifest can be found. This is also displayed by the Graph Explorer.
dataSources.source: the address of the smart contract the subgraph sources, and the abi of the smart contract to use. The address is optional; omitting it allows to index matching events from all contracts.
dataSources.source.startBlock (optional): the number of the block that the data source starts indexing from. In most cases we suggest using the block in which the contract was created.
dataSources.mapping.entities : the entities that the data source writes to the store. The schema for each entity is defined in the the schema.graphql file.
dataSources.mapping.abis: one or more named ABI files for the source contract as well as any other smart contracts that you interact with from within the mappings.
dataSources.mapping.eventHandlers: lists the smart contract events this subgraph reacts to and the handlers in the mapping — ./src/mapping.ts in the example — that transform these events into entities in the store.
Defining the entities
With The Graph, you define entity types in schema.graphql, and Graph Node will generate top level fields for querying single instances and collections of that entity type. Each type that should be an entity is required to be annotated with an @entity directive.

The entities / data we will be indexing are the Token and User. This way we can index the Tokens created by the users as well as the users themselves.

To do this, update schema.graphql with the following code:
type Token @entity {
  id: ID!
  tokenID: BigInt!
  contentURI: String!
  metadataURI: String!
  creator: User!
  owner: User!
}

type User @entity {
  id: ID!
  tokens: [Token!]! @derivedFrom(field: "owner")
  created: [Token!]! @derivedFrom(field: "creator")
}
On Relationships via @derivedFrom (from the docs):
Reverse lookups can be defined on an entity through the @derivedFrom field. This creates a virtual field on the entity that may be queried but cannot be set manually through the mappings API. Rather, it is derived from the relationship defined on the other entity. For such relationships, it rarely makes sense to store both sides of the relationship, and both indexing and query performance will be better when only one side is stored and the other is derived.

For one-to-many relationships, the relationship should always be stored on the 'one' side, and the 'many' side should always be derived. Storing the relationship this way, rather than storing an array of entities on the 'many' side, will result in dramatically better performance for both indexing and querying the subgraph. In general, storing arrays of entities should be avoided as much as is practical.

Now that we have created the GraphQL schema for our app, we can generate the entities locally to start using in the mappings created by the CLI:
graph codegen
In order to make working smart contracts, events and entities easy and type-safe, the Graph CLI generates AssemblyScript types from a combination of the subgraph's GraphQL schema and the contract ABIs included in the data sources.

Updating the subgraph with the entities and mappings
Now we can configure the subgraph.yaml to use the entities that we have just created and configure their mappings.

To do so, first update the dataSources.mapping.entities field with the User and Token entities:
entities:
  - Token
  - User
Next, update the dataSources.mapping.eventHandlers to include only the following two event handlers:
eventHandlers:
  - event: TokenURIUpdated(indexed uint256,address,string)
    handler: handleTokenURIUpdated
  - event: Transfer(indexed address,indexed address,indexed uint256)
    handler: handleTransfer
Finally, update the configuration to add the startBlock:
source:
  address: "0xabEFBc9fD2F806065b4f3C237d4b59D9A97Bcac7"
  abi: Token
  startBlock: 11565020
Assemblyscript mappings
Next, open src/mappings.ts to write the mappings that we defined in our subgraph subgraph eventHandlers.

Update the file with the following code:
import {
  TokenURIUpdated as TokenURIUpdatedEvent,
  Transfer as TransferEvent,
  Token as TokenContract
} from "../generated/Token/Token"

import {
  Token, User
} from '../generated/schema'

export function handleTokenURIUpdated(event: TokenURIUpdatedEvent): void {
  let token = Token.load(event.params._tokenId.toString());
  token.contentURI = event.params._uri;
  token.save();
}

export function handleTransfer(event: TransferEvent): void {
  let token = Token.load(event.params.tokenId.toString());
  if (!token) {
    token = new Token(event.params.tokenId.toString());
    token.creator = event.params.to.toHexString();
    token.tokenID = event.params.tokenId;

    let tokenContract = TokenContract.bind(event.address);
    token.contentURI = tokenContract.tokenURI(event.params.tokenId);
    token.metadataURI = tokenContract.tokenMetadataURI(event.params.tokenId);
  }
  token.owner = event.params.to.toHexString();
  token.save();

  let user = User.load(event.params.to.toHexString());
  if (!user) {
    user = new User(event.params.to.toHexString());
    user.save();
  }
}
These mappings will handle events for when a new token is created, transfered, or updated. When these events fire, the mappings will save the data into the subgraph.

Running a build
Next, let's run a build to make sure that everything is configured properly. To do so, run the build command:
$ graph build
If the build is successful, you should see a new build folder generated in your root directory.

Deploying the subgraph
To deploy, we can run the deploy command using the Graph CLI. To deploy, you will first need to copy the Access token for the subgraph you created in the Graph Explorer:

Graph Explorer

Next, run the following command:
$ graph auth https://api.thegraph.com/deploy/ <ACCESS_TOKEN>

$ yarn deploy
Once the subgraph is deployed, you should see it show up in your dashboard:

Graph Dashboard

When you click on the subgraph, it should open the Graph explorer:

The Zora Subgraph

Querying for data
Now that we are in the dashboard, we should be able to start querying for data. Run the following query to get a list of tokens and their metadata:
{
  tokens {
    id
    tokenID
    contentURI
    metadataURI
  }
}
We can also configure the order direction:
{
  tokens(
    orderBy:id,
    orderDirection: desc
  ) {
    id
    tokenID
    contentURI
    metadataURI
  }
}
Or choose to skip forward a certain number of results to implement some basic pagination:
{
  tokens(
    skip: 100,
    orderBy:id,
    orderDirection: desc
  ) {
    id
    tokenID
    contentURI
    metadataURI
  }
}
Or query for users and their associated content:
{
  users {
    id
    tokens {
      id
      contentURI
    }
  }
}
Updating the subgraph
What if we want to make some changes to the subgraph and then redeploy? This is pretty easy, so let's learn how to do it.

Let's say that we want to add a new feature to our subgraph. In addition to our existing querying capabilities, let's say that we wanted to add the capabilities to sort by the timestamp that the NFT was created.

To do so, we need to first add a new createdAtTimestamp field to the Token entity:
type Token @entity {
  id: ID!
  tokenID: BigInt!
  contentURI: String!
  metadataURI: String!
  creator: User!
  owner: User!
  "Add new createdAtTimesamp field"
  createdAtTimestamp: BigInt!
}
Now we can re-run the codegen:
graph codegen
Next, we need to update the mapping to save this new field:
// update the handleTransfer function to add the createdAtTimestamp to the token object
export function handleTransfer(event: TransferEvent): void {
  let token = Token.load(event.params.tokenId.toString());
  if (!token) {
    token = new Token(event.params.tokenId.toString());
    token.creator = event.params.to.toHexString();
    token.tokenID = event.params.tokenId;
    // Add the createdAtTimestamp to the token object
    token.createdAtTimestamp = event.block.timestamp;

    let tokenContract = TokenContract.bind(event.address);
    token.contentURI = tokenContract.tokenURI(event.params.tokenId);
    token.metadataURI = tokenContract.tokenMetadataURI(event.params.tokenId);
  }
  token.owner = event.params.to.toHexString();
  token.save();

  let user = User.load(event.params.to.toHexString());
  if (!user) {
    user = new User(event.params.to.toHexString());
    user.save();
  }
}
Now we can re-deploy the subgraph:
$ yarn deploy
Once the subgraph has been redeployed, we can now query by timestamp to view the most recently created NFTS:
{
  tokens(
    orderBy:createdAtTimestamp,
    orderDirection: desc
  ) {
    id
    tokenID
    contentURI
    metadataURI
  }
}
The codebase for this project is located here

