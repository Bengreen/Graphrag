# Spec 02: Configuration Loader

**Status:** `draft`
**Derived From:** [Backend Foundations PRD](../prds/backend-foundations-prd.md)

## Objective
Implement a fail-fast, centralized configuration loading mechanism that hydrates strongly typed structs at startup, explicitly disallowing runtime access to environment variables or hardcoded fallbacks.

## Requirements

### 1. Centralized Struct (`AppConfig`)
- The application configuration must be modeled using a single, centralized struct, typically named `AppConfig`.
- Configuration should be loaded using a robust crate (e.g., `config` or `figment`).
- Sources must include YAML configuration files and environment variable overrides (e.g., prefixed with `AAD_BE__`).

### 2. Fail-Fast Validation
- `AppConfig` must implement a `.validate()` method (or equivalent initialization check).
- The application **must immediately panic/exit** with a non-zero status code if configuration is missing, malformed, or invalid during startup.

### 3. Strict Prohibitions
- **No Direct Environment Access:** Usage of `std::env::var`, `std::env::var_os`, or similar is strictly banned in all business logic and request handlers.
- **No Hardcoded Defaults:** The code cannot use methods like `.unwrap_or(...)` to fallback to default values in the business logic if the config parameter is intended to be required.

## Testing Strategy
- **Unit Tests:**
  - Provide a mock valid YAML/Env setup and assert successful parsing into `AppConfig`.
  - Provide an invalid config (e.g., missing required fields) and assert that initialization returns an error or panics.
- **Linting/Static Analysis:**
  - Search source files to ensure no usages of `std::env::var` exist outside of the `config` module initialization.
