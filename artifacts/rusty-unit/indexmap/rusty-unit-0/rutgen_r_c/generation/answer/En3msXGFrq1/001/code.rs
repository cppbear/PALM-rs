// Answer 0

#[test]
fn test_remove_existing_key() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}

        fn write_u8(&mut self, _: u8) {}

        fn write_u16(&mut self, _: u16) {}

        fn write_u32(&mut self, _: u32) {}

        fn write_u64(&mut self, _: u64) {}

        fn write_usize(&mut self, _: usize) {}

        fn write_i8(&mut self, _: i8) {}

        fn write_i16(&mut self, _: i16) {}

        fn write_i32(&mut self, _: i32) {}

        fn write_i64(&mut self, _: i64) {}

        fn write_isize(&mut self, _: isize) {}

        fn write_str(&mut self, _: &str) {}

        fn write_u128(&mut self, _: u128) {}

        fn write_i128(&mut self, _: i128) {}
    }

    impl Hash for String {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_bytes().hash(state);
        }
    }

    let mut map = IndexMap::new();
    map.insert("key1".to_string(), 10);
    
    let result = map.remove(&"key1".to_string());
    
    assert_eq!(result, Some(10));
    assert!(!map.contains_key(&"key1".to_string()));
}

#[test]
fn test_remove_non_existing_key() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}

        fn write_u8(&mut self, _: u8) {}

        fn write_u16(&mut self, _: u16) {}

        fn write_u32(&mut self, _: u32) {}

        fn write_u64(&mut self, _: u64) {}

        fn write_usize(&mut self, _: usize) {}

        fn write_i8(&mut self, _: i8) {}

        fn write_i16(&mut self, _: i16) {}

        fn write_i32(&mut self, _: i32) {}

        fn write_i64(&mut self, _: i64) {}

        fn write_isize(&mut self, _: isize) {}

        fn write_str(&mut self, _: &str) {}

        fn write_u128(&mut self, _: u128) {}

        fn write_i128(&mut self, _: i128) {}
    }

    impl Hash for String {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_bytes().hash(state);
        }
    }

    let mut map = IndexMap::new();
    
    let result = map.remove(&"non_existing_key".to_string());
    
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_remove_panic() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}

        fn write_u8(&mut self, _: u8) {}

        fn write_u16(&mut self, _: u16) {}

        fn write_u32(&mut self, _: u32) {}

        fn write_u64(&mut self, _: u64) {}

        fn write_usize(&mut self, _: usize) {}

        fn write_i8(&mut self, _: i8) {}

        fn write_i16(&mut self, _: i16) {}

        fn write_i32(&mut self, _: i32) {}

        fn write_i64(&mut self, _: i64) {}

        fn write_isize(&mut self, _: isize) {}

        fn write_str(&mut self, _: &str) {}

        fn write_u128(&mut self, _: u128) {}

        fn write_i128(&mut self, _: i128) {}
    }

    impl Hash for String {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_bytes().hash(state);
        }
    }

    let mut map = IndexMap::new();
    map.insert("key1".to_string(), 10);
    
    // Assuming some internal condition that causes panic, 
    // this is for demonstration purposes.
    map.remove(unsafe { std::mem::transmute::<&str, &String>("key1") });
}

