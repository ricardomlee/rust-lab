pub fn convert(value: &str, from: u32, to: u32) -> Result<String, String> {
    if !(2..=36).contains(&from) || !(2..=36).contains(&to) {
        return Err("base must be between 2 and 36".into());
    }

    let n = i128::from_str_radix(value, from).map_err(|e| e.to_string())?;

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
