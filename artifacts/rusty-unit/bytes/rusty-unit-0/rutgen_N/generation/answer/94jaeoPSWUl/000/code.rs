// Answer 0

#[test]
fn test_is_empty_with_empty_bytes() {
    struct Bytes {
        len: usize,
    }

    impl Bytes {
        pub const fn new() -> Self {
            Self { len: 0 }
        }

        pub const fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = Bytes::new();
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_bytes() {
    struct Bytes {
        len: usize,
    }

    impl Bytes {
        pub const fn new(len: usize) -> Self {
            Self { len }
        }

        pub const fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = Bytes::new(1);
    assert!(!b.is_empty());
}

