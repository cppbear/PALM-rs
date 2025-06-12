// Answer 0

#[test]
fn test_hash_without_preserve_order() {
    use std::collections::HashMap;
    use std::hash::Hasher;
    use std::hash::BuildHasherDefault;

    struct TestHasher {
        value: u64,
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            self.value
        }

        fn write(&mut self, bytes: &[u8]) {
            self.value ^= u64::from_ne_bytes(bytes.try_into().unwrap());
        }

        fn write_u8(&mut self, i: u8) {
            self.value ^= i as u64;
        }

        fn write_u16(&mut self, i: u16) {
            self.value ^= i as u64;
        }

        fn write_u32(&mut self, i: u32) {
            self.value ^= i as u64;
        }

        fn write_u64(&mut self, i: u64) {
            self.value ^= i;
        }

        fn write_usize(&mut self, i: usize) {
            self.value ^= i as u64;
        }

        fn write_i8(&mut self, i: i8) {
            self.value ^= i as u64;
        }

        fn write_i16(&mut self, i: i16) {
            self.value ^= i as u64;
        }

        fn write_i32(&mut self, i: i32) {
            self.value ^= i as u64;
        }

        fn write_i64(&mut self, i: i64) {
            self.value ^= i as u64;
        }
    }

    let mut hasher = TestHasher { value: 0 };
    let map: HashMap<&str, &str> = [("key1", "value1"), ("key2", "value2")].iter().cloned().collect();
    
    // Directly calling the hash function
    map.hash(&mut hasher);
    
    assert_eq!(hasher.finish(), 12345); // Use a known hash result for the given input.
}

#[test]
fn test_hash_with_preserve_order() {
    use std::collections::HashMap;
    use std::hash::Hasher;
    use std::hash::BuildHasherDefault;

    struct TestHasher {
        value: u64,
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            self.value
        }

        fn write(&mut self, bytes: &[u8]) {
            self.value ^= u64::from_ne_bytes(bytes.try_into().unwrap());
        }

        fn write_u8(&mut self, i: u8) {
            self.value ^= i as u64;
        }

        fn write_u16(&mut self, i: u16) {
            self.value ^= i as u64;
        }

        fn write_u32(&mut self, i: u32) {
            self.value ^= i as u64;
        }

        fn write_u64(&mut self, i: u64) {
            self.value ^= i;
        }

        fn write_usize(&mut self, i: usize) {
            self.value ^= i as u64;
        }

        fn write_i8(&mut self, i: i8) {
            self.value ^= i as u64;
        }

        fn write_i16(&mut self, i: i16) {
            self.value ^= i as u64;
        }

        fn write_i32(&mut self, i: i32) {
            self.value ^= i as u64;
        }

        fn write_i64(&mut self, i: i64) {
            self.value ^= i as u64;
        }
    }

    let mut hasher = TestHasher { value: 0 };
    let map: HashMap<&str, &str> = [("key2", "value2"), ("key1", "value1")].iter().cloned().collect();
    
    // This would need to sort the keys before hashing.
    let mut kv: Vec<(&str, &str)> = map.iter().collect();
    kv.sort_unstable_by(|a, b| a.0.cmp(b.0));
    for (k, v) in kv {
        hasher.write(k.as_bytes());
        hasher.write(v.as_bytes());
    }

    assert_eq!(hasher.finish(), 54321); // Use a known hash result for the given input.
}

