# Graph RAG Product Requirements Document (PRD)

## Overview
This PRD outlines the architecture and requirements for the **Graph Retrieval-Augmented Generation (Graph RAG)** capability within the `agent-as-data` platform. The goal is to move beyond standard semantic search by ingesting markdown documents, extracting structured knowledge, and building a traversable knowledge graph. This enables the system to answer complex, multi-hop queries that require synthesizing information across multiple interconnected concepts and documents.

## High-Level Pipeline Architecture

The Graph RAG pipeline consists of three primary phases: Ingestion & Extraction, Storage & Graph Construction, and Querying.

```mermaid
flowchart TD
    subgraph "Phase 1: Ingestion & Extraction"
        MD[Markdown Documents]
        Chunker[Document Splitter]
        LLMExtract[LLM Extraction\n(rig-core)]
    end

    subgraph "Phase 2: Storage & Graph Construction"
        RelDB[(PostgreSQL\nRelational Data)]
        AGE[(Apache AGE\nGraph Engine)]
        VecDB[(pgvector\nEmbeddings)]
    end

    subgraph "Phase 3: Hierarchical Summarization (Layers)"
        GraphClustering[Graph Clustering\n(Communities)]
        LLMSummarize[LLM Summarization\n(Community Nodes)]
    end

    subgraph "Phase 4: Querying"
        UserQuery[User Request]
        QueryAnalyzer[Query Router / Analyzer]
        Cypher[Cypher Graph Traversal]
        VectorSearch[Semantic Vector Search]
        Synthesizer[LLM Synthesizer]
    end

    MD --> Chunker
    Chunker -->|Text Chunks| LLMExtract
    LLMExtract -->|Structured Output:\nNodes (Knowledge Objects)\nEdges (Graph Tuples)| RelDB
    RelDB -.-> AGE
    RelDB -.-> VecDB

    AGE --> GraphClustering
    GraphClustering --> LLMSummarize
    LLMSummarize -->|Summary Nodes & Hierarchy Edges| AGE
    LLMSummarize -->|Summary Embeddings| VecDB

    UserQuery --> QueryAnalyzer
    QueryAnalyzer --> VectorSearch
    QueryAnalyzer --> Cypher
    VectorSearch --> Synthesizer
    Cypher --> Synthesizer
    Synthesizer -->|Final Answer| UserQuery
```

## Core Components & Requirements

### 1. Ingestion & LLM Extraction (Knowledge Objects & Tuples)
**Objective:** Parse raw markdown text into structured graph entities.
- **Input:** Raw markdown files containing thematic or narrative data.
- **Chunking:** The system must split markdown files into manageable semantic chunks (e.g., by headers, paragraphs, or fixed token length) before sending to the LLM.
- **LLM Prompting (rig-core):**
  - We use the `rig-core` crate to prompt an LLM to extract structured data from each chunk.
  - **Knowledge Objects (Nodes):** The LLM must identify key entities, concepts, or descriptive units of knowledge from the text. Each object must have a unique identifier, a label/type, and a coherent textual description.
  - **Graph Tuples (Edges):** The LLM must extract relationships between these objects in the form of tuples (Source Node, Relationship Type/Predicate, Target Node).
- **Idempotency:** Re-ingesting the same document should update or merge nodes/edges rather than creating duplicates.

### 2. Graph Storage (Apache AGE + pgvector)
**Objective:** Persist the extracted entities in a way that supports both semantic similarity search and structural graph traversal.
- **Storage Backend:** We utilize PostgreSQL as the unified storage layer.
- **Vector Storage:** The textual descriptions of Knowledge Objects (and later, community summaries) are embedded and stored in standard Postgres tables using the `pgvector` extension for semantic search.
- **Graph Engine:** We leverage the **Apache AGE** extension for Postgres to store the Nodes and Edges. This allows us to use OpenCypher queries to traverse the graph (e.g., finding all concepts connected to "Entity X" within 2 hops) directly within our relational database.
- **Data Synchronization:** The system must maintain consistency between the standard relational/vector tables and the AGE graph structures (if stored separately) or utilize a unified schema supported by AGE.

### 3. Hierarchical Knowledge (Layers & Communities)
**Objective:** Enable the system to answer broad, global queries ("What are the main themes?") by summarizing dense local graphs.
- **Graph Clustering:** The system must implement (or utilize an external library for) community detection algorithms (e.g., Leiden or Louvain) to group heavily interconnected Knowledge Objects into clusters (Communities).
- **Hierarchical Summarization:**
  - **Layer 0:** The base layer of extracted Knowledge Objects and Tuples.
  - **Layer 1..N:** For each identified community, an LLM is prompted to write a comprehensive summary of that community based on its constituent nodes and edges.
  - These summaries become new "Summary Nodes" in the graph, hierarchically linked to the lower-level nodes they summarize.
- **Embeddings:** Summary Nodes are also embedded via `pgvector`, allowing high-level semantic search across broad topics.
- *Note: This hierarchical component is documented here as a core capability but will be broken out into a separate technical specification during the implementation phase.*

### 4. Baseline Evaluation Data & Benchmarking
**Objective:** Ensure the Graph RAG pipeline can be empirically tested for accuracy and regression.
- **Dataset:** We will curate a subset of interconnected markdown documents derived from an established multi-hop reasoning dataset (e.g., HotpotQA style) or a complex narrative wiki (e.g., historical events).
- **Test Harness:** The sample data must be stored in the repository (e.g., `tests/data/graph_rag_baseline/`) and used in automated integration tests to:
  1. Validate that the LLM extraction yields a consistent graph structure.
  2. Prove that Cypher queries on Apache AGE return the expected multi-hop results.
  3. Ensure that changes to prompts or chunking logic do not regress the quality of the extracted knowledge graph.

## Future Considerations
- Supporting dynamic updates to the graph when source markdown files are edited or deleted.
- Implementing a specialized Query Router LLM that decides whether an incoming user query requires a local search (Layer 0 nodes), a global search (Community Summaries), or a hybrid approach.
