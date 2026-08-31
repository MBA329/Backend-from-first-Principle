import { z } from "zod";
import * as dotenv from "dotenv";

// Config holds all runtime application settings.
// The `z.object` schema enforces rules at startup, this is the
// single most important safeguard for config management.
const ConfigSchema = z.object({
  // Application settings
  Port: z.coerce.number().min(1).max(65535).default(8080),
  LogLevel: z.enum(["debug", "info", "warn", "error"]).default("info"),
  Env: z.enum(["development", "staging", "production"]).default("development"),

  // Database config (sensitive)
  DBHost: z.string().min(1),
  DBPort: z.coerce.number().default(5432),
  DBUser: z.string().min(1),
  DBPassword: z.string().min(1),
  DBName: z.string().min(1),
  DBPoolSize: z.coerce.number().min(1).default(10), // dev=10, prod=50

  // External services (sensitive)
  StripeAPIKey: z.string().min(1),

  // Feature flags (optional, default false)
  NewCheckoutEnabled: z.coerce.boolean().default(false),
});

export type Config = z.infer<typeof ConfigSchema>;

// DatabaseURL constructs the connection URL from the parts.
export function databaseURL(c: Config): string {
  return `postgres://${c.DBUser}:${c.DBPassword}@${c.DBHost}:${c.DBPort}/${c.DBName}`;
}

// Load reads env vars, applies defaults, then VALIDATES before returning.
// Called once at startup, fail loudly here, never silently in production.
export function loadConfig(): Config {
  // In local dev, load .env into the OS environment.
  // In production this is a no-op (vars already injected by the platform).
  dotenv.config();

  const rawConfig = {
    Port: process.env.PORT,
    LogLevel: process.env.LOG_LEVEL,
    Env: process.env.APP_ENV,
    DBHost: process.env.DB_HOST,
    DBPort: process.env.DB_PORT,
    DBUser: process.env.DB_USER,
    DBPassword: process.env.DB_PASSWORD,
    DBName: process.env.DB_NAME,
    DBPoolSize: process.env.DB_POOL_SIZE,
    StripeAPIKey: process.env.STRIPE_API_KEY,
    NewCheckoutEnabled: process.env.NEW_CHECKOUT === "true",
  };

  // THE critical step, validate everything before the app boots
  const parsed = ConfigSchema.safeParse(rawConfig);
  if (!parsed.success) {
    throw new Error(`config validation failed: ${parsed.error.message}`);
  }

  return parsed.data;
}

// Usage in index.ts:
//   try {
//     const cfg = loadConfig();
//   } catch (err) {
//     console.error(err); // crash early, crash loud
//     process.exit(1);
//   }
