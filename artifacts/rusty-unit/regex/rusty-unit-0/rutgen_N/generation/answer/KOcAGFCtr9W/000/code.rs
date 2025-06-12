// Answer 0

#[derive(Debug)]
struct Captures {
    data: Vec<u8>,
}

impl Captures {
    fn expand(&self, pattern: &[u8], dst: &mut Vec<u8>) {
        dst.extend_from_slice(pattern);
        dst.extend_from_slice(&self.data);
    }
}

#[test]
fn test_replace_append() {
    let mut dst: Vec<u8> = Vec::new();
    let pattern: &[u8] = b"pattern_";
    let captures = Captures { data: b"captured_data".to_vec() };

    replace_append(pattern, &captures, &mut dst);

    let expected: Vec<u8> = [b'p', b'a', b't', b't', b'e', b'r', b'n', b'_', b'c', b'a', b'p', b't', b'u', b'r', b'e', b'd', b'_', b'd', b'a', b't', b'a'].to_vec();
    assert_eq!(dst, expected);
}

