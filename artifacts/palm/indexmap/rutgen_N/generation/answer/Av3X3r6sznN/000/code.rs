// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasherDefault, Hasher};

    struct TestMap {
        hash_builder: BuildHasherDefault<DefaultHasher>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap {
                hash_builder: BuildHasherDefault::new(),
            }
        }

        pub fn hasher(&self) -> &BuildHasherDefault<DefaultHasher> {
            &self.hash_builder
        }
    }

    #[test]
    fn test_hasher() {
        let test_map = TestMap::new();
        let hasher_ref = test_map.hasher();
        assert!(hasher_ref != std::ptr::null_mut());
    }

    #[test]
    fn test_hasher_initialization() {
        let test_map = TestMap::new();
        let hasher_ref = test_map.hasher();
        let mut hasher = hasher_ref.build_hasher();
        hasher.write_u8(42);
        assert_eq!(hasher.finish(), 12067319864318627941);
    }
}

