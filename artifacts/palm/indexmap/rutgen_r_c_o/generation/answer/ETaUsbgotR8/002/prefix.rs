// Answer 0

#[test]
fn test_get_full_mut2_valid_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash};
    
    struct ValidKeyType(u32);
    
    impl Hash for ValidKeyType {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    
    impl Equivalent<ValidKeyType> for ValidKeyType {
        fn equivalent(&self, other: &ValidKeyType) -> bool {
            self.0 == other.0
        }
    }
    
    let mut index_map = IndexMap::<ValidKeyType, String, RandomState> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };
    
    index_map.insert(ValidKeyType(1), "value1".to_string());
    
    let key = ValidKeyType(1);
    
    let result = index_map.get_full_mut2(&key);
}

#[test]
fn test_get_full_mut2_with_non_existent_key() {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash};

    struct ValidKeyType(u32);

    impl Hash for ValidKeyType {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<ValidKeyType> for ValidKeyType {
        fn equivalent(&self, other: &ValidKeyType) -> bool {
            self.0 == other.0
        }
    }

    let mut index_map = IndexMap::<ValidKeyType, String, RandomState> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };

    let key = ValidKeyType(2);

    let result = index_map.get_full_mut2(&key);
}

#[test]
fn test_get_full_mut2_multiple_inserts() {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash};

    struct ValidKeyType(u32);

    impl Hash for ValidKeyType {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<ValidKeyType> for ValidKeyType {
        fn equivalent(&self, other: &ValidKeyType) -> bool {
            self.0 == other.0
        }
    }

    let mut index_map = IndexMap::<ValidKeyType, String, RandomState> {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };

    index_map.insert(ValidKeyType(0), "value0".to_string());
    index_map.insert(ValidKeyType(1), "value1".to_string());
    index_map.insert(ValidKeyType(2), "value2".to_string());

    let key = ValidKeyType(1);

    let result = index_map.get_full_mut2(&key);
}

