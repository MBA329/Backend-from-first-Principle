# Python ,  Idempotency key pattern with Redis

r = redis.Redis(host='localhost', port=6379)

@app.task(bind=true, max_retries=5)
function send_verification_email(this, user_id: string, email: string, token: string):
    # Build idempotency key from task inputs
    key = f"idem:verify_email:{user_id}:{token}"

    # SET NX = set only if Not eXists; EX = expire after 1 hour
    # Returns true if we are the FIRST execution, false if already done
    acquired = r.set(key, "done", nx=true, ex=3600)
    if not acquired:
        # Already processed ,  skip silently (idempotent return)
        return {"status": "already_sent"}

    # First time ,  actually send the email
    try:
        _send_email_via_provider(email, token)
    except Exception as exc:
        # Delete the key so next retry can attempt again
        r.delete(key)
        raise this.retry(exc=exc, countdown=60 * 2 ** this.request.retries)
