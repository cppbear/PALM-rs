// Answer 0

#[test]
fn test_hash_none() {
    use std::collections::hash_map::DefaultHasher;

    struct TestHasher(DefaultHasher);

    impl Hasher for TestHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes);
        }

        fn finish(&self) -> u64 {
            self.0.finish()
        }
    }

    let scheme = Scheme { inner: Scheme2::None };
    let mut hasher = TestHasher(DefaultHasher::new());
    scheme.hash(&mut hasher);
    assert_eq!(hasher.finish(), 0);
}

#[test]
fn test_hash_http() {
    use std::collections::hash_map::DefaultHasher;

    struct TestHasher(DefaultHasher);

    impl Hasher for TestHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes);
        }

        fn finish(&self) -> u64 {
            self.0.finish()
        }
    }

    let scheme = Scheme { inner: Scheme2::Standard(Protocol::Http) };
    let mut hasher = TestHasher(DefaultHasher::new());
    scheme.hash(&mut hasher);
    assert_eq!(hasher.finish(), 1);
}

#[test]
fn test_hash_https() {
    use std::collections::hash_map::DefaultHasher;

    struct TestHasher(DefaultHasher);

    impl Hasher for TestHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes);
        }

        fn finish(&self) -> u64 {
            self.0.finish()
        }
    }

    let scheme = Scheme { inner: Scheme2::Standard(Protocol::Https) };
    let mut hasher = TestHasher(DefaultHasher::new());
    scheme.hash(&mut hasher);
    assert_eq!(hasher.finish(), 2);
}

