# Contentstack API Reference

## Delivery (CDN) Auth Headers
| Header | Value |
|---|---|
| `api_key` | Stack API key |
| `access_token` | Delivery token (NOT `authorization`) |
| `environment` | Publishing environment name |

> Header name is `access_token`, not `delivery_token` or `authorization` - a common mistake.

## Base URLs by Region
Handled by `Region::delivery_base_url()` and `Region::management_base_url()` in `src/client/config.rs`.
Default region: `AwsNa` → `https://cdn.contentstack.io` (delivery) / `https://api.contentstack.io` (management).

## Endpoint Patterns
```
GET /content_types/{content_type_uid}/entries          # list
GET /content_types/{content_type_uid}/entries/{uid}    # single
```

## Query Parameter Quirk
Contentstack requires the `query` filter as a **JSON-serialized string**, not a nested object:
```
?query={"title":"Hello"}   ✓
?query[title]=Hello         ✗
```
This is why `Query` (`HashMap<String, Value>`) is serialized via `serde_json::to_string()` inside
`From<GetManyParams> for SerializedGetManyParams` before being passed to `.query()`.

## Response Shapes
- List: `{ "entries": [...], "count": N }` — `count` only present when `include_count: true`
- Single: `{ "entry": { ... } }`
- Every entry system fields: `uid`, `title`, `locale`, `created_at`, `updated_at`, `created_by`, `updated_by`, `_version`

## Management API (not yet implemented)
- Requires `auth_token` header (user session token), not a delivery token
- CRUD on entries, assets, content types, publishing/releases
- Planned module: `src/client/management/`
