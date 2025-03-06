pub const QUERIES: [&str; 4] = [
    r#"
        CREATE TABLE IF NOT EXISTS Accounts (
            account_id INTEGER PRIMARY KEY AUTOINCREMENT,
            pfp_id INTEGER,
            username VARCHAR(20) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            role VARCHAR(15) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Tokens (
            token_id INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id INTEGER NOT NULL,
            token VARCHAR(260) NOT NULL UNIQUE,
            role VARCHAR(15) NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (account_id) REFERENCES Accounts(account_id) ON DELETE CASCADE
        );
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Files (
            file_id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_blob BLOB NOT NULL,
            size INTEGER NOT NULL,
            file_type VARCHAR(256) NOT NULL,
            account_id INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (account_id) REFERENCES Accounts(account_id)
        );
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS DockerConnections (
            docker_connetion_id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR(256) NOT NULL,
            host VARCHAR(256) NOT NULL,
            protocol VARCHAR(32) NOT NULL,
            cert_path VARCHAR(256),
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#,
];