use super::{Connection, Error};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String, 
    pub email: String,
}


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

pub fn create_user(conn: &Connection, user: &User) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email],
    )
    .map(drop)
}

pub fn list_users(conn: &Connection) -> Result<Vec<User>, Error> {
    let res = conn
        .query("SELECT name, email FROM users", &[])?
        .into_iter()
        .map(|row| User{
            name:row.get(0), 
            email:row.get(1)
        })
        .collect();
    Ok(res)
}
