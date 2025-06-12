// Answer 0

#[cfg(test)]
mod tests {
    use crate::hashbrown::HashMap;
    use crate::raw::Global;
    use std::hash::{Hash, BuildHasherDefault};
    
    #[test]
    fn test_insert_vacant_entry() {
        struct TestHasher;

        impl BuildHasher for TestHasher {
            type Hasher = std::collections::hash_map::RandomState;

            fn build_hasher(&self) -> Self::Hasher {
                std::collections::hash_map::RandomState::new()
            }
        }

        let mut map: HashMap<&str, u32, BuildHasherDefault<TestHasher>, Global> = HashMap::new();

        if let Some(o) = map.entry("poneyland").or_insert_with(|| {
            let vacant = VacantEntry {
                hash: 0,
                key: "poneyland",
                table: &mut map,
            };
            vacant
        }) {
            o.insert(37);
        }
        
        assert_eq!(map["poneyland"], 37);
    }

    #[test]
    fn test_insert_multiple_entries() {
        struct TestHasher;

        impl BuildHasher for TestHasher {
            type Hasher = std::collections::hash_map::RandomState;

            fn build_hasher(&self) -> Self::Hasher {
                std::collections::hash_map::RandomState::new()
            }
        }

        let mut map: HashMap<&str, u32, BuildHasherDefault<TestHasher>, Global> = HashMap::new();
        
        for i in 1..=5 {
            let key = format!("key{}", i);
            if let Some(o) = map.entry(key.as_str()).or_insert_with(|| {
                let vacant = VacantEntry {
                    hash: 0,
                    key: key.as_str(),
                    table: &mut map,
                };
                vacant
            }) {
                o.insert(i);
            }
        }

        for i in 1..=5 {
            assert_eq!(map[format!("key{}", i).as_str()], i);
        }
    }
}

