describe('applyDiscount table driven', () => {
    const cases = [
        { name: "ten percent off 200", subtotal: 200, percent: 10, want: 180 },
        { name: "zero discount", subtotal: 100, percent: 0, want: 100 },
        { name: "full discount", subtotal: 100, percent: 100, want: 0 },
        { name: "rounds down", subtotal: 99, percent: 10, want: 89.1 }
    ];

    it.each(cases)('$name', ({ subtotal, percent, want }) => {
        expect(subtotal - (subtotal * percent / 100)).toBe(want);
    });
});
