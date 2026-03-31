CREATE TABLE telemetry (
    id SERIAL PRIMARY KEY,
    device_id TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    payload JSONB NOT NULL
);
