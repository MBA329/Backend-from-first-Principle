// Rust, Idempotent multi-step task with full transaction rollback
use sqlx::{PgPool, Postgres, Transaction};
use serde::Deserialize;

#[derive(Deserialize)]
struct DeleteAccountPayload {
    user_id: String,
}

pub struct Task {
    payload: String,
}

impl Task {
    fn payload(&self) -> &str {
        &self.payload
    }
}

pub struct AccountHandler {
    db: PgPool,
}

impl AccountHandler {
    pub async fn handle_delete_account(&self, t: &Task) -> Result<(), Box<dyn std::error::Error>> {
        let p: DeleteAccountPayload = serde_json::from_str(t.payload())?;

        // Check if already deleted (idempotency guard)
        let exists = self.user_exists(&p.user_id).await?;
        if !exists {
            return Ok(()); // already deleted on a previous attempt, ACK cleanly
        }

        // Wrap all DB writes in a single transaction
        let mut tx = self.db.begin().await?;
        
        self.delete_user_projects(&mut tx, &p.user_id).await?;
        self.delete_user_sessions(&mut tx, &p.user_id).await?;
        self.delete_user_assets(&mut tx, &p.user_id).await?;
        self.delete_user_account(&mut tx, &p.user_id).await?;
        
        tx.commit().await?; // all steps succeeded -> commit -> ACK
        
        Ok(())
    }

    async fn user_exists(&self, _user_id: &str) -> Result<bool, sqlx::Error> {
        Ok(true) // placeholder
    }

    async fn delete_user_projects(&self, _tx: &mut Transaction<'_, Postgres>, _user_id: &str) -> Result<(), sqlx::Error> { Ok(()) }
    async fn delete_user_sessions(&self, _tx: &mut Transaction<'_, Postgres>, _user_id: &str) -> Result<(), sqlx::Error> { Ok(()) }
    async fn delete_user_assets(&self, _tx: &mut Transaction<'_, Postgres>, _user_id: &str) -> Result<(), sqlx::Error> { Ok(()) }
    async fn delete_user_account(&self, _tx: &mut Transaction<'_, Postgres>, _user_id: &str) -> Result<(), sqlx::Error> { Ok(()) }
}
