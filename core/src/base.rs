pub fn convert(value: &str, from: u32, to: u32) -> Result<String, String> {
    if !(2..=36).contains(&from) || !(2..=36).contains(&to) {
        return Err("base must be between 2 and 36".into());
    }

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("value cannot be empty".into());
    }

    let n = i128::from_str_radix(trimmed, from).map_err(|e| e.to_string())?;

    Ok(to_base(n, to))
}

fn to_base(mut n: i128, base: u32) -> String {
    if n == 0 {
        return "0".into();
    }

    let neg = n < 0;
    if neg {
        n = -n;
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push(std::char::from_digit((n % base as i128) as u32, base).unwrap());
        n /= base as i128;
    }

    if neg {
        digits.push('-');
    }

    digits.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn converts_common_positive_values() {
        assert_eq!(convert("ff", 16, 10).unwrap(), "255");
        assert_eq!(convert("101101", 2, 16).unwrap(), "2d");
        assert_eq!(convert("1295", 10, 36).unwrap(), "zz");
    }

    #[test]
    fn converts_negative_values() {
        assert_eq!(convert("-42", 10, 2).unwrap(), "-101010");
        assert_eq!(convert("-1111", 2, 10).unwrap(), "-15");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        assert_eq!(convert("   7f  ", 16, 10).unwrap(), "127");
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(convert("   ", 10, 2).unwrap_err(), "value cannot be empty");
    }

    #[test]
    fn rejects_invalid_bases() {
        assert_eq!(
            convert("10", 1, 2).unwrap_err(),
            "base must be between 2 and 36"
        );
        assert_eq!(
            convert("10", 10, 37).unwrap_err(),
            "base must be between 2 and 36"
        );
    }

    #[test]
    fn rejects_digits_not_in_source_base() {
        let err = convert("2", 2, 10).unwrap_err();
        assert!(err.contains("invalid digit"));
    }
}
