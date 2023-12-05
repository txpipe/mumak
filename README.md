# Mumak

A custom PostgreSQL extension to interact with Cardano CBOR data directly.

What if postgresql knew how to “talk” cardano?

## Introduction

The natural wire format used to exchange Cardano data is CBOR. By Cardano data we refer to Blocks, Transactions, UTxOs, Certificates and all of its inner structures.

When indexing this data onto a relational database, such as PostgresQL, we usually deserialize the CBOR structures and map the values to different columns & tables in our schema.

Some times having a normalized data model is exactly what your use-case needs, but there are multiple scenarios where this mapping just adds complexity and performance penalties.

Mumak is a PostgreSQL extension that provides several custom functions to interact with Cardano CBOR data directly on the database.

For example, this means that you could store a whole Cardano Tx as CBOR in a bytes PostgreSQL column and then use SQL to ask things such as:

- Does this transaction involve address X?
- What's the total ADA output of this transaction?
- Is this transaction minting a token for policy X?

And the evaluation of this values will happen in-process, as part of the Db query execution.

## Available Functions

This is a list of the available / planned functions that are / will be supported by Mumak.

// TODO

## Getting Started

// TODO

## Ecosystem

### Oura Integration [PLANNED]

An Oura sink will be developed to output common Cardano structures (Blocks, Transactions, UTxOs, Certificates) to a well-known schema that leverages the Mumak extensions for further querying.

Using Oura's integration with Mithril, this provides a way to bootstrap a queryable Cardano dataset in very little time.

### Scrolls Integration [PLANNED]

Scrolls will provide helper utility functions to create reducers that map data into Mumak-compatible records that can be stored in PostgreSQL directly. This approach simplifies the complexity of building complex reducer algorithms by offloading some of the complexity to the PostgreSQL database.

### Demeter Integration [PLANNED]

Demeter will provide a highly available, horiztonally scalable access to Mumak data as part of their services. Developers will be able to access a PostgreSQL instance directly and perform SQL queries on common Cardano CBOR structures using Mumak statements.

Data will be updated continously showing the most recent data from each of the common network in the Cardano blockchain ecosystem.
