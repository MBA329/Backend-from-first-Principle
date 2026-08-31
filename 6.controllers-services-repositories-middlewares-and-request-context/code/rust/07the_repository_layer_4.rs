use sqlx::PgPool;

pub struct BookRepo {
    db: PgPool,
}

#[derive(sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

impl BookRepo {
    // ONE method, ONE shape of result: all books, sorted.
    pub async fn find_all_books(&self, sort: &str) -> Result<Vec<Book>, sqlx::Error> {
        // Build the query from the data the service handed down.
        let query = format!("SELECT id, title, author FROM books ORDER BY {}", sort);
        let books = sqlx::query_as::<_, Book>(&query)
            .fetch_all(&self.db)
            .await?;
        Ok(books)
    }

    // A SEPARATE method for the single-book case. No optional toggles.
    pub async fn find_book_by_id(&self, id: i32) -> Result<Book, sqlx::Error> {
        let book = sqlx::query_as::<_, Book>("SELECT id, title, author FROM books WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(book)
    }
}
