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
