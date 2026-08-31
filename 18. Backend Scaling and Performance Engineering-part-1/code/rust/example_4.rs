use sqlx::{PgPool, FromRow};
use std::collections::{HashSet, HashMap};

#[derive(FromRow, Clone)]
pub struct Post { pub id: i64, pub author_id: i64 }
#[derive(FromRow, Clone)]
pub struct Author { pub id: i64 }
pub struct PostWithAuthor { pub post: Post, pub author: Option<Author> }

pub async fn get_posts_with_authors(db: &PgPool) -> Result<Vec<PostWithAuthor>, sqlx::Error> {
    let posts: Vec<Post> = sqlx::query_as("SELECT * FROM posts").fetch_all(db).await?;
    
    let author_ids: Vec<i64> = posts.iter()
        .map(|p| p.author_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let authors: Vec<Author> = sqlx::query_as("SELECT * FROM authors WHERE id = ANY($1)")
        .bind(&author_ids)
        .fetch_all(db)
        .await?;

    let author_map: HashMap<i64, Author> = authors.into_iter().map(|a| (a.id, a)).collect();

    let result = posts.into_iter().map(|post| {
        let author = author_map.get(&post.author_id).cloned();
        PostWithAuthor { post, author }
    }).collect();

    Ok(result)
}
