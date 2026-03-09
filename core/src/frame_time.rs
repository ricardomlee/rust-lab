#[derive(Debug, Clone, PartialEq)]
pub struct FrameTimeFormat {
    pub fps: f64,
    pub frame_ms: f64,
    pub label: String,
    pub quality_hint: String,
}

pub fn format_frame_time(fps: f64) -> Result<FrameTimeFormat, String> {
    if !fps.is_finite() {
        return Err("fps must be a finite number".into());
    }

    if fps <= 0.0 {
        return Err("fps must be greater than 0".into());
    }

    let frame_ms = 1000.0 / fps;

    Ok(FrameTimeFormat {
        fps,
        frame_ms: round_to(frame_ms, 3),
        label: frame_label(fps, frame_ms),
        quality_hint: quality_hint(fps),
    })
}

fn frame_label(fps: f64, frame_ms: f64) -> String {
    format!("{} FPS (~{} ms/frame)", trim_float(fps), trim_float(frame_ms))
}

fn quality_hint(fps: f64) -> String {
    if fps >= 120.0 {
        "ultra smooth".into()
    } else if fps >= 60.0 {
        "smooth".into()
    } else if fps >= 30.0 {
        "playable".into()
    } else if fps >= 24.0 {
        "cinematic".into()
    } else {
        "choppy".into()
    }
}

fn round_to(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

fn trim_float(value: f64) -> String {
    let rounded = round_to(value, 3);
    let mut rendered = format!("{rounded:.3}");
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
    use super::format_frame_time;

    #[test]
    fn formats_common_refresh_rates() {
        let result = format_frame_time(60.0).unwrap();
        assert_eq!(result.frame_ms, 16.667);
        assert_eq!(result.label, "60 FPS (~16.667 ms/frame)");
        assert_eq!(result.quality_hint, "smooth");
    }

    #[test]
    fn formats_fractional_fps() {
        let result = format_frame_time(23.976).unwrap();
        assert_eq!(result.frame_ms, 41.708);
        assert_eq!(result.label, "23.976 FPS (~41.708 ms/frame)");
        assert_eq!(result.quality_hint, "choppy");
    }

    #[test]
    fn identifies_ultra_smooth_range() {
        let result = format_frame_time(144.0).unwrap();
        assert_eq!(result.frame_ms, 6.944);
        assert_eq!(result.quality_hint, "ultra smooth");
    }

    #[test]
    fn rejects_non_positive_or_invalid_values() {
        assert_eq!(format_frame_time(0.0).unwrap_err(), "fps must be greater than 0");
        assert_eq!(format_frame_time(-1.0).unwrap_err(), "fps must be greater than 0");
        assert_eq!(format_frame_time(f64::NAN).unwrap_err(), "fps must be a finite number");
    }
}
