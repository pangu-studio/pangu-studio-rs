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