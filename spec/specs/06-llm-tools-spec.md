# Spec 06: LLM Tools Integration

**Status:** `complete`
**Derived From:** [Backend Foundations PRD](../prds/backend-foundations-prd.md)

## Objective
Define the internal abstractions and tools required for the backend to communicate with external Large Language Models (LLMs).

## Requirements

### 1. Configuration Enforcement
- LLM API endpoints, keys, model names, and timeouts must be defined in the central `AppConfig`.
- The application must not fall back to hardcoded model names or keys if absent in the configuration.

### 2. Provider Abstraction
- The `llm_tools` module should define a generic interface (e.g., a trait) for prompting models, parsing responses, and handling errors (like rate limits or timeouts) uniformly, regardless of the underlying provider (OpenAI, Anthropic, local).

### 3. State Integration
- Instantiated LLM clients must be stored within the `AppState` so they can be injected into handlers seamlessly.

## Testing Strategy
- **Unit Tests:**
  - Mock the LLM provider interface.
  - Assert that tool execution maps generic prompt requests to the appropriate client implementation and correctly unwraps the simulated response.
