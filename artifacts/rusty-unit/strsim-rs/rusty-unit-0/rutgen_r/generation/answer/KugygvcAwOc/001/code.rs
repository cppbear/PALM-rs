// Answer 0

#[derive(Default)]
struct Entry {
    key: u32,
    value: usize,
}

struct HashMap {
    map: Option<Vec<Entry>>,
    mask: u32,
}

impl HashMap {
    fn lookup(&self, key: u32) -> usize {
        let hash = key;
        let mut i = hash as usize & self.mask as usize;

        let map = self
            .map
            .as_ref()
            .expect("callers have to ensure map is allocated");

        if map[i].value == Default::default() || map[i].key == key {
            return i;
        }

        let mut perturb = key;
        loop {
            i = (i * 5 + perturb as usize + 1) & self.mask as usize;

            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }

            perturb >>= 5;
        }
    }
}

#[test]
fn test_lookup_with_empty_map() {
    let map_structure = vec![Entry::default(); 16]; // Creating a map of 16 empty entries
    let hashmap = HashMap {
        map: Some(map_structure),
        mask: 15,
    };
    assert_eq!(hashmap.lookup(0), 0); // Key 0 should return index 0
}

#[test]
fn test_lookup_with_key_present() {
    let mut map_structure = vec![Entry::default(); 16];
    map_structure[2] = Entry { key: 2, value: 10 }; // Insert key 2
    let hashmap = HashMap {
        map: Some(map_structure),
        mask: 15,
    };
    assert_eq!(hashmap.lookup(2), 2); // Key 2 should return index 2
}

#[test]
fn test_lookup_with_collision_resolution() {
    let mut map_structure = vec![Entry::default(); 16];
    map_structure[1] = Entry { key: 1, value: 10 }; // Insert key 1 at index 1
    map_structure[2] = Entry { key: 2, value: 10 }; // Insert key 2 at index 2
    let hashmap = HashMap {
        map: Some(map_structure),
        mask: 15,
    };
    assert_eq!(hashmap.lookup(1), 1); // Should resolve to index 1
    assert_eq!(hashmap.lookup(2), 2); // Should resolve to index 2
}

#[test]
#[should_panic(expected = "callers have to ensure map is allocated")]
fn test_lookup_without_map_allocated() {
    let hashmap = HashMap {
        map: None,
        mask: 15,
    };
    hashlookup(0); // Should panic
}

#[test]
fn test_lookup_with_nonexistent_key() {
    let mut map_structure = vec![Entry::default(); 16]; // Empty map
    let hashmap = HashMap {
        map: Some(map_structure),
        mask: 15,
    };
    assert_eq!(hashmap.lookup(3), 3); // Key 3 hasn't been added, but should return its index
}

