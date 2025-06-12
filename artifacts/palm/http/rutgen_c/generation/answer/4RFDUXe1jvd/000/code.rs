// Answer 0

#[test]
fn test_write_unreachable() {
    struct TestHasher;

    impl Hasher for TestHasher {
        fn write(&mut self, _: &[u8]) {
            panic!("This should not be called.");
        }
        fn write_u64(&mut self, _: u64) {}
        fn finish(&self) -> u64 {
            0
        }
    }

    let mut hasher = TestHasher;
    let result = std::panic::catch_unwind(|| {
        hasher.write(b"test");
    });

    assert!(result.is_err());
}

#[test]
fn test_finish() {
    let hasher = IdHasher::default();
    let result = hasher.finish();
    assert_eq!(result, 0);
}

