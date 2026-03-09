#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationFormat {
    pub milliseconds: u64,
    pub compact: String,
    pub clock: String,
    pub verbose_zh: String,
}

pub fn format_duration(milliseconds: u64) -> DurationFormat {
    let total_seconds = milliseconds / 1000;
    let millis_remainder = milliseconds % 1000;

    let days = total_seconds / 86_400;
    let hours = (total_seconds % 86_400) / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    DurationFormat {
        milliseconds,
        compact: format_compact(days, hours, minutes, seconds, millis_remainder),
        clock: format_clock(days, hours, minutes, seconds),
        verbose_zh: format_verbose_zh(days, hours, minutes, seconds, millis_remainder),
    }
}

fn format_compact(days: u64, hours: u64, minutes: u64, seconds: u64, millis: u64) -> String {
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
    if seconds > 0 {
        parts.push(format!("{}s", seconds));
    }
    if millis > 0 || parts.is_empty() {
        parts.push(format!("{}ms", millis));
    }

    parts.join(" ")
}

fn format_clock(days: u64, hours: u64, minutes: u64, seconds: u64) -> String {
    if days > 0 {
        format!("{}d {:02}:{:02}:{:02}", days, hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

fn format_verbose_zh(days: u64, hours: u64, minutes: u64, seconds: u64, millis: u64) -> String {
    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{}天", days));
    }
    if hours > 0 {
        parts.push(format!("{}小时", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}分钟", minutes));
    }
    if seconds > 0 {
        parts.push(format!("{}秒", seconds));
    }
    if millis > 0 || parts.is_empty() {
        parts.push(format!("{}毫秒", millis));
    }

    parts.join("")
}

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn formats_zero_duration() {
        let result = format_duration(0);
        assert_eq!(result.compact, "0ms");
        assert_eq!(result.clock, "00:00:00");
        assert_eq!(result.verbose_zh, "0毫秒");
    }

    #[test]
    fn formats_sub_second_duration() {
        let result = format_duration(250);
        assert_eq!(result.compact, "250ms");
        assert_eq!(result.clock, "00:00:00");
        assert_eq!(result.verbose_zh, "250毫秒");
    }

    #[test]
    fn formats_mixed_duration() {
        let result = format_duration(3_726_045);
        assert_eq!(result.compact, "1h 2m 6s 45ms");
        assert_eq!(result.clock, "01:02:06");
        assert_eq!(result.verbose_zh, "1小时2分钟6秒45毫秒");
    }

    #[test]
    fn formats_multi_day_duration() {
        let result = format_duration(183_845_000);
        assert_eq!(result.compact, "2d 3h 4m 5s");
        assert_eq!(result.clock, "2d 03:04:05");
        assert_eq!(result.verbose_zh, "2天3小时4分钟5秒");
    }
}
