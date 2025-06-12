// Answer 0

#[derive(Debug)]
struct ByteStr {
    bytes: Vec<u8>,
}

impl ByteStr {
    fn new(bytes: Vec<u8>) -> Self {
        ByteStr { bytes }
    }

    fn deref(&self) -> &str {
        let b: &[u8] = self.bytes.as_ref();
        // Safety: the invariant of `bytes` is that it contains valid UTF-8.
        unsafe { std::str::from_utf8_unchecked(b) }
    }
}

#[test]
fn test_deref_valid_utf8() {
    let byte_str = ByteStr::new(vec![104, 101, 108, 108, 111]); // "hello" in UTF-8
    let result = byte_str.deref();
    assert_eq!(result, "hello");
}

#[test]
fn test_deref_empty() {
    let byte_str = ByteStr::new(vec![]);
    let result = byte_str.deref();
    assert_eq!(result, "");
}

#[should_panic]
fn test_deref_invalid_utf8() {
    let byte_str = ByteStr::new(vec![255]); // Invalid UTF-8 byte
    let _result = byte_str.deref(); // This should cause a panic, but it's unsafe block
}

