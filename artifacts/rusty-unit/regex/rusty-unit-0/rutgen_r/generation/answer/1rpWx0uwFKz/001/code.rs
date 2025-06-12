// Answer 0

#[test]
fn test_set_limit_size_valid() {
    struct Literals {
        limit_size: usize,
    }

    impl Literals {
        pub fn new() -> Self {
            Literals { limit_size: 0 }
        }

        pub fn set_limit_size(&mut self, size: usize) -> &mut Literals {
            self.limit_size = size;
            self
        }
    }

    let mut literals = Literals::new();
    let result = literals.set_limit_size(100);
    assert_eq!(result.limit_size, 100);
}

#[test]
fn test_set_limit_size_zero() {
    struct Literals {
        limit_size: usize,
    }

    impl Literals {
        pub fn new() -> Self {
            Literals { limit_size: 0 }
        }

        pub fn set_limit_size(&mut self, size: usize) -> &mut Literals {
            self.limit_size = size;
            self
        }
    }

    let mut literals = Literals::new();
    let result = literals.set_limit_size(0);
    assert_eq!(result.limit_size, 0);
}

#[test]
fn test_set_limit_size_large_value() {
    struct Literals {
        limit_size: usize,
    }

    impl Literals {
        pub fn new() -> Self {
            Literals { limit_size: 0 }
        }

        pub fn set_limit_size(&mut self, size: usize) -> &mut Literals {
            self.limit_size = size;
            self
        }
    }

    let mut literals = Literals::new();
    let result = literals.set_limit_size(usize::MAX);
    assert_eq!(result.limit_size, usize::MAX);
}

