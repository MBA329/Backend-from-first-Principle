pub struct BookService {
    repo: Box<dyn BookRepoTrait>,
    mailer: Box<dyn MailerTrait>,
}

impl BookService {
    // Notice: no request, no response, no status codes.
    // You cannot tell from this signature that it serves an API.
    pub async fn list_books(&self, sort: &str) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        // Orchestration: call the repository for the data it needs.
        let books = self.repo.find_all_books(sort).await?;
        // Could also: enrich, merge other repo calls, send notifications...
        Ok(books)
    }

    // A service that needs no database at all is perfectly valid:
    pub async fn notify_owner(&self, email: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mailer.send(email, "Your book was added").await
    }
}

// Sketches
pub trait BookRepoTrait {
    fn find_all_books(&self, sort: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Book>, Box<dyn std::error::Error>>>>>;
}
pub trait MailerTrait {
    fn send(&self, email: &str, msg: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>>>;
}
pub struct Book {}
