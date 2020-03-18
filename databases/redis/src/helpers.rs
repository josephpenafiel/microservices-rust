use crate::prelude::*;
use crate::SESSIONS;
pub fn add_session(conn: &Connection, token: &str, uid: &str) -> Result<(), RedisError> {
    conn.hset(SESSIONS, token, uid)
}

pub fn remove_session(conn: &Connection, token: &str) -> Result<(), RedisError> {
    conn.hdel(SESSIONS, token)
}

pub fn list_sessions(conn: &Connection) -> Result<HashMap<String, String>, RedisError> {
    conn.hgetall(SESSIONS)
}
