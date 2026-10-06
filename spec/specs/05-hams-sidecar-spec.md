# Spec 05: Health Monitoring Sidecar (HaMS)

**Status:** `complete`
**Derived From:** [Backend Foundations PRD](../prds/backend-foundations-prd.md)

## Objective
Implement a separate sidecar process (HaMS) embedded within the backend to serve telemetry, metrics, and health probes independently from the main business logic webservice.

## Requirements

### 1. Dedicated Listener & Crate Integration
- The HaMS sidecar must utilize the external `hams` crate.
- Configuration is provided through a nested `hams: HamsConfig` block within `AppConfig`.
- It binds to a separate port managed by the crate's internal configuration.

### 2. Probes and Metrics
- **Liveness & Readiness:** The `hams` crate automatically exposes `/health/live` and `/health/ready` endpoints. A custom `Manual` ready signal (`db-connected`) must be initialized and toggled to `true` upon successful database connection pool verification.
- **Metrics Exposure:** The crate automatically handles Prometheus metric exposure.

### 3. Lifecycle Interlock & Graceful Shutdown
- The HaMS sidecar must be instantiated immediately *after* the configuration is loaded and validated, but *before* database connections or the main Axum listener are established.
- A `tokio_util::sync::CancellationToken` must be instantiated.
- The HaMS instance must register a shutdown closure that calls `cancel()` on the token.
- The main Axum server must utilize `.with_graceful_shutdown(ct.cancelled())` to ensure clean termination.

## Testing Strategy
- **Integration Tests:**
  - Launch the full backend process locally.
  - Assert that `curl http://localhost:8079/health/live` returns a 200 OK while the main application might be blocked or bootstrapping.
  - Assert that `curl http://localhost:8079/metrics` returns valid Prometheus text-formatted output.
