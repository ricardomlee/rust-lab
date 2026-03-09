use sha2::{Digest, Sha256};

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

#[cfg(test)]
mod tests {
    use super::analyze;

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
}
