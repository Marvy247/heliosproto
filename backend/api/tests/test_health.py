from __future__ import annotations

from fastapi.testclient import TestClient

from main import app


def test_health_endpoint_returns_ok() -> None:
    client = TestClient(app)
    response = client.get("/health")

    assert response.status_code == 200
    payload = response.json()
    assert payload["status"] == "ok"
    assert payload["service"] == "heliosproto-api"
    assert payload["version"] == "0.1.0"
