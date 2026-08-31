use redis::AsyncCommands;

pub async fn store_session(mut con: redis::aio::Connection, session_id: &str, user_id: &str) -> redis::RedisResult<()> {
    con.set_ex(format!("session:{}", session_id), user_id, 24 * 3600).await
}

pub async fn get_session(mut con: redis::aio::Connection, session_id: &str) -> redis::RedisResult<String> {
    con.get(format!("session:{}", session_id)).await
}
