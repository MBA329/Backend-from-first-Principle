import pino from "pino";

export function setupLogger() {
    const level = process.env.APP_ENV === "development" ? "debug" : "info";
    // Using pino for structured JSON logging by default
    return pino({ level });
}
