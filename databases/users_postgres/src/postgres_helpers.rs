use super::{Connection, Error};

pub fn create_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE users (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        email VARCHAR NOT NULL
    )",
        &[],
    )
    .map(drop)
}

pub fn create_user(conn: &Connection, name: &str, email: &str) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&name, &email],
    )
    .map(drop)
}

pub fn list_users(conn: &Connection) -> Result<Vec<(String, String)>, Error> {
    let res = conn
        .query("SELECT name, email FROM users", &[])?
        .into_iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();
    Ok(res)
}
