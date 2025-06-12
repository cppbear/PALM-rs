// Answer 0

#[test]
fn test_size_limit() {
    struct Compile {
        size_limit: usize,
    }

    impl Compile {
        pub fn new() -> Self {
            Compile { size_limit: 0 }
        }

        pub fn size_limit(mut self, size_limit: usize) -> Self {
            self.size_limit = size_limit;
            self
        }
    }

    let compiler = Compile::new();
    let compiler_with_limit = compiler.size_limit(1024);
    assert_eq!(compiler_with_limit.size_limit, 1024);
}

#[test]
fn test_size_limit_zero() {
    struct Compile {
        size_limit: usize,
    }

    impl Compile {
        pub fn new() -> Self {
            Compile { size_limit: 0 }
        }

        pub fn size_limit(mut self, size_limit: usize) -> Self {
            self.size_limit = size_limit;
            self
        }
    }

    let compiler = Compile::new();
    let compiler_with_zero_limit = compiler.size_limit(0);
    assert_eq!(compiler_with_zero_limit.size_limit, 0);
}

#[test]
fn test_size_limit_large_value() {
    struct Compile {
        size_limit: usize,
    }

    impl Compile {
        pub fn new() -> Self {
            Compile { size_limit: 0 }
        }

        pub fn size_limit(mut self, size_limit: usize) -> Self {
            self.size_limit = size_limit;
            self
        }
    }

    let compiler = Compile::new();
    let compiler_with_large_limit = compiler.size_limit(usize::MAX);
    assert_eq!(compiler_with_large_limit.size_limit, usize::MAX);
}

