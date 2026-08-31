struct User {
    id: String,
    email: String,
    active: bool,
}

fn new_user(email_override: Option<String>) -> User {
    User {
        id: "u1".into(),
        email: email_override.unwrap_or_else(|| "default@x.com".into()),
        active: true,
    }
}

#[test]
fn test_signup() {
    let u = new_user(Some("ada@x.com".into()));
    assert_eq!(u.email, "ada@x.com");
}
