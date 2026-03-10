# Roadmap & Planned Work

## Not Yet Implemented
- `ClientError` not wired into client methods - they return `reqwest::Error` directly
- Management API client (`src/client/management/`)
- Rate limiting - plan: `governor` (GCRA) + `tokio::sync::Semaphore` together
  - `governor` enforces strict req/s; Semaphore caps concurrency - both needed
- Streaming helpers (`response.bytes_stream()` is the reqwest 0.13 API)
- Tests - use `wiremock` (v0.6), NOT `mockito`
  - wiremock: async-native, real local HTTP server, scoped mock guards, call count expectations

## Planned Dependencies
```toml
reqwest-middleware = "0.5"   # middleware chain
reqwest-retry     = "0.7"   # exponential backoff via RetryTransientMiddleware
reqwest-tracing   = "0.5"   # auto tracing spans
governor          = "0.10"  # GCRA rate limiter
tracing           = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
wiremock = "0.6"
```

## CI / Release Flow
```
feature/* --PR--> main --release-plz--> opens Release PR (Cargo.toml bump + CHANGELOG)
                        --merge PR-----> tags + publishes to crates.io automatically
```
- `.github/workflows/ci.yml` - PR gate: fmt + clippy (-D warnings) + test + `ci-pass` rollup
- `.github/workflows/release-plz.yml` - triggers on push to main: opens/updates release PR, tags, publishes
- `release-plz.toml` - config: changelog, git releases, crates.io publish
- Required secrets: `CARGO_REGISTRY_TOKEN`, `GITHUB_TOKEN` (auto-provided)
- Conventional Commits required: `feat:` → minor, `fix:` → patch, `feat!:` → major

## Error Handling Plan
Expand `ClientError` to cover:
- `Http(reqwest::Error)` via `#[from]`
- `RateLimit` (map 429 responses)
- `Unauthorized` (map 401)
- `Parse(serde_json::Error)` via `#[from]`
- Replace all `reqwest::Error` returns in client methods with `crate::error::Result<T>`
