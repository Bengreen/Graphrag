# Spec 03: Database & Storage Layer

**Status:** `complete`
**Derived From:** [Backend Foundations PRD](../prds/backend-foundations-prd.md)

## Objective
Establish a robust interaction layer with PostgreSQL using `sqlx`, ensuring proper connection pooling, automatic migrations, and support for vector embeddings (`pgvector`).

## Requirements

### 1. Connection Pooling
- The application must initialize an `sqlx::PgPool` at startup.
- The pool size and timeout settings must be configurable via `AppConfig`.

### 2. Startup Verification & Migrations
- **pgvector Verification:** Before accepting traffic, the startup sequence must execute a query to verify the `pgvector` extension is enabled in the database.
- **Automatic Migrations:** The application must automatically execute all pending `.up.sql` migrations sequentially using `sqlx::migrate!()` during startup.
- **Failure Condition:** If verification fails or migrations fail to apply, the application must abort startup immediately.

### 3. State Management Constraints
- Never manually alter schemas pending an `sqlx` migration. The application assumes complete ownership of database schema state via its migration files.

## Testing Strategy
- **Integration Tests:**
  - Spin up a fresh Postgres container.
  - Assert that starting the application successfully applies all migrations.
  - Assert that `pgvector` validation passes.
  - Assert that attempting to start against a database without `pgvector` installed fails gracefully.
