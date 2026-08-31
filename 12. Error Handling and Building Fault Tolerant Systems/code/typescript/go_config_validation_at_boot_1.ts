// TypeScript, Config validation at boot
export interface Config {
    DatabaseURL: string;
    OpenAIKey: string;
    JWTSecret: string;
    ResendAPIKey: string;
}

// mustLoad throws if any required variable is missing.
// Call this once in main() before starting the server.
export function mustLoad(): Config {
    const required = [
        "DATABASE_URL",
        "OPENAI_API_KEY",
        "JWT_SECRET",
        "RESEND_API_KEY",
    ];

    const missing = required.filter(key => !process.env[key]);
    if (missing.length > 0) {
        // Crash immediately, loud and clear
        throw new Error(`[FATAL] missing required env vars: ${missing.join(", ")}`);
    }

    return {
        DatabaseURL: process.env.DATABASE_URL!,
        OpenAIKey: process.env.OPENAI_API_KEY!,
        JWTSecret: process.env.JWT_SECRET!,
        ResendAPIKey: process.env.RESEND_API_KEY!,
    };
}

// main.ts
if (require.main === module) {
    const cfg = mustLoad(); // throws here if config invalid
    // const server = newServer(cfg);
    // server.listen();
}
