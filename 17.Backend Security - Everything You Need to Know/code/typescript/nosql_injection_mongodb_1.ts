// X NEVER, string concatenation
const query = "SELECT * FROM users WHERE email = '" + userInput + "'";

// OK ALWAYS, parameterised ($1 is the slot)
const row = await db.query(
    "SELECT id, name FROM users WHERE email = $1",
    [userInput]  // passed separately, treated purely as data
);

// With an ORM (Prisma), parameterised automatically
const user = await prisma.user.findFirst({ where: { email: userInput } });
