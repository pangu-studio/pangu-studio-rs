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
    update_time TIMESTAMP,
    deleted BOOLEAN NOT NULL DEFAULT 0
);
create index idx_name on endpoints(name);

CREATE TABLE IF NOT EXISTS dns_providers (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    api_key TEXT NOT NULL,
    api_secret INTEGER NOT NULL,
    provider_type TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT 0
);
create index idx_name on endpoints(name);

CREATE TABLE IF NOT EXISTS ssl_certificates (
    id INTEGER PRIMARY KEY NOT NULL,
    domains TEXT NOT NULL,
    cert_chain TEXT NOT NULL,
    private_key TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT 0,
    create_time TIMESTAMP NOT NULL,
    update_time TIMESTAMP
);