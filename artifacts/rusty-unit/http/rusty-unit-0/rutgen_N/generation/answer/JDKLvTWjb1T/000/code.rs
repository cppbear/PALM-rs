// Answer 0

#[derive(Debug)]
struct AllocatedExtension(Vec<u8>);

impl AllocatedExtension {
    fn new(data: Vec<u8>) -> Self {
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
    let data = AllocatedExtension::new(b"Hello, world!".to_vec());
    assert_eq!(data.as_str(), "Hello, world!");
}

#[test]
fn test_as_str_empty_string() {
    let data = AllocatedExtension::new(Vec::new());
    assert_eq!(data.as_str(), "");
}

#[test]
#[should_panic]
fn test_as_str_invalid_utf8() {
    let data = AllocatedExtension::new(vec![255, 255, 255]);
    // This should not panic at compile time but will at runtime
    // because the byte array does not constitute valid UTF-8.
    // Unsafe code assumes this is valid, and we're invoking a failure case.
    let _ = data.as_str();
}

