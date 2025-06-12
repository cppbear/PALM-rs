// Answer 0

#[test]
fn test_find_with_non_empty_entries() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct HeaderName(String);

    #[derive(Debug)]
    struct Entry {
        key: HeaderName,
    }

    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<(usize, usize)>>,
        mask: usize,
        danger: DefaultHasher,
    }

    impl Map {
        fn new() -> Self {
            Self {
                entries: vec![],
                indices: vec![],
                mask: 0,
                danger: DefaultHasher::new(),
            }
        }

        fn find<K>(&self, key: &K) -> Option<(usize, usize)>
        where
            K: Hash + Into<HeaderName> + ?Sized,
            HeaderName: PartialEq<K>,
        {
            if self.entries.is_empty() {
                return None;
            }

            let hash = hash_elem_using(&self.danger, key);
            let mask = self.mask;
            let mut probe = desired_pos(mask, hash);
            let mut dist = 0;

            probe_loop!(probe < self.indices.len(), {
                if let Some((i, entry_hash)) = self.indices[probe].resolve() {
                    if dist > probe_distance(mask, entry_hash, probe) {
                        return None;
                    } else if entry_hash == hash && self.entries[i].key == *key {
                        return Some((probe, i));
                    }
                } else {
                    return None;
                }

                dist += 1;
            });
        }
    }

    fn hash_elem_using<S: Hash>(hasher: &mut S, key: &HeaderName) -> usize {
        key.hash(hasher);
        hasher.finish() as usize
    }

    fn desired_pos(mask: usize, hash: usize) -> usize {
        hash & mask
    }

    // Mocking 'probe_loop!' macro functionality
    macro_rules! probe_loop {
        ($condition:expr, $body:block) => {
            while $condition {
                $body
            }
        };
    }

    fn probe_distance(mask: usize, entry_hash: usize, probe: usize) -> usize {
        (entry_hash & mask).wrapping_sub(probe & mask)
    }

    let mut map = Map::new();
    map.entries.push(Entry { key: HeaderName("key1".to_string()) });
    map.indices.push(Some((0, 0))); // mock resolved probe entry

    assert_eq!(map.find(&HeaderName("key1".to_string())), Some((0, 0)));
}

#[test]
fn test_find_with_empty_entries() {
    struct HeaderName(String);

    #[derive(Debug)]
    struct Entry {
        key: HeaderName,
    }

    struct Map {
        entries: Vec<Entry>,
        indices: Vec<Option<(usize, usize)>>,
        mask: usize,
        danger: std::collections::hash_map::DefaultHasher,
    }

    impl Map {
        fn new() -> Self {
            Self {
                entries: vec![],
                indices: vec![],
                mask: 0,
                danger: std::collections::hash_map::DefaultHasher::new(),
            }
        }

        fn find<K>(&self, key: &K) -> Option<(usize, usize)>
        where
            K: Hash + Into<HeaderName> + ?Sized,
            HeaderName: PartialEq<K>,
        {
            if self.entries.is_empty() {
                return None;
            }

            let hash = hash_elem_using(&self.danger, key);
            let mask = self.mask;
            let mut probe = desired_pos(mask, hash);
            let mut dist = 0;

            probe_loop!(probe < self.indices.len(), {
                if let Some((i, entry_hash)) = self.indices[probe].resolve() {
                    if dist > probe_distance(mask, entry_hash, probe) {
                        return None;
                    } else if entry_hash == hash && self.entries[i].key == *key {
                        return Some((probe, i));
                    }
                } else {
                    return None;
                }

                dist += 1;
            });
        }
    }

    fn hash_elem_using<S: Hash>(hasher: &mut S, key: &HeaderName) -> usize {
        key.hash(hasher);
        hasher.finish() as usize
    }

    fn desired_pos(mask: usize, hash: usize) -> usize {
        hash & mask
    }

    macro_rules! probe_loop {
        ($condition:expr, $body:block) => {
            while $condition {
                $body
            }
        };
    }

    fn probe_distance(mask: usize, entry_hash: usize, probe: usize) -> usize {
        (entry_hash & mask).wrapping_sub(probe & mask)
    }

    let map = Map::new();

    assert_eq!(map.find(&HeaderName("key1".to_string())), None);
}

