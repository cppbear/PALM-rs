// Answer 0

#[test]
fn test_hash_lower_case() {
    use std::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;

    struct TestHasher {
        hasher: DefaultHasher,
    }

    impl TestHasher {
        fn new() -> Self {
            TestHasher {
                hasher: DefaultHasher::new(),
            }
        }

        fn finish(self) -> u64 {
            self.hasher.finish()
        }
    }

    struct HeaderName {
        lower: bool,
        buf: &'static [u8],
    }

    impl HeaderName {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            if self.lower {
                hasher.write(self.buf);
            } else {
                for &b in self.buf {
                    hasher.write(&[b]);
                }
            }
        }
    }

    let header = HeaderName {
        lower: true,
        buf: b"test",
    };

    let mut hasher = TestHasher::new();
    header.hash(&mut hasher.hasher);
    let result = hasher.finish();

    assert_ne!(result, 0);
}

#[test]
fn test_hash_upper_case() {
    use std::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;

    struct TestHasher {
        hasher: DefaultHasher,
    }

    impl TestHasher {
        fn new() -> Self {
            TestHasher {
                hasher: DefaultHasher::new(),
            }
        }

        fn finish(self) -> u64 {
            self.hasher.finish()
        }
    }

    struct HeaderName {
        lower: bool,
        buf: &'static [u8],
    }

    impl HeaderName {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            if self.lower {
                hasher.write(self.buf);
            } else {
                for &b in self.buf {
                    hasher.write(&[b]);
                }
            }
        }
    }

    let header = HeaderName {
        lower: false,
        buf: b"TEST",
    };

    let mut hasher = TestHasher::new();
    header.hash(&mut hasher.hasher);
    let result = hasher.finish();

    assert_ne!(result, 0);
}

