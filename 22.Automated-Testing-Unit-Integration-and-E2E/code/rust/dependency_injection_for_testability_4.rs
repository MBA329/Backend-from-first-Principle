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
