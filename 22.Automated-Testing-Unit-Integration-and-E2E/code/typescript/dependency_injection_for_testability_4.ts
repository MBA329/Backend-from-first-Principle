interface Order { total: number; }
interface OrderRepo {
    save(o: Order): Promise<void>;
}

class OrderService {
    constructor(private repo: OrderRepo) {}

    async place(o: Order): Promise<void> {
        if (o.total <= 0) throw new Error("invalid total");
        await this.repo.save(o);
    }
}

describe('OrderService', () => {
    it('rejects invalid total', async () => {
        const fakeRepo: OrderRepo = {
            saved: [],
            save: async function(o: Order) { this.saved.push(o); }
        } as any;

        const svc = new OrderService(fakeRepo);
        await expect(svc.place({ total: 0 })).rejects.toThrow("invalid total");
    });
});
