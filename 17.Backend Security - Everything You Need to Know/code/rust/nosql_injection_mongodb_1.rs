// X NEVER, string concatenation
let query = format!("SELECT * FROM users WHERE email = '{}'", user_input);

// OK ALWAYS, parameterised ($1 is the slot)
let row = sqlx::query!(
    "SELECT id, name FROM users WHERE email = $1",
    user_input // passed separately, treated purely as data
)
.fetch_one(&pool)
.await?;

// With an ORM (SeaORM), parameterised automatically
let user = User::find()
    .filter(user::Column::Email.eq(user_input))
    .one(&db)
    .await?;
