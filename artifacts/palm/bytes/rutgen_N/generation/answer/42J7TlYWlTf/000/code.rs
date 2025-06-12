// Answer 0

#[test]
fn test_len_empty_bytes() {
    struct Bytes {
        len: usize,
    }

    impl Bytes {
        pub fn from(data: &[u8]) -> Self {
            Self { len: data.len() }
        }

        pub const fn len(&self) -> usize {
            self.len
        }
    }

    let b = Bytes::from(&b""[..]);
    assert_eq!(b.len(), 0);
}

#[test]
fn test_len_non_empty_bytes() {
    struct Bytes {
        len: usize,
    }

    impl Bytes {
        pub fn from(data: &[u8]) -> Self {
            Self { len: data.len() }
        }

        pub const fn len(&self) -> usize {
            self.len
        }
    }

    let b = Bytes::from(&b"hello"[..]);
    assert_eq!(b.len(), 5);
}

#[test]
fn test_len_long_bytes() {
    struct Bytes {
        len: usize,
    }

    impl Bytes {
        pub fn from(data: &[u8]) -> Self {
            Self { len: data.len() }
        }

        pub const fn len(&self) -> usize {
            self.len
        }
    }

    let b = Bytes::from(&b"this is a longer string"[..]);
    assert_eq!(b.len(), 25);
}

