/// Time utility functions for benchmarking and formatting
use std::time::{Duration, Instant};

/// Format a duration in a human-readable way (for benchmarking/timing)
pub fn format_elapsed(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if total_secs >= 3600 {
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        format!("{}h {}m {}s", hours, mins, secs)
    } else if total_secs >= 60 {
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}m {}s", mins, secs)
    } else if total_secs > 0 {
        format!("{}.{:03}s", total_secs, millis)
    } else if millis > 0 {
        format!("{}ms", millis)
    } else {
        format!("{}μs", duration.as_micros())
    }
}

/// Simple benchmark helper - measures time to run a closure N times
pub fn benchmark<F, R>(iterations: usize, mut f: F) -> BenchmarkResult
where
    F: FnMut() -> R,
{
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f());
    }
    let elapsed = start.elapsed();
    
    BenchmarkResult {
        iterations,
        total_duration: elapsed,
        avg_per_iteration: elapsed.div_f64(iterations as f64),
    }
}

/// Result of a benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub iterations: usize,
    pub total_duration: Duration,
    pub avg_per_iteration: Duration,
}

impl BenchmarkResult {
    pub fn report(&self) -> String {
        format!(
            "Benchmark: {} iterations\n  Total: {}\n  Avg: {}\n  Iter/s: {:.2}",
            self.iterations,
            format_elapsed(self.total_duration),
            format_elapsed(self.avg_per_iteration),
            self.iterations as f64 / self.total_duration.as_secs_f64()
        )
    }
}

/// Calculate frames per second from frame time in milliseconds
pub fn fps_from_frame_time_ms(frame_time_ms: f64) -> f64 {
    if frame_time_ms <= 0.0 {
        return 0.0;
    }
    1000.0 / frame_time_ms
}

/// Calculate frame time in milliseconds from FPS
pub fn frame_time_from_fps(fps: f64) -> f64 {
    if fps <= 0.0 {
        return 0.0;
    }
    1000.0 / fps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_sub_second_elapsed() {
        let d = Duration::from_millis(250);
        assert_eq!(format_elapsed(d), "250ms");
    }

    #[test]
    fn formats_seconds_elapsed() {
        let d = Duration::from_secs(5);
        assert_eq!(format_elapsed(d), "5.000s");
    }

    #[test]
    fn formats_minutes_elapsed() {
        let d = Duration::from_secs(125); // 2m 5s
        assert_eq!(format_elapsed(d), "2m 5s");
    }

    #[test]
    fn formats_hours_elapsed() {
        let d = Duration::from_secs(3725); // 1h 2m 5s
        assert_eq!(format_elapsed(d), "1h 2m 5s");
    }

    #[test]
    fn benchmark_runs_correct_iterations() {
        let mut counter = 0;
        let result = benchmark(100, || {
            counter += 1;
        });
        assert_eq!(counter, 100);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn fps_from_frame_time_calculates_correctly() {
        assert!((fps_from_frame_time_ms(16.667) - 60.0).abs() < 0.1);
        assert!((fps_from_frame_time_ms(33.333) - 30.0).abs() < 0.1);
        assert_eq!(fps_from_frame_time_ms(0.0), 0.0);
    }

    #[test]
    fn frame_time_from_fps_calculates_correctly() {
        assert!((frame_time_from_fps(60.0) - 16.667).abs() < 0.1);
        assert!((frame_time_from_fps(30.0) - 33.333).abs() < 0.1);
        assert_eq!(frame_time_from_fps(0.0), 0.0);
    }
}
