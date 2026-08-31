const fs = require('fs');
const files = {
  "anatomy_of_a_test_1.rs": `
fn apply_discount(cart_total: u32, percent: u32) -> u32 {
    cart_total - (cart_total * percent / 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_discount() {
        // Arrange
        let total = 200;
        let percent = 10;
        // Act
        let result = apply_discount(total, percent);
        // Assert
        assert_eq!(result, 180);
    }
}
`,
  "unit_tests_2.rs": `
pub fn is_strong_password(p: &str) -> bool {
    if p.len() < 8 { return false; }
    let mut has_digit = false;
    let mut has_upper = false;
    for c in p.chars() {
        if c.is_digit(10) { has_digit = true; }
        if c.is_uppercase() { has_upper = true; }
    }
    has_digit && has_upper
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strong_password() {
        assert!(!is_strong_password("short1A"));
        assert!(is_strong_password("longEnough9"));
    }
}
`,
  "test_doubles_3.rs": `
use std::sync::Mutex;

struct User { email: String, name: String }

trait UserRepo {
    fn find_by_id(&self, id: &str) -> Result<User, ()>;
}
trait Notifier {
    fn send(&self, to: &str, msg: &str) -> Result<(), ()>;
}

fn greet(repo: &impl UserRepo, notifier: &impl Notifier, id: &str) -> Result<(), ()> {
    let u = repo.find_by_id(id)?;
    notifier.send(&u.email, &format!("Hi {}", u.name))
}

struct StubRepo;
impl UserRepo for StubRepo {
    fn find_by_id(&self, _id: &str) -> Result<User, ()> {
        Ok(User { email: "a@x.com".into(), name: "Ada".into() })
    }
}

struct SpyNotifier {
    calls: Mutex<u32>,
    last_to: Mutex<String>,
}
impl Notifier for SpyNotifier {
    fn send(&self, to: &str, _msg: &str) -> Result<(), ()> {
        *self.calls.lock().unwrap() += 1;
        *self.last_to.lock().unwrap() = to.to_string();
        Ok(())
    }
}

#[test]
fn test_greet() {
    let repo = StubRepo;
    let spy = SpyNotifier { calls: Mutex::new(0), last_to: Mutex::new(String::new()) };
    let _ = greet(&repo, &spy, "u1");
    assert_eq!(*spy.calls.lock().unwrap(), 1);
    assert_eq!(*spy.last_to.lock().unwrap(), "a@x.com");
}
`,
  "dependency_injection_for_testability_4.rs": `
struct Order { total: i32 }
trait OrderRepo { fn save(&mut self, o: Order) -> Result<(), ()>; }

struct OrderService<R: OrderRepo> { repo: R }

impl<R: OrderRepo> OrderService<R> {
    fn place(&mut self, o: Order) -> Result<(), ()> {
        if o.total <= 0 { return Err(()); }
        self.repo.save(o)
    }
}

struct FakeRepo { saved: Vec<Order> }
impl OrderRepo for FakeRepo {
    fn save(&mut self, o: Order) -> Result<(), ()> {
        self.saved.push(o); Ok(())
    }
}

#[test]
fn test_place_rejects_invalid_total() {
    let mut svc = OrderService { repo: FakeRepo { saved: vec![] } };
    assert!(svc.place(Order { total: 0 }).is_err());
}
`,
  "table_driven_parametrized_tests_5.rs": `
fn apply_discount(cart_total: u32, percent: u32) -> u32 {
    cart_total - (cart_total * percent / 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_discount_table() {
        let cases = vec![
            ("ten percent off 200", 200, 10, 180),
            ("zero discount", 100, 0, 100),
            ("full discount", 100, 100, 0),
            ("rounds down", 99, 10, 89),
        ];

        for (name, subtotal, percent, want) in cases {
            assert_eq!(apply_discount(subtotal, percent), want, "{}", name);
        }
    }
}
`,
  "testing_with_a_real_database_6.rs": `
// In rust, tests with database might use sqlx and testcontainers-rs.
// This is a pseudo-implementation to represent the concept.
/*
use sqlx::PgPool;

#[tokio::test]
async fn test_user_repo_save_and_find() {
    let pool = PgPool::connect("postgres://user:pass@localhost/db").await.unwrap();
    // Act
    sqlx::query!("INSERT INTO users (id, email) VALUES ($1, $2)", "u1", "a@x.com")
        .execute(&pool).await.unwrap();
    let row = sqlx::query!("SELECT email FROM users WHERE id = $1", "u1")
        .fetch_one(&pool).await.unwrap();
    // Assert
    assert_eq!(row.email, "a@x.com");
}
*/
`,
  "testing_http_handlers_apis_7.rs": `
use axum::{routing::post, Router, response::IntoResponse, http::StatusCode};
use axum::http::Request;
use hyper::Body;
use tower::ServiceExt;

async fn create_user() -> impl IntoResponse {
    (StatusCode::CREATED, "{\"id\": 1}")
}

#[tokio::test]
async fn test_create_user_returns_201() {
    let app = Router::new().route("/api/v1/users", post(create_user));
    
    let response = app
        .oneshot(Request::builder()
            .method("POST")
            .uri("/api/v1/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email":"a@x.com","name":"Ada"}"#))
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
`,
  "fixtures_factories_8.rs": `
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
`,
  "mocking_time_randomness_external_apis_9.rs": `
use std::time::SystemTime;

trait Clock { fn now(&self) -> SystemTime; }
struct FixedClock { t: SystemTime }
impl Clock for FixedClock {
    fn now(&self) -> SystemTime { self.t }
}

#[test]
fn test_token_expiry() {
    let clk = FixedClock { t: SystemTime::now() };
    assert_eq!(clk.now(), clk.t);
}
`,
  "performance_load_testing_10.rs": `
// In Rust, Criterion is commonly used for performance/benchmark testing.
// #[macro_use]
// extern crate criterion;
// use criterion::{black_box, Criterion};
// 
// fn benchmark_parse(c: &mut Criterion) {
//     c.bench_function("parse event", |b| b.iter(|| black_box(parse_event())));
// }
`
};

for (const [name, content] of Object.entries(files)) {
    fs.writeFileSync('code/rust/' + name, content.trim() + '\n');
}
