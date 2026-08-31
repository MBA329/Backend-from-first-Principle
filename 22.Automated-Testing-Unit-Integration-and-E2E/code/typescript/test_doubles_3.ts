interface UserRepo {
    findById(id: string): Promise<{email: string, name: string}>;
}
interface Notifier {
    send(to: string, msg: string): Promise<void>;
}

async function greet(repo: UserRepo, notifier: Notifier, id: string) {
    const u = await repo.findById(id);
    await notifier.send(u.email, 'Hi ' + u.name);
}

describe('greet', () => {
    it('sends email', async () => {
        const stubRepo: UserRepo = {
            findById: async () => ({ email: 'a@x.com', name: 'Ada' })
        };
        const spyNotifier = {
            calls: 0,
            lastTo: '',
            send: async (to: string) => {
                spyNotifier.calls++;
                spyNotifier.lastTo = to;
            }
        };

        await greet(stubRepo, spyNotifier, 'u1');

        expect(spyNotifier.calls).toBe(1);
        expect(spyNotifier.lastTo).toBe('a@x.com');
    });
});
