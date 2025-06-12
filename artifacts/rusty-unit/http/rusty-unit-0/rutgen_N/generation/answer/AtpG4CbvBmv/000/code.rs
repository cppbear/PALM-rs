// Answer 0

#[derive(Debug)]
struct Authority {
    data: ByteStr,
}

#[derive(Debug)]
struct ByteStr {
    bytes: Vec<u8>,
}

impl ByteStr {
    pub fn new() -> Self {
        ByteStr { bytes: Vec::new() }
    }
}

impl Authority {
    pub(super) fn empty() -> Self {
        Authority {
            data: ByteStr::new(),
        }
    }
}

#[test]
fn test_authority_empty() {
    let authority = Authority::empty();
    assert_eq!(authority.data.bytes.len(), 0);
}

