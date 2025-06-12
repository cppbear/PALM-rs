// Answer 0

#[test]
fn test_lookup_existing_key() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new(size: usize) -> Self {
            let map = vec![MapEntry { key: 0, value: 0 }; size];
            let mask = (size - 1) as u32;
            HashMap { map, mask }
        }

        fn lookup(&self, key: u32) -> usize {
            let hash = key;
            let mut i = hash as usize & self.mask as usize;

            let map = &self.map;

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

    let mut hashmap = HashMap::new(16);
    hashmap.map[5] = MapEntry { key: 10, value: 1 }; // existing entry

    assert_eq!(hashmap.lookup(10), 5);
}

#[test]
fn test_lookup_non_existing_key() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new(size: usize) -> Self {
            let map = vec![MapEntry { key: 0, value: 0 }; size];
            let mask = (size - 1) as u32;
            HashMap { map, mask }
        }

        fn lookup(&self, key: u32) -> usize {
            let hash = key;
            let mut i = hash as usize & self.mask as usize;

            let map = &self.map;

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

    let hashmap = HashMap::new(16);
    
    assert_ne!(hashmap.lookup(99), 0);
}

