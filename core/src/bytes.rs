#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteSizeFormat {
    pub bytes: u64,
    pub binary: String,
    pub decimal: String,
    pub binary_per_second: String,
    pub decimal_per_second: String,
}

pub fn format_byte_size(bytes: u64) -> ByteSizeFormat {
    ByteSizeFormat {
        bytes,
        binary: format_with_units(
            bytes,
            1024,
            &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"],
            None,
        ),
        decimal: format_with_units(
            bytes,
            1000,
            &["B", "KB", "MB", "GB", "TB", "PB", "EB"],
            None,
        ),
        binary_per_second: format_with_units(
            bytes,
            1024,
            &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"],
            Some("/s"),
        ),
        decimal_per_second: format_with_units(
            bytes,
            1000,
            &["B", "KB", "MB", "GB", "TB", "PB", "EB"],
            Some("/s"),
        ),
    }
}

fn format_with_units(bytes: u64, base: u64, units: &[&str], suffix: Option<&str>) -> String {
    let suffix = suffix.unwrap_or("");

    if bytes < base {
        return format!("{} {}{}", bytes, units[0], suffix);
    }

    let mut value = bytes as f64;
    let mut unit_index = 0usize;

    while value >= base as f64 && unit_index + 1 < units.len() {
        value /= base as f64;
        unit_index += 1;
    }

    let precision = if value >= 10.0 { 1 } else { 2 };
    let mut rendered = format!("{:.*}", precision, value);
    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }

    format!("{} {}{}", rendered, units[unit_index], suffix)
}

#[cfg(test)]
mod tests {
    use super::format_byte_size;

    #[test]
    fn formats_small_values() {
        let result = format_byte_size(999);
        assert_eq!(result.binary, "999 B");
        assert_eq!(result.decimal, "999 B");
        assert_eq!(result.binary_per_second, "999 B/s");
        assert_eq!(result.decimal_per_second, "999 B/s");
    }

    #[test]
    fn formats_kib_and_kb() {
        let result = format_byte_size(1536);
        assert_eq!(result.binary, "1.5 KiB");
        assert_eq!(result.decimal, "1.54 KB");
        assert_eq!(result.binary_per_second, "1.5 KiB/s");
        assert_eq!(result.decimal_per_second, "1.54 KB/s");
    }

    #[test]
    fn formats_larger_values() {
        let result = format_byte_size(5_368_709_120);
        assert_eq!(result.binary, "5 GiB");
        assert_eq!(result.decimal, "5.37 GB");
        assert_eq!(result.binary_per_second, "5 GiB/s");
        assert_eq!(result.decimal_per_second, "5.37 GB/s");
    }
}
