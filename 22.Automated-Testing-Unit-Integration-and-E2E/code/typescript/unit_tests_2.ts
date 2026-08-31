export function isStrongPassword(p: string): boolean {
    if (p.length < 8) return false;
    let hasDigit = false;
    let hasUpper = false;
    for (const char of p) {
        if (char >= '0' && char <= '9') hasDigit = true;
        if (char >= 'A' && char <= 'Z') hasUpper = true;
    }
    return hasDigit && hasUpper;
}

describe('isStrongPassword', () => {
    it('rejects short passwords', () => {
        expect(isStrongPassword('short1A')).toBe(false);
    });
    it('accepts valid passwords', () => {
        expect(isStrongPassword('longEnough9')).toBe(true);
    });
});
