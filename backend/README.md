# Backend

The services pillar of `heliosproto`. Python 3.12, FastAPI, Postgres, Redis, Celery.

See the [Pillar 3 section of ARCHITECTURE.md](../ARCHITECTURE.md#5-pillar-3--backend-python-312--fastapi) for the full design.

## Layout

| Path | Purpose |
|---|---|
| `indexer/` | Soroban event indexer (RPC → Postgres) |
| `relayer/` | G→C funding bridge + paymaster off-chain sponsor |
| `recovery/` | Guardian coordination for social recovery |
| `notifications/` | WebSocket + push (FCM/APNS) + email fan-out |
| `anchors/` | SEP-24 / SEP-31 / SEP-10 adapter framework |
| `prices/` | Reflector and oracle adapters, normalized USD prices |
| `api/` | REST + GraphQL gateway |
| `workers/` | Celery workers for backfills, fan-out, polling |

## Prerequisites

- Python 3.12+
- Postgres 16+ (Docker Compose ships one)
- Redis 7+ (Docker Compose ships one)

## Working in this pillar

Each service is its own package with its own `pyproject.toml`. From inside any service directory:

```bash
python -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"
ruff check
mypy --strict
pytest
uvicorn main:app --reload   # for HTTP services
```

A `docker-compose.yml` at the pillar root will boot Postgres + Redis + all services together once the first service lands.

## Notes for Python contributors

The architecture deliberately keeps Python here so contributors comfortable with Django/Flask/FastAPI can be productive without touching Rust or the smart contract layer. The on-chain logic lives in `contracts/`; backend services interact with it via the Soroban RPC HTTP API — no Rust required.
