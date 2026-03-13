/// UUID generation utilities
/// Works in both server-side Rust and WASM contexts
/// Uses a simple PRNG seeded by time for portability

/// Generate a random UUID v4
pub fn generate_uuid() -> String {
    let mut bytes = [0u8; 16];
    fill_random_bytes(&mut bytes);
    
    // Set version (4) and variant bits for UUID v4
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u16::from_be_bytes([bytes[4], bytes[5]]),
        u16::from_be_bytes([bytes[6], bytes[7]]),
        u16::from_be_bytes([bytes[8], bytes[9]]),
        u64::from_be_bytes([0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]])
    )
}

/// Fill a byte slice with pseudo-random bytes
fn fill_random_bytes(bytes: &mut [u8]) {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u64;
    
    let mut s = seed;
    for byte in bytes.iter_mut() {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
        *byte = (s >> 33) as u8;
    }
}

/// Validate if a string is a valid UUID format
pub fn is_valid_uuid(uuid: &str) -> bool {
    if uuid.len() != 36 {
        return false;
    }
    
    // Check format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    if uuid.chars().nth(8) != Some('-')
        || uuid.chars().nth(13) != Some('-')
        || uuid.chars().nth(18) != Some('-')
        || uuid.chars().nth(23) != Some('-')
    {
        return false;
    }
    
    // Check all other chars are hex
    uuid.chars()
        .enumerate()
        .filter(|(i, _)| !matches!(i, 8 | 13 | 18 | 23))
        .all(|(_, c)| c.is_ascii_hexdigit())
}

/// Generate a short unique ID (8 chars, URL-safe)
pub fn generate_short_id() -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    
    let mut bytes = [0u8; 8];
    fill_random_bytes(&mut bytes);
    
    bytes.iter().map(|&b| CHARS[(b % 64) as usize] as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_format() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36, "UUID should be 36 characters");
        assert_eq!(uuid.chars().nth(8), Some('-'), "UUID should have dash at position 8");
        assert_eq!(uuid.chars().nth(13), Some('-'), "UUID should have dash at position 13");
        assert_eq!(uuid.chars().nth(18), Some('-'), "UUID should have dash at position 18");
        assert_eq!(uuid.chars().nth(23), Some('-'), "UUID should have dash at position 23");
    }

    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = generate_uuid();
        let uuid2 = generate_uuid();
        assert_ne!(uuid1, uuid2, "Generated UUIDs should be unique");
    }

    #[test]
    fn test_uuid_version() {
        let uuid = generate_uuid();
        let version_char = uuid.chars().nth(14);
        assert_eq!(version_char, Some('4'), "UUID should be version 4");
    }

    #[test]
    fn test_uuid_variant() {
        let uuid = generate_uuid();
        let variant_char = uuid.chars().nth(19);
        assert!(matches!(variant_char, Some('8' | '9' | 'a' | 'b' | 'A' | 'B')), 
                "UUID should have correct variant bits");
    }

    #[test]
    fn test_valid_uuid() {
        assert!(is_valid_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(is_valid_uuid("123e4567-e89b-12d3-a456-426614174000"));
    }

    #[test]
    fn test_invalid_uuid() {
        assert!(!is_valid_uuid("not-a-uuid"));
        assert!(!is_valid_uuid("550e8400e29b41d4a716446655440000")); // no dashes
        assert!(!is_valid_uuid("550e8400-e29b-41d4-a716-44665544000")); // too short
    }

    #[test]
    fn test_short_id_format() {
        let id = generate_short_id();
        assert_eq!(id.len(), 8, "Short ID should be 8 characters");
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'), 
                "Short ID should only contain URL-safe characters");
    }

    #[test]
    fn test_short_id_uniqueness() {
        let id1 = generate_short_id();
        let id2 = generate_short_id();
        assert_ne!(id1, id2, "Generated short IDs should be unique");
    }
}
