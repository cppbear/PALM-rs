// Answer 0

#[derive(Debug)]
struct ExampleHasher {
    value: Vec<u8>,
}

impl std::hash::Hasher for ExampleHasher {
    fn finish(&self) -> u64 {
        self.value.iter().fold(0, |acc, &b| acc.wrapping_add(b as u64))
    }

    fn write(&mut self, bytes: &[u8]) {
        self.value.extend_from_slice(bytes);
    }

    fn write_u8(&mut self, i: u8) {
        self.value.push(i);
    }
}

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

#[derive(Debug)]
struct Uri {
    inner: Scheme2,
}

impl Uri {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
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

#[test]
fn test_hash_other_no_bytes() {
    let mut hasher = ExampleHasher { value: Vec::new() };
    let uri = Uri {
        inner: Scheme2::Other(String::new()), // Constraint: Empty string, &b in other.as_bytes() is false
    };
    uri.hash(&mut hasher);
    assert_eq!(hasher.value, vec![0]); // The length of the empty string is 0, should write nothing else.
}

#[test]
fn test_hash_other_with_bytes() {
    let mut hasher = ExampleHasher { value: Vec::new() };
    let uri = Uri {
        inner: Scheme2::Other("Example".to_string()), // Non-empty string, will be processed
    };
    uri.hash(&mut hasher);
    assert_eq!(hasher.value.len(), 8); // 1 for the length and 7 for the bytes of 'Example'
    assert_eq!(hasher.value[0], 7); // Length of "Example"
    assert_eq!(hasher.value[1..], b"example"); // Should be the lowercased bytes
}

#[test]
fn test_hash_other_special_chars() {
    let mut hasher = ExampleHasher { value: Vec::new() };
    let uri = Uri {
        inner: Scheme2::Other("TeSt@123".to_string()), // Now using special characters
    };
    uri.hash(&mut hasher);
    assert_eq!(hasher.value.len(), 11); // 1 for length and 10 for characters in lower case
    assert_eq!(hasher.value[0], 10); // Length of "TeSt@123"
    assert_eq!(hasher.value[1..], b"test@123"); // Lowercased bytes
}

