pub const QUERIES: [&str; 3] = [
    r#"
        CREATE TABLE IF NOT EXISTS Accounts (

            account_id SERIAL PRIMARY KEY,
            pfp_id SERIAL,
            username VARCHAR(20) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            role VARCHAR(15) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        
        );
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Tokens (
            token_id SERIAL PRIMARY KEY NOT NULL UNIQUE,
            account_id INTEGER NOT NULL,
            token VARCHAR(260) NOT NULL UNIQUE,
            role VARCHAR(15) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (account_id) REFERENCES Accounts(account_id) ON DELETE CASCADE
        );
    "#,
    r#"
        CREATE TABLE IF NOT EXISTS Files (
            file_id SERIAL PRIMARY KEY NOT NULL,
            file_blob BYTEA NOT NULL,
            size INT NOT NULL,
            file_type VARCHAR(256) NOT NULL,

            account_id INT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            
            FOREIGN KEY (account_id) REFERENCES Accounts(account_id)
        );
    "#,
];


// CREATE TABLE IF NOT EXISTS Accounts (
//     account_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
//     pfp_id INT DEFAULT NULL,
//     username VARCHAR(20) NOT NULL UNIQUE,
//     password VARCHAR(255) NOT NULL,
//     role VARCHAR(20) NOT NULL,
//     created_at DATETIME DEFAULT CURRENT_TIMESTAMP
// ) ENGINE=InnoDB;