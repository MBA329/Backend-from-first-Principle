use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmailJob {
    pub to: String,
    pub subject: String,
    pub template: String,
}

pub async fn enqueue_email(mut con: redis::aio::Connection, job: EmailJob) -> redis::RedisResult<()> {
    let data = serde_json::to_string(&job).unwrap();
    con.lpush("queue:emails", data).await
}

pub async fn email_worker(mut con: redis::aio::Connection) {
    loop {
        let result: redis::RedisResult<Vec<String>> = con.brpop("queue:emails", 0.0).await;
        if let Ok(res) = result {
            if res.len() == 2 {
                let job: EmailJob = serde_json::from_str(&res[1]).unwrap();
                // send_email(job) takes 300ms, but no user is waiting
            }
        }
    }
}
