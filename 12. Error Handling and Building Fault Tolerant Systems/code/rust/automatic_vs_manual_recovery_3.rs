// Rust, Automatic vs manual recovery
use std::error::Error;
use std::time::Duration;
use rand::Rng;
use tokio::time::sleep;

#[derive(Debug)]
pub struct HttpError {
    pub status_code: u16,
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP Error: {}", self.status_code)
    }
}

impl Error for HttpError {}

struct EmailClient;
impl EmailClient {
    async fn send(&self, _to: &str, _subject: &str, _body: &str) -> Result<(), Box<dyn Error>> {
        // mock implementation
        Ok(())
    }
}

pub async fn send_email_with_retry(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
    let email_client = EmailClient;
    let max_retries = 5;
    let base_delay = Duration::from_secs(1);

    for attempt in 0..max_retries {
        match email_client.send(to, subject, body).await {
            Ok(_) => return Ok(()), // success
            Err(err) => {
                if !is_retryable(&*err) {
                    return Err(format!("permanent failure: {}", err).into());
                }

                // Exponential backoff: 1s, 2s, 4s, 8s, 16s
                let wait_ms = base_delay.as_millis() as u64 * (1 << attempt);
                // Add jitter (+/-20%) to prevent thundering herd
                let jitter = rand::thread_rng().gen_range(0..=(wait_ms / 5));
                let wait = Duration::from_millis(wait_ms + jitter);
                
                sleep(wait).await;

                log::warn!("email send failed, retrying attempt={} wait_ms={} error={}", 
                    attempt + 1, wait.as_millis(), err);
            }
        }
    }
    Err(format!("all {} retries exhausted", max_retries).into())
}

fn is_retryable(err: &(dyn Error + 'static)) -> bool {
    // Retry on 429, 503, network errors; not on 400, 401, 422
    if let Some(http_err) = err.downcast_ref::<HttpError>() {
        return http_err.status_code == 429 || http_err.status_code >= 500;
    }
    true // network errors are always retryable
}
