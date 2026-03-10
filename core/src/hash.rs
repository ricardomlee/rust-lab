//! Hash utilities for SHA256 and other cryptographic hashes.
//!
//! Provides simple wrappers for common hashing operations suitable for WASM exposure.

use sha2::{Digest, Sha256, Sha512};

/// Result of a hash operation.
#[derive(Debug, Clone, PartialEq)]
pub struct HashResult {
    /// The input that was hashed.
    pub input: String,
    /// The algorithm used.
    pub algorithm: String,
    /// The hex-encoded hash output.
    pub hash: String,
}

/// Compute SHA256 hash of input bytes.
pub fn sha256_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Compute SHA256 hash of a UTF-8 string, returning hex-encoded result.
pub fn sha256_hex(input: &str) -> String {
    let hash = sha256_bytes(input.as_bytes());
    hex_encode(&hash)
}

/// Compute SHA512 hash of input bytes.
pub fn sha512_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Compute SHA512 hash of a UTF-8 string, returning hex-encoded result.
pub fn sha512_hex(input: &str) -> String {
    let hash = sha512_bytes(input.as_bytes());
    hex_encode(&hash)
}

/// Full hash operation with metadata.
pub fn hash_full(input: &str, algorithm: &str) -> HashResult {
    let (hash, algo) = match algorithm.to_lowercase().as_str() {
        "sha512" | "sha-512" | "512" => (sha512_hex(input), "SHA512"),
        _ => (sha256_hex(input), "SHA256"),
    };
    HashResult {
        input: input.to_string(),
        algorithm: algo.to_string(),
        hash,
    }
}

/// Convert bytes to hex string.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Verify if input produces expected hash.
pub fn verify_hash(input: &str, expected_hash: &str, algorithm: &str) -> bool {
    let result = hash_full(input, algorithm);
    result.hash.to_lowercase() == expected_hash.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_value() {
        // "hello" SHA256 is 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
        let hash = sha256_hex("hello");
        assert_eq!(hash, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test_sha512_known_value() {
        // "hello" SHA512
        let hash = sha512_hex("hello");
        assert_eq!(
            hash,
            "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
        );
    }

    #[test]
    fn test_hash_full_metadata() {
        let result = hash_full("test", "sha256");
        assert_eq!(result.input, "test");
        assert_eq!(result.algorithm, "SHA256");
        assert_eq!(result.hash.len(), 64); // SHA256 hex is 64 chars
    }

    #[test]
    fn test_hash_full_sha512_metadata() {
        let result = hash_full("test", "sha512");
        assert_eq!(result.input, "test");
        assert_eq!(result.algorithm, "SHA512");
        assert_eq!(result.hash.len(), 128); // SHA512 hex is 128 chars
    }

    #[test]
    fn test_verify_hash_true() {
        let hash = sha256_hex("hello");
        assert!(verify_hash("hello", &hash, "sha256"));
    }

    #[test]
    fn test_verify_hash_false() {
        assert!(!verify_hash("hello", "wronghash", "sha256"));
        assert!(!verify_hash("world", &sha256_hex("hello"), "sha256"));
    }

    #[test]
    fn test_empty_string_hash() {
        // Empty string SHA256
        let hash = sha256_hex("");
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_utf8_hash() {
        let hash = sha256_hex("Hello, 世界！🦀");
        assert_eq!(hash.len(), 64);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_different_algorithms_produce_different_hashes() {
        let input = "test";
        let sha256_result = sha256_hex(input);
        let sha512_result = sha512_hex(input);
        assert_ne!(sha256_result, sha512_result);
        assert_eq!(sha256_result.len(), 64);
        assert_eq!(sha512_result.len(), 128);
    }
}
