#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteSizeFormat {
    pub bytes: u64,
    pub binary: String,
    pub decimal: String,
}

pub fn format_byte_size(bytes: u64) -> ByteSizeFormat {
    ByteSizeFormat {
        bytes,
        binary: format_with_units(
            bytes,
            1024,
            &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"],
        ),
        decimal: format_with_units(bytes, 1000, &["B", "KB", "MB", "GB", "TB", "PB", "EB"]),
    }
}

fn format_with_units(bytes: u64, base: u64, units: &[&str]) -> String {
    if bytes < base {
        return format!("{} {}", bytes, units[0]);
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

    format!("{} {}", rendered, units[unit_index])
}

#[cfg(test)]
mod tests {
    use super::format_byte_size;

    #[test]
    fn formats_small_values() {
        let result = format_byte_size(999);
        assert_eq!(result.binary, "999 B");
        assert_eq!(result.decimal, "999 B");
    }

    #[test]
    fn formats_kib_and_kb() {
        let result = format_byte_size(1536);
        assert_eq!(result.binary, "1.5 KiB");
        assert_eq!(result.decimal, "1.54 KB");
    }

    #[test]
    fn formats_larger_values() {
        let result = format_byte_size(5_368_709_120);
        assert_eq!(result.binary, "5 GiB");
        assert_eq!(result.decimal, "5.37 GB");
    }
}
