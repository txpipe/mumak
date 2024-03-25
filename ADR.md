# Mumak

Mumak is a specialized PostgreSQL extension developed to facilitate direct interaction with Cardano blockchain data encoded in CBOR format, aiming to provide efficient, in-database querying capabilities without the need for extensive data normalization.

# Motivation

The conventional approach to indexing Cardano blockchain data involves deserializing CBOR structures and mapping them to various tables and columns within a relational database schema. This method, while suitable for some applications, often introduces unnecessary complexity and performance bottlenecks, particularly in scenarios where direct access to the raw blockchain data is more efficient. Mumak emerges as a solution to this challenge, offering SQL-based interactions with CBOR data to streamline blockchain data analysis within PostgreSQL. Importantly, Mumak is developed with a strong focus on enhancing the developer experience, ensuring that the extension is not only performant but also intuitive and accessible for developers. This dual emphasis on efficiency and usability underpins the project's design and implementation strategies.

# Detailed Design

### Core Features

- **CBOR Data Storage**: Enable the storage of Cardano transactions and other blockchain entities in their native CBOR format within PostgreSQL, using bytea columns.

- **Custom SQL Functions**: Provide a set of SQL functions that allow users to query and manipulate CBOR-encoded blockchain data directly, covering common use cases like transaction analysis, address involvement checks, and token minting verifications.

- **Performance Optimization**: Utilize native Rust types and PostgreSQL's advanced features to ensure high performance and efficient data handling.

### Output Formats of Projection Functions

Mumak's projection functions are designed to offer flexible output formats to accommodate various use cases and developer preferences, providing three primary types of results:

- **PostgreSQL Query Result**: Standard output format, ideal for direct use within SQL queries and compatible with SQL-based data analysis and reporting tools.

- **JSON (CIP-116 Compliant)**: In alignment with the Cardano Improvement Proposal [CIP-116](https://github.com/klntsky/CIPs/blob/klntsky/json-spec-cip/CIP-XXXX/README.md), which standardizes JSON representations of blockchain data, Mumak's projection functions can return results in a structured JSON format compliant with these guidelines. This standardization ensures that the JSON output is not only web-friendly but also consistent with the broader Cardano ecosystem, facilitating seamless integration and interoperability with other Cardano-based applications and services.

- **Pure CBOR**: Essential for scenarios requiring the original data format. This option not only ensures fidelity to the original Cardano blockchain data structure but also enables the chaining or piping of function results. By returning pure CBOR, subsequent functions can directly process the output of a preceding function without needing intermediate transformations. This capability is particularly valuable in complex data processing workflows, where multiple operations need to be performed sequentially on the data.

By providing these varied output formats, Mumak enhances the flexibility and power of SQL-based blockchain data analysis, enabling developers to select the most suitable format for their specific application needs and workflows.



### Implementation Strategy

**Data Representation**

Leverage PostgreSQL's bytea data type for storing CBOR data, ensuring fidelity to the original blockchain data structure.

**Custom SQL Functions**: Projections and Filters

The development of custom SQL functions is central to Mumak's functionality, enabling on-the-fly decoding of CBOR data and extraction of relevant information in response to user queries. These functions are categorized into two main types: **projections** and **filters**, each serving a distinct purpose within the extension:

- **Projections**: These functions are designed to extract specific pieces of information from the CBOR-encoded blockchain data. Projections allow users to retrieve particular data points, such as transaction inputs, outputs, the total ADA output of a transaction, or the metadata associated with a transaction. By directly accessing and decoding the CBOR data, projections provide a powerful means of analyzing and extracting valuable insights from the blockchain data stored within PostgreSQL.

- **Filters**: Filters are specialized functions that enable users to apply conditions to the CBOR data, effectively narrowing down the dataset based on specific criteria. For example, filters can identify transactions involving a particular address, transactions minting tokens for a given policy, or transactions with a certain validity interval. Filters enhance the querying capabilities by allowing users to focus on subsets of the blockchain data that meet their specific requirements, thereby improving query efficiency and relevance.

The distinction between projections and filters is fundamental to Mumak's design, offering users a versatile toolkit for interacting with Cardano's blockchain data. Projections provide the mechanism for data extraction and analysis, while filters offer the means to refine and focus the data set based on defined criteria.

**Parameter Handling**

For parameter handling, Mumak utilizes native types compatible between Rust and Postgres for both simple and complex data types, ensuring efficient data representation and manipulation.

**Simple Types**: Data as such, addresses, policy IDs, and asset names are represented as byte arrays in Rust `[u8]` and as bytea in PostgreSQL (e.g., `\x1234ffff`). This direct byte-level representation allows for straightforward and efficient processing of these elements.

**Complex Types**: For more complex structures like redeemers and certificates, Mumak encodes the data in CBOR, which is then represented as byte arrays in Rust and bytea in PostgreSQL. This ensures that the structural complexity of these elements is preserved while still enabling efficient storage and query capabilities within the database.

This approach streamlines interaction with Cardano blockchain data by utilizing consistent, byte-oriented data handling for both simple and complex types.

### Drawbacks

- **Complexity in Handling CBOR**: The need to dynamically decode CBOR within SQL functions could introduce complexity in function implementation and maintenance.

- **Specialized Use Case**: The extension caters to a specific set of use cases centered around direct CBOR data interaction, which might not be applicable for all blockchain data analysis scenarios.


### Alternatives

- **Normalized Data Approach**: Continue with the traditional method of deserializing CBOR data and mapping it to relational database schemas, accepting the associated complexity and performance trade-offs.

- **Hybrid Models:** Develop a solution that combines normalized data storage for frequently accessed data points with direct CBOR storage for less common, complex queries.

### Unresolved Questions


- **Block projections**

- **Error Returns Best Practice**

- **Performance Benchmarks**: While the extension is designed with performance in mind, the actual gains from direct CBOR interaction versus traditional data normalization approaches need to be quantified through comprehensive benchmarking.