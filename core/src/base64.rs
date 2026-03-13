//! Base64 encoding and decoding utilities.
//!
//! Provides safe wrappers around base64 operations for both standard and URL-safe variants.

use base64::{engine::general_purpose, Engine as _};

/// Result of a base64 encode/decode operation.
#[derive(Debug, Clone, PartialEq)]
pub struct Base64Result {
    /// The encoded or decoded string.
    pub value: String,
    /// The variant used (standard or url_safe).
    pub variant: String,
}

/// Encode bytes to standard Base64.
pub fn encode_standard(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Encode bytes to URL-safe Base64.
pub fn encode_url_safe(data: &[u8]) -> String {
    general_purpose::URL_SAFE.encode(data)
}

/// Decode standard Base64 to bytes.
pub fn decode_standard(encoded: &str) -> Result<Vec<u8>, String> {
    general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| format!("Base64 decode error: {}", e))
}

/// Decode URL-safe Base64 to bytes.
pub fn decode_url_safe(encoded: &str) -> Result<Vec<u8>, String> {
    general_purpose::URL_SAFE
        .decode(encoded)
        .map_err(|e| format!("Base64 decode error: {}", e))
}

/// Encode a UTF-8 string to standard Base64.
pub fn encode_string_standard(input: &str) -> String {
    encode_standard(input.as_bytes())
}

/// Encode a UTF-8 string to URL-safe Base64.
pub fn encode_string_url_safe(input: &str) -> String {
    encode_url_safe(input.as_bytes())
}

/// Decode standard Base64 to a UTF-8 string.
pub fn decode_string_standard(encoded: &str) -> Result<String, String> {
    let bytes = decode_standard(encoded)?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {}", e))
}

/// Decode URL-safe Base64 to a UTF-8 string.
pub fn decode_string_url_safe(encoded: &str) -> Result<String, String> {
    let bytes = decode_url_safe(encoded)?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {}", e))
}

/// Encode/decode result with metadata.
pub struct EncodeDecodeResult {
    /// The result value.
    pub value: String,
    /// Original input (for roundtrip verification).
    pub original: String,
    /// Whether it's encoded or decoded.
    pub operation: String,
    /// Base64 variant used.
    pub variant: String,
}

/// Full encode operation with metadata.
pub fn encode_full(input: &str, url_safe: bool) -> EncodeDecodeResult {
    let (value, variant) = if url_safe {
        (encode_string_url_safe(input), "url_safe".to_string())
    } else {
        (encode_string_standard(input), "standard".to_string())
    };
    EncodeDecodeResult {
        value,
        original: input.to_string(),
        operation: "encode".to_string(),
        variant,
    }
}

/// Full decode operation with metadata.
pub fn decode_full(encoded: &str, url_safe: bool) -> Result<EncodeDecodeResult, String> {
    let (result, variant) = if url_safe {
        (decode_string_url_safe(encoded), "url_safe".to_string())
    } else {
        (decode_string_standard(encoded), "standard".to_string())
    };
    result.map(|value| EncodeDecodeResult {
        value,
        original: encoded.to_string(),
        operation: "decode".to_string(),
        variant,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_standard() {
        let input = "Hello, World!";
        let encoded = encode_string_standard(input);
        let decoded = decode_string_standard(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_encode_decode_url_safe() {
        let input = "Hello?World&Test";
        let encoded = encode_string_url_safe(input);
        let decoded = decode_string_url_safe(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_encode_standard_known_value() {
        // "Man" in base64 is "TWFu"
        let encoded = encode_string_standard("Man");
        assert_eq!(encoded, "TWFu");
    }

    #[test]
    fn test_decode_invalid_base64() {
        let result = decode_string_standard("!!!invalid!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_full_metadata() {
        let result = encode_full("test", false);
        assert_eq!(result.original, "test");
        assert_eq!(result.operation, "encode");
        assert_eq!(result.variant, "standard");
        assert!(!result.value.is_empty());
    }

    #[test]
    fn test_url_safe_differs_from_standard() {
        let input = "test?>";
        let std_encoded = encode_string_standard(input);
        let url_encoded = encode_string_url_safe(input);
        // URL-safe uses - and _ instead of + and /
        assert_ne!(std_encoded, url_encoded);
    }

    #[test]
    fn test_roundtrip_utf8() {
        let test_strings = [
            "Hello, 世界！",
            "Привет мир",
            "🦀 Rust WASM 🚀",
            "Special: \n\t\r",
        ];
        for input in test_strings {
            let encoded = encode_string_standard(input);
            let decoded = decode_string_standard(&encoded).unwrap();
            assert_eq!(input, decoded, "Failed for: {}", input);
        }
    }
}
