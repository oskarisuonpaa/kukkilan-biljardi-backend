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