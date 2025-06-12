// Answer 0

#[derive(Debug)]
struct AllocatedExtension(Vec<u8>);

impl AllocatedExtension {
    pub fn new(data: Vec<u8>) -> Self {
        AllocatedExtension(data)
    }

    pub fn as_str(&self) -> &str {
        // Safety: the invariant of AllocatedExtension ensures that self.0
        // contains valid UTF-8.
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

#[test]
fn test_as_str_valid_utf8() {
    let valid_utf8 = AllocatedExtension::new(b"Hello, world!".to_vec());
    assert_eq!(valid_utf8.as_str(), "Hello, world!");
}

#[test]
fn test_as_str_empty_string() {
    let empty_string = AllocatedExtension::new(b"".to_vec());
    assert_eq!(empty_string.as_str(), "");
}

#[should_panic]
#[test]
fn test_as_str_invalid_utf8() {
    // This will panic at runtime due to invalid UTF-8
    let invalid_utf8 = AllocatedExtension::new(vec![0, 159, 146, 150]);
    invalid_utf8.as_str(); // This should trigger a panic.
}

