use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextAnalysis {
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
    pub sha256: String,
}

pub fn analyze(text: &str) -> Result<TextAnalysis, String> {
    if text.trim().is_empty() {
        return Err("text cannot be empty".into());
    }

    let chars = text.chars().count();
    let words = text.split_whitespace().count();
    let lines = text.lines().count();

    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let sha256 = format!("{:x}", hasher.finalize());

    Ok(TextAnalysis {
        chars,
        words,
        lines,
        sha256,
    })
}

/// Convert text to a URL-friendly slug
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Truncate text to max_len characters, adding ellipsis if truncated
pub fn truncate(text: &str, max_len: usize) -> String {
    if max_len == 0 {
        return String::new();
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_len {
        return text.to_string();
    }
    if max_len <= 3 {
        return chars[..max_len].iter().collect();
    }
    chars[..max_len - 3].iter().collect::<String>() + "..."
}

/// Count word frequency in text, returns sorted by count (descending)
pub fn word_frequency(text: &str) -> Vec<(String, usize)> {
    let mut freq: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        let cleaned: String = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        if !cleaned.is_empty() {
            *freq.entry(cleaned).or_insert(0) += 1;
        }
    }
    let mut result: Vec<(String, usize)> = freq.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}

/// Extract first N words from text
pub fn first_n_words(text: &str, n: usize) -> String {
    text.split_whitespace().take(n).collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyzes_multiline_text() {
        let result = analyze("Rust + WASM\nworks well").unwrap();

        assert_eq!(result.chars, 22);
        assert_eq!(result.words, 5);
        assert_eq!(result.lines, 2);
        assert_eq!(
            result.sha256,
            "baa087dd3871b8ab28f2bd79b7168cd8b3ef461e757db449c887cbcfdc085d1c"
        );
    }

    #[test]
    fn rejects_blank_text() {
        assert_eq!(analyze("   \n\t").unwrap_err(), "text cannot be empty");
    }

    #[test]
    fn slugify_converts_to_lowercase_and_dashes() {
        assert_eq!(slugify("Hello World!"), "hello-world");
        assert_eq!(slugify("Rust + WASM = Awesome"), "rust-wasm-awesome");
        assert_eq!(slugify("Café & Restaurant"), "café-restaurant");
    }

    #[test]
    fn slugify_handles_multiple_spaces() {
        assert_eq!(slugify("multiple   spaces"), "multiple-spaces");
        assert_eq!(slugify("  leading and trailing  "), "leading-and-trailing");
    }

    #[test]
    fn truncate_returns_original_if_short_enough() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("exact", 5), "exact");
    }

    #[test]
    fn truncate_adds_ellipsis() {
        assert_eq!(truncate("hello world", 8), "hello...");
        assert_eq!(truncate("Rust is great", 10), "Rust is...");
    }

    #[test]
    fn truncate_handles_edge_cases() {
        assert_eq!(truncate("test", 0), "");
        assert_eq!(truncate("test", 1), "t");
        assert_eq!(truncate("test", 3), "tes");
        assert_eq!(truncate("test", 4), "test");
    }

    #[test]
    fn word_frequency_counts_correctly() {
        let result = word_frequency("hello world hello rust world hello");
        assert_eq!(result[0], ("hello".to_string(), 3));
        assert_eq!(result[1], ("world".to_string(), 2));
        assert_eq!(result[2], ("rust".to_string(), 1));
    }

    #[test]
    fn word_frequency_cleans_punctuation() {
        let result = word_frequency("Hello, hello! HELLO.");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ("hello".to_string(), 3));
    }

    #[test]
    fn first_n_words_extracts_correctly() {
        assert_eq!(first_n_words("one two three four five", 3), "one two three");
        assert_eq!(first_n_words("single", 5), "single");
        assert_eq!(first_n_words("", 3), "");
    }
}
