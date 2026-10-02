# Spec 01: Backend Foundations & Bootstrapping

**Status**: `draft`
**Derived From**: [Backend Foundations PRD](../prds/backend-foundations-prd.md)

## Objective
Implement the core Rust microservice orchestrator, integrating fail-fast configuration, the HaMS health sidecar, a PostgreSQL connection pool (with pgvector check), and the Axum webservice structure. This represents the underlying framework that business logic and tools will rely on.

## Components & Technical Requirements

### 1. `config` (Configuration Loader)
- **Module Path**: `src/config.rs`
- **Responsibilities**: Load environment variables and YAML configurations into a strongly typed `AppConfig` struct.
- **Constraints**:
  - Must not use fallback `.unwrap_or()` defaults for critical runtime properties (like database URLs or API bindings).
  - Must validate the struct payload during instantiation (`validate()` method).

### 2. `hams_tools` (Health Sidecar Harness)
- **Module Path**: `src/hams_tools.rs`
- **Responsibilities**: Initialize the external `hams` library.
- **Constraints**:
  - Must spawn on the designated sidecar port (e.g., `8079`).
  - Must expose standard readiness and liveness probes.
  - Must securely register Axum/Application state prometheus metrics into the HaMS registry.

### 3. `db` (Database Connection & Verification)
- **Module Path**: `src/db.rs`
- **Responsibilities**: Create the SQLx connection pool and apply auto-migrations.
- **Constraints**:
  - Run `.up.sql` scripts located in `./migrations/`.
  - Provide a function `verify_pgvector_extension()` that queries the `pg_extension` table to ensure `pgvector` exists before the service becomes ready.

### 4. `webserver` (Axum REST Orchestration)
- **Module Path**: `src/webserver.rs`
- **Responsibilities**: Bind the `axum::Router` on the primary application port (e.g., `8080`).
- **Constraints**:
  - Accept an `AppState` struct injected via Axum extensions.

## TDD Execution Strategy

### Unit Tests
1. **Config Validation Tests**:
   - Write a test passing a missing/invalid database string, expecting a validation error (Fail-Fast).
   - Write a test with valid configuration, verifying it parses successfully.
2. **State Injection Tests**:
   - Mock a database pool and test that an axum handler can successfully extract `AppState`.

### Integration Tests (Robot Framework / API Tests)
1. **Service Readiness Journey**:
   - Spin up the microservice container along with a PostgreSQL container.
   - Assert `GET http://localhost:8079/health/ready` returns `200 OK` (proving HaMS and DB connections succeed).
2. **Metrics Emission Journey**:
   - Assert `GET http://localhost:8079/metrics` returns a body containing initialized prometheus gauges for the application version.
