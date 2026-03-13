//! Security-related utilities: password generation, HTML/URL encoding.

use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use std::fmt;

/// Result of password generation with metadata.
#[derive(Debug, Clone)]
pub struct PasswordResult {
    pub password: String,
    pub length: usize,
    pub has_uppercase: bool,
    pub has_lowercase: bool,
    pub has_digit: bool,
    pub has_special: bool,
    pub entropy_bits: f64,
}

/// Configuration for password generation.
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_digits: bool,
    pub use_special: bool,
    pub exclude_ambiguous: bool, // Exclude 0, O, l, I, 1
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 16,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_special: true,
            exclude_ambiguous: false,
        }
    }
}

/// Characters considered ambiguous (can be confused with each other).
const AMBIGUOUS_CHARS: &[char] = &['0', 'O', 'l', 'I', '1'];

/// Special characters for password generation.
const SPECIAL_CHARS: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}', '|',
    ';', ':', ',', '.', '<', '>', '?', '/', '~', '`',
];

/// Generate a random password with the given configuration.
pub fn generate_password(config: &PasswordConfig) -> Result<PasswordResult, String> {
    if config.length == 0 {
        return Err("Password length must be greater than 0".to_string());
    }

    let mut char_pool = String::new();

    if config.use_lowercase {
        if config.exclude_ambiguous {
            char_pool.push_str("abcdefghjkmnpqrstuvwxyz");
        } else {
            char_pool.push_str("abcdefghijklmnopqrstuvwxyz");
        }
    }

    if config.use_uppercase {
        if config.exclude_ambiguous {
            char_pool.push_str("ABCDEFGHJKLMNPQRSTUVWXYZ");
        } else {
            char_pool.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
    }

    if config.use_digits {
        if config.exclude_ambiguous {
            char_pool.push_str("23456789");
        } else {
            char_pool.push_str("0123456789");
        }
    }

    if config.use_special {
        char_pool.extend(SPECIAL_CHARS.iter().copied());
    }

    if char_pool.is_empty() {
        return Err("At least one character type must be selected".to_string());
    }

    let pool_chars: Vec<char> = char_pool.chars().collect();
    let pool_size = pool_chars.len();

    // Calculate entropy: log2(pool_size^length) = length * log2(pool_size)
    let entropy_bits = config.length as f64 * (pool_size as f64).log2();

    let mut rng = OsRng;
    let password: String = (0..config.length)
        .map(|_| {
            let idx = rng.gen_range(0..pool_size);
            pool_chars[idx]
        })
        .collect();

    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| SPECIAL_CHARS.contains(&c));

    Ok(PasswordResult {
        password,
        length: config.length,
        has_uppercase,
        has_lowercase,
        has_digit,
        has_special,
        entropy_bits,
    })
}

