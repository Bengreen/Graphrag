# Spec 10: Graph RAG - Baseline Evaluation & Benchmarking

**Status:** `draft`
**Derived From:** [Graph RAG PRD](../prds/graph-rag-prd.md)

## Objective
Ensure the Graph RAG pipeline can be empirically tested for accuracy, performance, and regression through automated test harnesses and baseline data.

## Requirements

### 1. Curated Dataset
- The system must include a curated subset of interconnected markdown documents stored within the repository (e.g., `tests/data/graph_rag_baseline/`).
- This dataset should be derived from complex narrative wikis or established multi-hop reasoning datasets (e.g., HotpotQA style) to adequately test graph extraction and traversal.

### 2. Automated Test Harness
- Integration tests must be implemented using this baseline dataset to validate the entire Graph RAG pipeline.
- **Validation Criteria:**
  - **Extraction Quality:** Prove that the LLM extraction yields a consistent and expected graph structure from the baseline documents.
  - **Query Accuracy:** Prove that Cypher queries executed on Apache AGE return the expected multi-hop results corresponding to the baseline data.
  - **Regression Prevention:** Ensure that modifications to chunking logic, prompts, or the LLM model do not regress the quality of the extracted knowledge graph or query results.

## Testing Strategy
- **End-to-End Evaluation Tests:**
  - Ingest the baseline dataset.
  - Perform automated assertions against the resulting graph structure (node counts, edge types).
  - Execute a suite of multi-hop Cypher queries and semantic searches, comparing the results against expected ground-truth answers.
