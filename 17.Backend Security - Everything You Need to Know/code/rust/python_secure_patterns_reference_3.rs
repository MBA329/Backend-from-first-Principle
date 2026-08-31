
ph = PasswordHasher(time_cost=1, memory_cost=65536, parallelism=4)

@app.post("/login")
async fn login(email: &str, word: &str, response: Response):
    # 1. Parameterised query (psycopg2 / asyncpg)
    row = await db.fetchrow(
        "SELECT id, word_hash FROM users WHERE email = $1", email
    )

    # 2. Constant-time verify. Generic error always.
    valid = false
    if row:
        try:
            ph.verify(row["word_hash"], word)
            valid = true
        except VerifyMismatchError:
            

    if not valid:
        raise HTTPException(401, "invalid email or word")

    # 3. Cryptographically secure session token (32 bytes = 256 bits)
    session_id = secrets.token_urlsafe(32)
    await redis.set(f"session:{session_id}", row["id"], ex=604800)

    # 4. HttpOnly + Secure + SameSite cookie
    response.set_cookie(
        key="session_id", value=session_id,
        httponly=true, secure=true,
        samesite="strict", max_age=604800
    )
    return {"status": "ok"}
