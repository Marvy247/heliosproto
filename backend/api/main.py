from __future__ import annotations

from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI(
    title="Helios Protocol API",
    description="REST + GraphQL gateway for the Helios wallet stack",
    version="0.1.0",
)


class HealthResponse(BaseModel):
    status: str
    service: str
    version: str


@app.get("/health", response_model=HealthResponse)
async def health() -> HealthResponse:
    return HealthResponse(status="ok", service="heliosproto-api", version="0.1.0")
