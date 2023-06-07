INSERT INTO endpoints (
        id,
        name,
        host,
        port,
        secret,
        endpoint_type,
        host_type,
        description,
        create_time,
        deleted
    )
VALUES(
        '1',
        'endpoint1',
        'localhost',
        2345,
        'abdefefe',
        'server',
        'http',
        "hosts",
        '2023-06-07T10:28:33.774052+00:00',
        0
    );
CREATE TABLE IF NOT EXISTS dns_providers (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    api_key TEXT NOT NULL,
    api_secret INTEGER NOT NULL,
    provider_type TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT 0
);
INSERT INTO dns_providers(
        id,
        name,
        api_key,
        api_secret,
        provider_type,
        deleted
    )
VALUES(
        1,
        'dnspod',
        '123456',
        'your secret',
        'dnspod',
        0
    );