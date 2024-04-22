// sql schema
// CREATE TABLE csrf_tokens (
//     session_id INTEGER PRIMARY KEY AUTOINCREMENT,
//     csrf_token TEXT NOT NULL,
//     timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//     expiry_timestamp TIMESTAMP NOT NULL
// );

// insert item
// INSERT INTO csrf_tokens ('token1', timestamp('now', '+5 minutes'))

use rusqlite::{Connection, Result};

// Generate and Store CSRF Token
// INSERT INTO csrf_tokens (csrf_token, timestamp, expiry_timestamp) VALUES ('token1', datetime('now'), datetime('now', '+5 minutes'));
fn generate_and_store_csrf_token(conn: &Connection, session_id: &str) -> Result<[int64, String]> {
    let csrf_token = generate_csrf_token();
    
    conn.execute(
        // "INSERT INTO csrf_tokens (csrf_token) VALUES (?1, ?2)",
        "INSERT INTO csrf_tokens (csrf_token, timestamp, expiry_timestamp) VALUES (?1, datetime('now'), datetime('now', '+5 minutes'))",
        &[&csrf_token],
    )?;
    
    Ok([csrf_token])
}


// Retrieve the CSRF token from the database based on the session ID
// and validate it against the token sent in the request.
fn get_stored_csrf_token(conn: &Connection, session_id: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT csrf_token FROM csrf_tokens WHERE session_id = ?1")?;
    let mut rows = stmt.query(&[&session_id])?;
    
    let csrf_token: Option<String> = match rows.next()? {
        Some(row) => row.get(0)?,
        None => None,
    };
    
    Ok(csrf_token)
}

//Compare the retrieved CSRF token with the one sent in the form submission request.
fn is_valid_csrf_token(request_csrf_token: &str, stored_csrf_token: &str) -> bool {
    request_csrf_token == stored_csrf_token
}

/// Cleanup mechanism to remove expired CSRF tokens, preventing
///  the database from growing indefinitely and token reuse.
fn cleanup_expired_tokens(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM csrf_tokens WHERE expiry_timestamp < CURRENT_TIMESTAMP", [])?;
    Ok(())
}