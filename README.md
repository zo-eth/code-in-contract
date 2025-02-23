# code-in-contract
Solana's lowest gas on-chain project that allows everything to be written on-chain.


**Introduction**
Code-in is Solana On-chain storage system

Store values ​​using block state changes. 
"We don't allocate space to a single variable."

**Key Innovations of Code-In**

1. Storage Optimization:

Code-In reconstructs data dynamically using update records instead of storing it explicitly, significantly reducing storage costs.


2. State Update Method:

The system avoids logs (which are not 100% on-chain) and operates purely through state updates to ensure complete on-chain reliability.

3. DB PDA for Fast Data Retrieval (Updated):

The DB PDA is solely used to get data and provides quick access to:

lastTxID: The most recent transaction ID.

datatype: The type of data (e.g., MP3, image).

offset: A flexible field for storing metadata like compression methods.


By centralizing data access in the DB PDA, Code-In avoids unnecessary traversals of the state-linked list.


4. Split Compression for Search Optimization (New):

Code-In implements split compression to optimize search operations for large text data.

Mechanism:

If a text size exceeds 10,000 bytes, the data is broken into smaller encoded segments.

This approach, referred to as decode break, ensures efficient encoding and decoding of large data chunks without compromising speed.


Benefit: Improves search and retrieval performance for large datasets.


5. Linked List for Sequential Access:

The linked list remains useful for sequential data access when needed, particularly during state updates.


Updated System Workflow

1. Data Retrieval (DB PDA):

The DB PDA is now the primary access point for retrieving data and metadata efficiently.

2. Split Compression for Large Text:

Text data exceeding 10,000 bytes is split into manageable chunks for encoding and decoding.

During retrieval, the system uses decode break to reconstruct the original data seamlessly.

3. State Updates and Linked List:

State updates ensure all changes remain on-chain.

The linked list structure is reserved for sequential access scenarios.


Strengths (Finalized)

1. Fast Data Retrieval:

Centralizing retrieval through the DB PDA minimizes complexity and ensures quick access.

2. Search Optimization for Large Text:

Split compression and decode break make handling large text data efficient and scalable.

3. Cost Efficiency:

Avoiding explicit storage and using state changes with split compression achieves up to 2000x cost reduction.

4. Scalability:

Code-In handles large-scale text data effectively without compromising on retrieval speed.

5. Full On-Chain Reliability:

The state update method ensures all data operations remain fully on-chain.


Conclusion

By centralizing data access through the DB PDA and implementing split compression for large datasets, Code-In achieves efficient, cost-effective, and scalable on-chain storage on Solana. This optimized architecture balances fast data retrieval, search efficiency, and reliability, addressing the challenges of traditional blockchain storage systems.



