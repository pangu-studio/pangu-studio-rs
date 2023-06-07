CREATE TABLE IF NOT EXISTS endpoints (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    secret TEXT NOT NULL,
    endpoint_type TEXT NOT NULL,
    host_type TEXT NOT NULL,
    description TEXT,
    create_time TIMESTAMP NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS dns_providers (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    api_key TEXT NOT NULL,
    api_secret INTEGER NOT NULL,
    provider_type TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT 0
);