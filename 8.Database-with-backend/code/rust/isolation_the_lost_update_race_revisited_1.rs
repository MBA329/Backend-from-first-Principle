use sqlx::{PgPool, Error};

pub async fn transfer_funds(pool: &PgPool, a: &str, b: &str) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    
    // In sqlx, transactions rollback automatically if dropped before commit
    // so it acts as our safety net implicitly!
    
    sqlx::query!("UPDATE accounts SET balance = balance - 500 WHERE id = $1", a)
        .execute(&mut *tx).await?;
    // if error occurs -> early return -> tx dropped -> A is untouched
        
    sqlx::query!("UPDATE accounts SET balance = balance + 500 WHERE id = $1", b)
        .execute(&mut *tx).await?;
    // if error occurs -> early return -> tx dropped -> both reverted
        
    tx.commit().await?; // only here do both writes become permanent
    Ok(())
}
