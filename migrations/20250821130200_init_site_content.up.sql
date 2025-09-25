CREATE TABLE IF NOT EXISTS contact_info (
    id TINYINT UNSIGNED PRIMARY KEY,
    address VARCHAR(255) NOT NULL,
    phone VARCHAR(64) NOT NULL,
    email VARCHAR(255) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CHECK (id = 1)
);
INSERT INTO contact_info (id, address, phone, email)
VALUES (
        1,
        'Latojantie 6, 15270 Kukkila',
        '040 0421453',
        'vilkasprosnookervaraukset@gmail.com'
    ) ON DUPLICATE KEY
UPDATE address =
VALUES(address),
    phone =
VALUES(phone),
    email =
VALUES(email);
CREATE TABLE IF NOT EXISTS notices (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)
);
CREATE TABLE IF NOT EXISTS opening_hours (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    weekday TINYINT UNSIGNED NOT NULL,
    opens_at TIME NOT NULL,
    closes_at TIME NOT NULL,
    UNIQUE KEY uq_weekday (weekday),
    CHECK (
        weekday BETWEEN 1 AND 7
    ),
    CHECK (opens_at < closes_at)
);
CREATE TABLE IF NOT EXISTS opening_exceptions (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    date DATE NOT NULL UNIQUE,
    is_closed TINYINT(1) NOT NULL DEFAULT 0,
    opens_at TIME NULL,
    closes_at TIME NULL,
    CHECK (
        (
            is_closed = 1
            AND opens_at IS NULL
            AND closes_at IS NULL
        )
        OR (
            is_closed = 0
            AND opens_at IS NOT NULL
            AND closes_at IS NOT NULL
            AND opens_at < closes_at
        )
    )
);
CREATE TABLE IF NOT EXISTS media (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    is_background BOOLEAN NOT NULL DEFAULT FALSE,
    kind TEXT NOT NULL CHECK (kind IN ('image', 'video')),
    file_url TEXT NOT NULL,
    alt_text TEXT NOT NULL,
    width INT UNSIGNED,
    height INT UNSIGNED,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);