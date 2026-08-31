pub fn is_strong_password(p: &str) -> bool {
    if p.len() < 8 { return false; }
    let mut has_digit = false;
    let mut has_upper = false;
    for c in p.chars() {
        if c.is_digit(10) { has_digit = true; }
        if c.is_uppercase() { has_upper = true; }
    }
    has_digit && has_upper
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strong_password() {
        assert!(!is_strong_password("short1A"));
        assert!(is_strong_password("longEnough9"));
    }
}
