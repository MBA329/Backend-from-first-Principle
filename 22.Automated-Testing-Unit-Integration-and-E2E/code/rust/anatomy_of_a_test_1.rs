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