/// Generate a simple alphanumeric token (URL-safe).
pub fn generate_token(length: usize) -> String {
    let mut rng = OsRng;
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

/// Generate a memorable password using word-like patterns.
pub fn generate_memorable_password(word_count: usize) -> String {
    // Simple syllable-based approach for memorable passwords
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut rng = OsRng;
    let mut password = String::new();

    for i in 0..word_count {
        if i > 0 {
            password.push('-');
        }

        // Generate a 2-syllable "word"
        for _ in 0..2 {
            let c = consonants[rng.gen_range(0..consonants.len())];
            password.push(c);
            let v = vowels[rng.gen_range(0..vowels.len())];
            password.push(v);
        }
    }

    // Capitalize first letter of each "word"
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in password.chars() {
        if c == '-' {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// HTML entity encode a string.
pub fn html_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            c => result.push(c),
        }
    }
    result
}

/// HTML entity decode a string.
pub fn html_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '&' {
            let mut entity = String::new();
            while let Some(&next) = chars.peek() {
                if next == ';' {
                    chars.next();
                    break;
                }
                entity.push(chars.next().unwrap());
            }

            match entity.as_str() {
                "amp" => result.push('&'),
                "lt" => result.push('<'),
                "gt" => result.push('>'),
                "quot" => result.push('"'),
                "apos" | "#39" => result.push('\''),
                "nbsp" => result.push(' '),
                s if s.starts_with('#') => {
                    // Numeric entity
                    let code = s[1..].parse::<u32>();
                    if let Ok(code) = code {
                        if let Some(c) = char::from_u32(code) {
                            result.push(c);
                        }
                    }
                }
                _ => {
                    result.push('&');
                    result.push_str(&entity);
                    result.push(';');
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// URL encode a string (percent encoding).
pub fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    for c in input.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ' ' => result.push('+'),
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}

/// URL decode a string (percent decoding).
pub fn url_decode(input: &str) -> Result<String, String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(' '),
            '%' => {
                let hex: String = chars.by_ref().take(2).collect();
                if hex.len() != 2 {
                    return Err("Invalid percent encoding: incomplete hex sequence".to_string());
                }
                match u8::from_str_radix(&hex, 16) {
                    Ok(byte) => result.push(byte as char),
                    Err(_) => return Err(format!("Invalid percent encoding: %{}", hex)),
                }
            }
            c => result.push(c),
        }
    }

    Ok(result)
}

/// Check password strength and return a detailed assessment.
pub fn assess_password_strength(password: &str) -> PasswordStrength {
    let length = password.len();
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| SPECIAL_CHARS.contains(&c));

    let char_types = [has_upper, has_lower, has_digit, has_special]
        .iter()
        .filter(|&&b| b)
        .count();

    // Calculate approximate entropy
    let pool_size = if has_upper { 26 } else { 0 }
        + if has_lower { 26 } else { 0 }
        + if has_digit { 10 } else { 0 }
        + if has_special { SPECIAL_CHARS.len() } else { 0 };

    let entropy_bits = if pool_size > 0 {
        length as f64 * (pool_size as f64).log2()
    } else {
        0.0
    };

    // Determine strength level
    let level = if entropy_bits >= 128.0 && char_types >= 4 && length >= 16 {
        StrengthLevel::VeryStrong
    } else if entropy_bits >= 80.0 && char_types >= 3 && length >= 12 {
        StrengthLevel::Strong
    } else if entropy_bits >= 60.0 && char_types >= 2 && length >= 8 {
        StrengthLevel::Moderate
    } else if length >= 6 {
        StrengthLevel::Weak
    } else {
        StrengthLevel::VeryWeak
    };

    let mut suggestions = Vec::new();

    if length < 12 {
        suggestions.push("Increase password length to at least 12 characters");
    }
    if !has_upper {
        suggestions.push("Add uppercase letters");
    }
    if !has_lower {
        suggestions.push("Add lowercase letters");
    }
    if !has_digit {
        suggestions.push("Add numbers");
    }
    if !has_special {
        suggestions.push("Add special characters (!@#$%^&*)");
    }

    PasswordStrength {
        level,
        entropy_bits,
        char_types,
        length,
        suggestions,
    }
}

/// Password strength level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrengthLevel {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

impl fmt::Display for StrengthLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrengthLevel::VeryWeak => write!(f, "Very Weak"),
            StrengthLevel::Weak => write!(f, "Weak"),
            StrengthLevel::Moderate => write!(f, "Moderate"),
            StrengthLevel::Strong => write!(f, "Strong"),
            StrengthLevel::VeryStrong => write!(f, "Very Strong"),
        }
    }
}

/// Detailed password strength assessment.
#[derive(Debug, Clone)]
pub struct PasswordStrength {
    pub level: StrengthLevel,
    pub entropy_bits: f64,
    pub char_types: usize,
    pub length: usize,
    pub suggestions: Vec<&'static str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_default() {
        let config = PasswordConfig::default();
        let result = generate_password(&config).unwrap();

