DROP TABLE IF EXISTS dns_providers;
CREATE TABLE IF NOT EXISTS dns_providers (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    api_key TEXT NOT NULL,
    api_secret TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT 0,
    create_time TIMESTAMP NOT NULL,
    update_time TIMESTAMP
);
create index IF NOT EXISTS idx_provider_name on endpoints(name);