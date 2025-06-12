// Answer 0

#[derive(Debug)]
enum Protocol {
    Http,
    Https,
}

#[derive(Debug)]
enum Scheme2 {
    None,
    Standard(Protocol),
    Other(String),
}

struct Uri {
    inner: Scheme2,
}

use std::hash::{Hasher, Hash};

impl Uri {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self.inner {
            Scheme2::None => (),
            Scheme2::Standard(Protocol::Http) => state.write_u8(1),
            Scheme2::Standard(Protocol::Https) => state.write_u8(2),
            Scheme2::Other(ref other) => {
                other.len().hash(state);
                for &b in other.as_bytes() {
                    state.write_u8(b.to_ascii_lowercase());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_hash_scheme2_other_non_empty() {
        let uri = Uri {
            inner: Scheme2::Other("Example".to_string()),
        };
        let mut hasher = DefaultHasher::new();
        uri.hash(&mut hasher);
        let result = hasher.finish();
        // Assert on the result as needed based on the expected hashing output
    }

    #[test]
    fn test_hash_scheme2_other_empty() {
        let uri = Uri {
            inner: Scheme2::Other("".to_string()),
        };
        let mut hasher = DefaultHasher::new();
        uri.hash(&mut hasher);
        let result = hasher.finish();
        // Assert on the result as needed; it should handle empty string gracefully
    }

    #[test]
    fn test_hash_scheme2_other_with_special_characters() {
        let uri = Uri {
            inner: Scheme2::Other("Example@2023!".to_string()),
        };
        let mut hasher = DefaultHasher::new();
        uri.hash(&mut hasher);
        let result = hasher.finish();
        // Assert on the result as needed; special characters should be handled properly
    }
    
    #[test]
    fn test_hash_scheme2_other_with_uppercase() {
        let uri = Uri {
            inner: Scheme2::Other("EXAMPLE".to_string()),
        };
        let mut hasher = DefaultHasher::new();
        uri.hash(&mut hasher);
        let result = hasher.finish();
        // Assert on the result as needed; should be consistent with lowercase conversion
    }
}

