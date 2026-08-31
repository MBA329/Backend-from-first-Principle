// TypeScript, Safe logging practices
// Assuming a configured logger like winston or pino
import pino from 'pino';
const logger = pino();

// X UNSAFE, never do this
/*
logger.error({
    email: user.Email,          // PII leak
    password: req.Password,     // catastrophic
    api_key: cfg.OpenAIKey,     // secret leak
}, "login_failed");
*/

// OK SAFE, IDs and correlation only
/*
logger.error({
    user_id: user.ID,
    correlation_id: req.headers['x-request-id'],
    reason: "invalid_credentials",  // generic code, not DB message
}, "login_failed");
*/
