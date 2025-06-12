// Answer 0

#[test]
fn test_is_empty_with_zero_length() {
    struct Bytes {
        len: usize,
    }
    
    impl Bytes {
        const fn new() -> Self {
            Bytes { len: 0 }
        }

        const fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = Bytes::new();
    assert!(b.is_empty());
}

#[test]
fn test_is_empty_with_non_zero_length() {
    struct Bytes {
        len: usize,
    }
    
    impl Bytes {
        const fn new(len: usize) -> Self {
            Bytes { len }
        }

        const fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = Bytes::new(1);
    assert!(!b.is_empty());
}

#[test]
fn test_is_empty_boundary_condition() {
    struct Bytes {
        len: usize,
    }
    
    impl Bytes {
        const fn new(len: usize) -> Self {
            Bytes { len }
        }

        const fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let b = Bytes::new(0);
    assert!(b.is_empty());

    let b_non_empty = Bytes::new(usize::MAX);
    assert!(!b_non_empty.is_empty());
}

