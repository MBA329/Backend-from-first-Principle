fn apply_discount(cart_total: u32, percent: u32) -> u32 {
    cart_total - (cart_total * percent / 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_discount_table() {
        let cases = vec![
            ("ten percent off 200", 200, 10, 180),
            ("zero discount", 100, 0, 100),
            ("full discount", 100, 100, 0),
            ("rounds down", 99, 10, 89),
        ];

        for (name, subtotal, percent, want) in cases {
            assert_eq!(apply_discount(subtotal, percent), want, "{}", name);
        }
    }
}
