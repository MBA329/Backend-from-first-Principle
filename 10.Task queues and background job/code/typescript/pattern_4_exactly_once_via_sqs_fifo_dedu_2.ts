// TypeScript, Idempotent multi-step task with full transaction rollback
import { PoolClient } from "pg";

interface DeleteAccountPayload {
    UserID: string;
}

interface Task {
    Payload(): string;
}

export class AccountHandler {
    private db: any; // e.g. Pool

    async handleDeleteAccount(t: Task): Promise<void> {
        const p: DeleteAccountPayload = JSON.parse(t.Payload());

        // Check if already deleted (idempotency guard)
        const exists = await this.db.userExists(p.UserID);
        if (!exists) {
            return; // already deleted on a previous attempt, ACK cleanly
        }

        // Wrap all DB writes in a single transaction
        return this.db.withTransaction(async (tx: PoolClient) => {
            const steps = [
                () => tx.query('DELETE FROM user_projects WHERE user_id = $1', [p.UserID]),
                () => tx.query('DELETE FROM user_sessions WHERE user_id = $1', [p.UserID]),
                () => tx.query('DELETE FROM user_assets WHERE user_id = $1', [p.UserID]),
                () => tx.query('DELETE FROM user_account WHERE id = $1', [p.UserID]),
            ];
            
            for (const step of steps) {
                await step(); // triggers full rollback on throw; task retried from scratch
            }
            // all steps succeeded -> commit -> ACK
        });
    }
}
