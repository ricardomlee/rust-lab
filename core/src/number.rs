/// Number formatting utilities
/// 
/// Provides formatting for:
/// - Numbers with thousands separators
/// - Currency formatting
/// - Scientific notation
/// - Ordinal numbers (1st, 2nd, 3rd...)

/// Format a number with thousands separators
/// 
/// ```ignore
/// use core::number::format_with_commas;
/// assert_eq!(format_with_commas(1000), "1,000");
/// assert_eq!(format_with_commas(1000000), "1,000,000");
/// ```
pub fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Format a number with thousands separators (signed version)
pub fn format_with_commas_signed(n: i64) -> String {
    if n < 0 {
        format!("-{}", format_with_commas((-n) as u64))
    } else {
        format_with_commas(n as u64)
    }
}

/// Format a number as currency (USD style)
/// 
/// ```ignore
/// use core::number::format_currency;
/// assert_eq!(format_currency(10000), "$100.00");
/// assert_eq!(format_currency(99), "$0.99");
/// ```
pub fn format_currency(cents: u64) -> String {
    let dollars = cents / 100;
    let remaining_cents = cents % 100;
    format!("${}.{:02}", format_with_commas(dollars), remaining_cents)
}

/// Format a number in scientific notation
/// 
/// ```ignore
/// use core::number::format_scientific;
/// let s = format_scientific(1234567.0, 2);
/// assert!(s.contains('e'));
/// ```
pub fn format_scientific(n: f64, precision: usize) -> String {
    format!("{:.precision$e}", n, precision = precision)
}

