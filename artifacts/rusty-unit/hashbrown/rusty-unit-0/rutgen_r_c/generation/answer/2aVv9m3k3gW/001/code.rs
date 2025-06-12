// Answer 0

#[test]
fn test_make_hasher_with_string_keys() {
    use core::hash::BuildHasherDefault;
    use std::collections::HashMap;
    use core::hash::Hasher;

    struct CustomHasher {
        total: u64,
    }

    impl Hasher for CustomHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.total += bytes.len() as u64;
        }

        fn finish(&self) -> u64 {
            self.total
        }
    }

    let make_hash = |hash_builder: &BuildHasherDefault<CustomHasher>, val: &String| {
        let mut hasher = hash_builder.build_hasher();
        hasher.write(val.as_bytes());
        hasher.finish()
    };

    let hash_builder = BuildHasherDefault::<CustomHasher>::default();
    let hasher = make_hasher::<String, i32, _>(&hash_builder);

    let key1 = String::from("key1");
    let key2 = String::from("key2");

    let result1 = hasher(&(key1.clone(), 10));
    let result2 = hasher(&(key2.clone(), 20));

    assert!(result1 != result2);
}

#[test]
fn test_make_hasher_with_integer_keys() {
    use core::hash::BuildHasherDefault;
    use core::hash::Hasher;

    struct CustomHasher {
        total: u64,
    }

    impl Hasher for CustomHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.total += bytes.len() as u64;
        }

        fn finish(&self) -> u64 {
            self.total
        }
    }

    let make_hash = |hash_builder: &BuildHasherDefault<CustomHasher>, val: &u32| {
        let mut hasher = hash_builder.build_hasher();
        val.hash(&mut hasher);
        hasher.finish()
    };

    let hash_builder = BuildHasherDefault::<CustomHasher>::default();
    let hasher = make_hasher::<u32, i32, _>(&hash_builder);

    let key1 = 42u32;
    let key2 = 99u32;

    let result1 = hasher(&(key1, 10));
    let result2 = hasher(&(key2, 20));

    assert!(result1 != result2);
}

#[test]
fn test_make_hasher_with_panic_condition() {
    use core::hash::BuildHasherDefault;
    use core::hash::Hasher;

    struct CustomHasher {
        total: u64,
    }

    impl Hasher for CustomHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.total += bytes.len() as u64;
        }

        fn finish(&self) -> u64 {
            self.total
        }
    }

    let hash_builder = BuildHasherDefault::<CustomHasher>::default();
    let hasher = make_hasher::<String, i32, _>(&hash_builder);

    let key = String::from("panic_key");
    
    // This will intentionally panic if something goes wrong while creating the hash
    let result = std::panic::catch_unwind(|| {
        hasher(&(key, 10));
    });

    assert!(result.is_ok());
}

