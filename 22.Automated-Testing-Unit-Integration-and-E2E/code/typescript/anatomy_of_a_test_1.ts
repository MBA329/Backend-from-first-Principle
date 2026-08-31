// In jest or vitest
function applyDiscount(cartTotal: number, percent: number): number {
    return cartTotal - (cartTotal * percent / 100);
}

describe('applyDiscount', () => {
    it('applies the discount correctly', () => {
        // Arrange
        const total = 200;
        const percent = 10;
        // Act
        const result = applyDiscount(total, percent);
        // Assert
        expect(result).toBe(180);
    });
});