/// Get the ordinal suffix for a number (1st, 2nd, 3rd, 4th...)
/// 
/// ```ignore
/// use core::number::ordinal_suffix;
/// assert_eq!(ordinal_suffix(1), "st");
/// assert_eq!(ordinal_suffix(2), "nd");
/// assert_eq!(ordinal_suffix(3), "rd");
/// assert_eq!(ordinal_suffix(4), "th");
/// assert_eq!(ordinal_suffix(11), "th");
/// assert_eq!(ordinal_suffix(21), "st");
/// ```
pub fn ordinal_suffix(n: u64) -> &'static str {
    let last_digit = n % 10;
    let last_two_digits = n % 100;
    
    // Special case for 11, 12, 13
    if last_two_digits >= 11 && last_two_digits <= 13 {
        return "th";
    }
    
    match last_digit {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

/// Format a number as an ordinal (1st, 2nd, 3rd...)
/// 
/// ```ignore
/// use core::number::format_ordinal;
/// assert_eq!(format_ordinal(1), "1st");
/// assert_eq!(format_ordinal(2), "2nd");
/// assert_eq!(format_ordinal(3), "3rd");
/// assert_eq!(format_ordinal(4), "4th");
/// assert_eq!(format_ordinal(21), "21st");
/// assert_eq!(format_ordinal(100), "100th");
/// ```
pub fn format_ordinal(n: u64) -> String {
    format!("{}{}", n, ordinal_suffix(n))
}

/// Format a number with a specified number of decimal places
/// 
/// ```ignore
/// use core::number::format_decimal;
/// assert_eq!(format_decimal(3.14159, 2), "3.14");
/// assert_eq!(format_decimal(1000.0, 0), "1,000");
/// ```
pub fn format_decimal(n: f64, decimals: usize) -> String {
    if decimals == 0 {
        format_with_commas(n.round() as u64)
    } else {
        let formatted = format!("{:.decimals$}", n, decimals = decimals);
        let parts: Vec<&str> = formatted.split('.').collect();
        if parts.len() == 2 {
            let int_part: u64 = parts[0].parse().unwrap_or(0);
            format!("{}.{}", format_with_commas(int_part), parts[1])
        } else {
            formatted
        }
    }
}

/// Compact number format for large numbers (1K, 1M, 1B, 1T)
/// 
/// ```ignore
/// use core::number::format_compact;
/// assert_eq!(format_compact(1000), "1.0K");
/// assert_eq!(format_compact(1500000), "1.5M");
/// assert_eq!(format_compact(2500000000), "2.5B");
/// ```
pub fn format_compact(n: u64) -> String {
    const THOUSAND: u64 = 1_000;
    const MILLION: u64 = 1_000_000;
    const BILLION: u64 = 1_000_000_000;
    const TRILLION: u64 = 1_000_000_000_000;
    
    match n {
        n if n >= TRILLION => {
            let val = n as f64 / TRILLION as f64;
            format!("{:.1}T", val)
        }
        n if n >= BILLION => {
            let val = n as f64 / BILLION as f64;
            format!("{:.1}B", val)
        }
        n if n >= MILLION => {
            let val = n as f64 / MILLION as f64;
            format!("{:.1}M", val)
        }
        n if n >= THOUSAND => {
            let val = n as f64 / THOUSAND as f64;
            format!("{:.1}K", val)
        }
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_with_commas() {
        assert_eq!(format_with_commas(0), "0");
        assert_eq!(format_with_commas(999), "999");
        assert_eq!(format_with_commas(1000), "1,000");
        assert_eq!(format_with_commas(10000), "10,000");
        assert_eq!(format_with_commas(100000), "100,000");
        assert_eq!(format_with_commas(1000000), "1,000,000");
        assert_eq!(format_with_commas(1234567890), "1,234,567,890");
    }

    #[test]
    fn test_format_with_commas_signed() {
        assert_eq!(format_with_commas_signed(0), "0");
        assert_eq!(format_with_commas_signed(1000), "1,000");
        assert_eq!(format_with_commas_signed(-1000), "-1,000");
        assert_eq!(format_with_commas_signed(-1234567), "-1,234,567");
    }

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(0), "$0.00");
        assert_eq!(format_currency(99), "$0.99");
        assert_eq!(format_currency(100), "$1.00");
        assert_eq!(format_currency(10000), "$100.00");
        assert_eq!(format_currency(123456), "$1,234.56");
    }

    #[test]
    fn test_format_scientific() {
        assert!(format_scientific(1234567.0, 2).contains('e'));
        assert_eq!(format_scientific(1000.0, 2), "1.00e3");
        assert_eq!(format_scientific(0.001, 2), "1.00e-3");
    }

    #[test]
    fn test_ordinal_suffix() {
        assert_eq!(ordinal_suffix(1), "st");
        assert_eq!(ordinal_suffix(2), "nd");
        assert_eq!(ordinal_suffix(3), "rd");
        assert_eq!(ordinal_suffix(4), "th");
        assert_eq!(ordinal_suffix(5), "th");
        assert_eq!(ordinal_suffix(10), "th");
        assert_eq!(ordinal_suffix(11), "th");
        assert_eq!(ordinal_suffix(12), "th");
        assert_eq!(ordinal_suffix(13), "th");
        assert_eq!(ordinal_suffix(21), "st");
        assert_eq!(ordinal_suffix(22), "nd");
        assert_eq!(ordinal_suffix(23), "rd");
        assert_eq!(ordinal_suffix(100), "th");
        assert_eq!(ordinal_suffix(101), "st");
    }

    #[test]
    fn test_format_ordinal() {
        assert_eq!(format_ordinal(1), "1st");
        assert_eq!(format_ordinal(2), "2nd");
        assert_eq!(format_ordinal(3), "3rd");
        assert_eq!(format_ordinal(4), "4th");
        assert_eq!(format_ordinal(11), "11th");
        assert_eq!(format_ordinal(21), "21st");
        assert_eq!(format_ordinal(100), "100th");
    }

    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(3.14159, 2), "3.14");
        assert_eq!(format_decimal(1000.0, 0), "1,000");
        assert_eq!(format_decimal(1234.567, 1), "1,234.6");
        assert_eq!(format_decimal(999999.99, 2), "999,999.99");
    }

    #[test]
    fn test_format_compact() {
        assert_eq!(format_compact(0), "0");
        assert_eq!(format_compact(999), "999");
        assert_eq!(format_compact(1000), "1.0K");
        assert_eq!(format_compact(1500), "1.5K");
        assert_eq!(format_compact(1000000), "1.0M");
        assert_eq!(format_compact(1500000), "1.5M");
        assert_eq!(format_compact(1000000000), "1.0B");
        assert_eq!(format_compact(2500000000), "2.5B");
        assert_eq!(format_compact(1000000000000), "1.0T");
        assert_eq!(format_compact(3500000000000), "3.5T");
    }
}
