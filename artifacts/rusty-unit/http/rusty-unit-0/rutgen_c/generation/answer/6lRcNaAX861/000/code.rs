// Answer 0

#[test]
fn test_hash_for_header_value() {
    use std::collections::hash_map::DefaultHasher;

    struct TestHasher {
        hasher: DefaultHasher,
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            self.hasher.finish()
        }

        fn write(&mut self, bytes: &[u8]) {
            self.hasher.write(bytes);
        }

        fn write_u8(&mut self, i: u8) {
            self.hasher.write_u8(i);
        }

        fn write_u32(&mut self, i: u32) {
            self.hasher.write_u32(i);
        }

        fn write_u64(&mut self, i: u64) {
            self.hasher.write_u64(i);
        }

        fn write_usize(&mut self, i: usize) {
            self.hasher.write_usize(i);
        }

        fn write_i8(&mut self, i: i8) {
            self.hasher.write_i8(i);
        }

        fn write_i32(&mut self, i: i32) {
            self.hasher.write_i32(i);
        }

        fn write_i64(&mut self, i: i64) {
            self.hasher.write_i64(i);
        }

        fn write_isize(&mut self, i: isize) {
            self.hasher.write_isize(i);
        }
    }

    let value = HeaderValue {
        inner: Bytes::from_static(b"test_value"),
        is_sensitive: false,
    };
    
    let mut hasher = TestHasher {
        hasher: DefaultHasher::new(),
    };

    value.hash(&mut hasher);
    let hash_result = hasher.finish();

    assert!(hash_result != 0); // Check that the hash is non-zero
}

