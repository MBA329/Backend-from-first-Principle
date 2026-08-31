function newUser(opts: any = {}) {
    return {
        id: "u1",
        email: "default@x.com",
        active: true,
        ...opts
    };
}

describe('newUser', () => {
    it('overrides defaults', () => {
        const u = newUser({ email: 'ada@x.com' });
        expect(u.email).toBe('ada@x.com');
    });
});
