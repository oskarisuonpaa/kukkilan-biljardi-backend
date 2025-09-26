CREATE TABLE IF NOT EXISTS admin_users (
    id CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    last_login DATETIME(6),
    INDEX idx_username (username),
    INDEX idx_is_active (is_active)
);
-- Insert default admin user only if it doesn't exist
INSERT IGNORE INTO admin_users (username, password_hash, email)
VALUES (
        'admin',
        '$2b$12$X8ZUL5QGpSHtCB7yJfQl8uvVX.rQA4DF2O5BnpxXEK8L6GnpZ7.gO',
        'admin@kukkilan-biljardi.fi'
    );