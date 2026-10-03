# Spec 05: Health Monitoring Sidecar (HaMS)

**Status:** `draft`
**Derived From:** [Backend Foundations PRD](../prds/backend-foundations-prd.md)

## Objective
Implement a separate sidecar process (HaMS) embedded within the backend to serve telemetry, metrics, and health probes independently from the main business logic webservice.

## Requirements

### 1. Dedicated Listener
- The HaMS sidecar must run as an independent asynchronous task (e.g., spawned via `tokio::spawn`).
- It must bind to a completely separate port (e.g., `8079`) configured via `AppConfig`.

### 2. Probes and Metrics
- **Liveness & Readiness:** It must expose `/health/live` and `/health/ready` endpoints suitable for Kubernetes probing.
- **Metrics Exposure:** It must expose a `/metrics` endpoint to serve Prometheus metrics scraped from the application state.

### 3. Lifecycle Interlock
- The HaMS sidecar must be instantiated immediately *after* the configuration is loaded and validated, but *before* database connections or the main Axum listener are established. This ensures startup errors can be observed.

## Testing Strategy
- **Integration Tests:**
  - Launch the full backend process locally.
  - Assert that `curl http://localhost:8079/health/live` returns a 200 OK while the main application might be blocked or bootstrapping.
  - Assert that `curl http://localhost:8079/metrics` returns valid Prometheus text-formatted output.
