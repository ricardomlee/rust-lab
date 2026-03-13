#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountdownFormat {
    pub total_seconds: i64,
    pub sign: String,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub clock: String,
    pub compact: String,
    pub status: String,
}

pub fn format_countdown(total_seconds: i64) -> CountdownFormat {
    let sign = if total_seconds < 0 { "-" } else { "" }.to_string();
    let abs = total_seconds.saturating_abs();

    let days = abs / 86_400;
    let hours = (abs % 86_400) / 3_600;
    let minutes = (abs % 3_600) / 60;
    let seconds = abs % 60;

    CountdownFormat {
        total_seconds,
        sign: sign.clone(),
        days,
        hours,
        minutes,
        seconds,
        clock: format_clock(&sign, days, hours, minutes, seconds),
        compact: format_compact(&sign, days, hours, minutes, seconds),
        status: classify_status(total_seconds, days),
    }
}

fn format_clock(sign: &str, days: i64, hours: i64, minutes: i64, seconds: i64) -> String {
    if days > 0 {
        format!("{}{}d {:02}:{:02}:{:02}", sign, days, hours, minutes, seconds)
    } else {
        format!("{}{:02}:{:02}:{:02}", sign, hours, minutes, seconds)
    }
}

fn format_compact(sign: &str, days: i64, hours: i64, minutes: i64, seconds: i64) -> String {
    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{}s", seconds));
    }

    format!("{}{}", sign, parts.join(" "))
}

fn classify_status(total_seconds: i64, days: i64) -> String {
    if total_seconds < 0 {
        "expired".into()
    } else if total_seconds == 0 {
        "due now".into()
    } else if total_seconds < 60 {
        "imminent".into()
    } else if total_seconds < 3_600 {
        "soon".into()
    } else if days >= 7 {
        "far ahead".into()
    } else {
        "scheduled".into()
    }
}

#[cfg(test)]
mod tests {
    use super::format_countdown;

    #[test]
    fn formats_zero_as_due_now() {
        let result = format_countdown(0);
        assert_eq!(result.clock, "00:00:00");
        assert_eq!(result.compact, "0s");
        assert_eq!(result.status, "due now");
    }

    #[test]
    fn formats_positive_countdown() {
        let result = format_countdown(3_726);
        assert_eq!(result.days, 0);
        assert_eq!(result.hours, 1);
        assert_eq!(result.minutes, 2);
        assert_eq!(result.seconds, 6);
        assert_eq!(result.clock, "01:02:06");
        assert_eq!(result.compact, "1h 2m 6s");
        assert_eq!(result.status, "scheduled");
    }

    #[test]
    fn formats_negative_countdown() {
        let result = format_countdown(-45);
        assert_eq!(result.sign, "-");
        assert_eq!(result.clock, "-00:00:45");
        assert_eq!(result.compact, "-45s");
        assert_eq!(result.status, "expired");
    }

    #[test]
    fn formats_multi_day_countdown() {
        let result = format_countdown(900_610);
        assert_eq!(result.days, 10);
        assert_eq!(result.clock, "10d 10:10:10");
        assert_eq!(result.compact, "10d 10h 10m 10s");
        assert_eq!(result.status, "far ahead");
    }

    #[test]
    fn marks_short_windows() {
        assert_eq!(format_countdown(12).status, "imminent");
        assert_eq!(format_countdown(600).status, "soon");
    }
}