        assert_eq!(result.length, 16);
        assert!(result.has_uppercase);
        assert!(result.has_lowercase);
        assert!(result.has_digit);
        assert!(result.has_special);
        assert!(result.entropy_bits > 80.0);
    }

    #[test]
    fn test_generate_password_custom_length() {
        let config = PasswordConfig {
            length: 32,
            ..Default::default()
        };
        let result = generate_password(&config).unwrap();
        assert_eq!(result.password.len(), 32);
        assert!(result.entropy_bits > 150.0);
    }

    #[test]
    fn test_generate_password_no_special() {
        let config = PasswordConfig {
            use_special: false,
            ..Default::default()
        };
        let result = generate_password(&config).unwrap();
        assert!(!result.has_special);
        assert!(result.has_uppercase);
        assert!(result.has_lowercase);
        assert!(result.has_digit);
    }

    #[test]
    fn test_generate_password_exclude_ambiguous() {
        let config = PasswordConfig {
            exclude_ambiguous: true,
            use_special: false,
            ..Default::default()
        };
        let result = generate_password(&config).unwrap();

        // Should not contain ambiguous characters
        for c in result.password.chars() {
            assert!(!AMBIGUOUS_CHARS.contains(&c));
        }
    }

    #[test]
    fn test_generate_password_empty_length() {
        let config = PasswordConfig {
            length: 0,
            ..Default::default()
        };
        assert!(generate_password(&config).is_err());
    }

    #[test]
    fn test_generate_password_no_char_types() {
        let config = PasswordConfig {
            use_uppercase: false,
            use_lowercase: false,
            use_digits: false,
            use_special: false,
            ..Default::default()
        };
        assert!(generate_password(&config).is_err());
    }

    #[test]
    fn test_generate_token() {
        let token = generate_token(16);
        assert_eq!(token.len(), 16);
        assert!(token.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_generate_memorable_password() {
        let password = generate_memorable_password(3);
        assert!(password.contains('-'));
        assert!(password.len() > 10);
    }

    #[test]
    fn test_html_encode() {
        assert_eq!(html_encode("<script>"), "&lt;script&gt;");
        assert_eq!(html_encode("a & b"), "a &amp; b");
        assert_eq!(html_encode("\"quotes\""), "&quot;quotes&quot;");
    }

    #[test]
    fn test_html_decode() {
        assert_eq!(html_decode("&lt;script&gt;"), "<script>");
        assert_eq!(html_decode("a &amp; b"), "a & b");
        assert_eq!(html_decode("&quot;quotes&quot;"), "\"quotes\"");
        assert_eq!(html_decode("&#39;"), "'");
    }

    #[test]
    fn test_html_roundtrip() {
        let original = "<div>Hello & World</div>";
        let encoded = html_encode(original);
        let decoded = html_decode(&encoded);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("hello world"), "hello+world");
        assert_eq!(url_encode("a/b"), "a%2Fb");
        assert_eq!(url_encode("test@email.com"), "test%40email.com");
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode("hello+world").unwrap(), "hello world");
        assert_eq!(url_decode("a%2Fb").unwrap(), "a/b");
        assert_eq!(url_decode("test%40email.com").unwrap(), "test@email.com");
    }

    #[test]
    fn test_url_roundtrip() {
        let original = "hello world & test@email.com";
        let encoded = url_encode(original);
        let decoded = url_decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_assess_password_strength_very_weak() {
        let strength = assess_password_strength("abc");
        assert_eq!(strength.level, StrengthLevel::VeryWeak);
        assert!(!strength.suggestions.is_empty());
    }

    #[test]
    fn test_assess_password_strength_strong() {
        let strength = assess_password_strength("Str0ng!Pass@123");
        assert!(strength.level == StrengthLevel::Strong || strength.level == StrengthLevel::VeryStrong);
    }

    #[test]
    fn test_assess_password_strength_suggestions() {
        let strength = assess_password_strength("abc");
        assert!(strength.suggestions.len() >= 3);
    }
}
