// Answer 0

#[derive(Debug, PartialEq)]
struct ByteStr {
    data: Vec<u8>,
}

impl ByteStr {
    pub fn new() -> Self {
        ByteStr { data: Vec::new() }
    }
}

#[derive(Debug, PartialEq)]
struct Authority {
    data: ByteStr,
}

impl Authority {
    pub(super) fn empty() -> Self {
        Authority {
            data: ByteStr::new(),
        }
    }
}

#[test]
fn test_empty_authority() {
    let expected = Authority {
        data: ByteStr::new(),
    };
    let actual = Authority::empty();
    assert_eq!(actual, expected);
}

#[test]
fn test_empty_byte_str() {
    let byte_str = ByteStr::new();
    assert_eq!(byte_str.data.len(), 0);
}

