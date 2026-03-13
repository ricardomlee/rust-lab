#[derive(Debug, Clone, PartialEq)]
pub struct PercentFormat {
    pub value: f64,
    pub percent: String,
    pub ratio: String,
    pub hint: String,
}

pub fn format_percent(value: f64) -> Result<PercentFormat, String> {
    if !value.is_finite() {
        return Err("value must be a finite number".into());
    }

    let clamped = value.clamp(0.0, 1.0);
    let percent_value = round_to(clamped * 100.0, 2);

    Ok(PercentFormat {
        value,
        percent: format_percent_string(percent_value),
        ratio: format_ratio(clamped),
        hint: health_hint(clamped),
    })
}

fn format_percent_string(value: f64) -> String {
    format!("{}%", trim_float(value))
}

fn format_ratio(value: f64) -> String {
    format!("{}/100", trim_float(round_to(value * 100.0, 1)))
}

fn health_hint(value: f64) -> String {
    if value >= 0.95 {
        "nearly full".into()
    } else if value >= 0.8 {
        "healthy".into()
    } else if value >= 0.5 {
        "moderate".into()
    } else if value >= 0.2 {
        "low".into()
    } else {
        "critical".into()
    }
}

fn round_to(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

fn trim_float(value: f64) -> String {
    let mut rendered = format!("{value:.2}");
    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::format_percent;

    #[test]
    fn formats_common_values() {
        let result = format_percent(0.875).unwrap();
        assert_eq!(result.percent, "87.5%");
        assert_eq!(result.ratio, "87.5/100");
        assert_eq!(result.hint, "healthy");
    }

    #[test]
    fn formats_small_values() {
        let result = format_percent(0.0325).unwrap();
        assert_eq!(result.percent, "3.25%");
        assert_eq!(result.ratio, "3.3/100");
        assert_eq!(result.hint, "critical");
    }

    #[test]
    fn clamps_values_outside_zero_to_one() {
        let low = format_percent(-0.2).unwrap();
        assert_eq!(low.percent, "0%");
        assert_eq!(low.hint, "critical");

        let high = format_percent(1.42).unwrap();
        assert_eq!(high.percent, "100%");
        assert_eq!(high.hint, "nearly full");
    }

    #[test]
    fn rejects_non_finite_values() {
        assert_eq!(
            format_percent(f64::NAN).unwrap_err(),
            "value must be a finite number"
        );
    }
}
