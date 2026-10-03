# Spec 04: Axum Web Service

**Status:** `draft`
**Derived From:** [Backend Foundations PRD](../prds/backend-foundations-prd.md)

## Objective
Configure the primary `Axum` REST API framework, integrating the database pool and configuration into shared application state.

## Requirements

### 1. Framework Setup
- The application will utilize `axum` running on the `tokio` async runtime.
- The web server must bind to the host and port specified in the `AppConfig` (e.g., typically `0.0.0.0:8080`).

### 2. Application State (`AppState`)
- An `AppState` struct must be defined and injected into handlers using axum's state extraction (`axum::extract::State`).
- The `AppState` must encapsulate, at minimum:
  - A cloneable reference to the `sqlx::PgPool`.
  - A read-only reference to the `AppConfig`.

### 3. Routing Organization
- Application routes must be declaratively defined and modularized based on domain function.
- Handlers should return typed JSON responses and standard HTTP status codes.

## Testing Strategy
- **Unit Tests:**
  - Create mock `AppState`.
  - Provide a test harness (using `axum::test_helpers` or `tower::ServiceExt`) to invoke endpoints and assert expected HTTP status codes and JSON payloads without requiring a real network bind.
