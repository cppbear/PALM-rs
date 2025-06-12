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
        unsafe { std::str::from_utf8_unchecked(b) }
    }
}

#[test]
fn test_deref_valid_utf8() {
    let byte_str = ByteStr::new(vec![72, 101, 108, 108, 111]); // "Hello"
    let result = byte_str.deref();
    assert_eq!(result, "Hello");
}

#[test]
fn test_deref_empty_string() {
    let byte_str = ByteStr::new(vec![]);
    let result = byte_str.deref();
    assert_eq!(result, "");
}

#[should_panic]
fn test_deref_invalid_utf8() {
    let byte_str = ByteStr::new(vec![255]); // Invalid UTF-8 byte
    let _result = byte_str.deref(); 
}

#[test]
fn test_deref_multibyte_characters() {
    let byte_str = ByteStr::new(vec![0xe2, 0x82, 0xac]); // "€"
    let result = byte_str.deref();
    assert_eq!(result, "€");
}

#[test]
fn test_deref_ascii_characters() {
    let byte_str = ByteStr::new(vec![97, 98, 99]); // "abc"
    let result = byte_str.deref();
    assert_eq!(result, "abc");
}

