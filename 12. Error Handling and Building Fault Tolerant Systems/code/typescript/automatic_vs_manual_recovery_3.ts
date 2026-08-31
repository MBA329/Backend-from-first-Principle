// TypeScript, Automatic vs manual recovery
export class HTTPError extends Error {
    constructor(public statusCode: number, message?: string) {
        super(message);
    }
}

const emailClient = {
    async send(to: string, subject: string, body: string): Promise<void> {
        // mock implementation
    }
};

export async function sendEmailWithRetry(to: string, subject: string, body: string): Promise<void> {
    const maxRetries = 5;
    const baseDelay = 1000; // 1 second in ms

    for (let attempt = 0; attempt < maxRetries; attempt++) {
        try {
            await emailClient.send(to, subject, body);
            return; // success
        } catch (err: any) {
            if (!isRetryable(err)) {
                throw new Error(`permanent failure: ${err.message}`);
            }

            // Exponential backoff: 1s, 2s, 4s, 8s, 16s
            const wait = baseDelay * (1 << attempt);
            // Add jitter (+/-20%) to prevent thundering herd
            const jitter = Math.floor(Math.random() * (wait / 5));
            const totalWait = wait + jitter;
            
            await new Promise(resolve => setTimeout(resolve, totalWait));

            console.warn("email send failed, retrying", {
                attempt: attempt + 1,
                wait_ms: totalWait,
                error: err.message
            });
        }
    }
    throw new Error(`all ${maxRetries} retries exhausted`);
}

function isRetryable(err: any): boolean {
    // Retry on 429, 503, network errors; not on 400, 401, 422
    if (err instanceof HTTPError) {
        return err.statusCode === 429 || err.statusCode >= 500;
    }
    return true; // network errors are always retryable
}
