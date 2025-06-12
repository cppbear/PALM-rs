// Answer 0

#[test]
fn test_is_empty_when_length_is_zero() {
    struct Literal {
        len: usize,
    }

    impl Literal {
        pub fn new(len: usize) -> Self {
            Literal { len }
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let literal = Literal::new(0);
    assert!(literal.is_empty());
}

#[test]
fn test_is_empty_when_length_is_non_zero() {
    struct Literal {
        len: usize,
    }

    impl Literal {
        pub fn new(len: usize) -> Self {
            Literal { len }
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }
    }

    let literal = Literal::new(1);
    assert!(!literal.is_empty());
}

