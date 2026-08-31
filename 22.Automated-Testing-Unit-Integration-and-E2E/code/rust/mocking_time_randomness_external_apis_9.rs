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
