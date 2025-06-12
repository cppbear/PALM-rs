// Answer 0

#[test]
fn test_lookup_with_uninitialized_map() {
    struct MockMap {
        value: usize,
        key: u32,
    }
    
    struct TestStruct {
        mask: u32,
        map: Option<Vec<MockMap>>,
    }
    
    impl TestStruct {
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

    // This will panic because map is None
    let test_struct = TestStruct { mask: 3, map: None };
    let result = std::panic::catch_unwind(|| test_struct.lookup(1));
    assert!(result.is_err());
}

#[test]
fn test_lookup_with_empty_map() {
    struct MockMap {
        value: usize,
        key: u32,
    }
    
    struct TestStruct {
        mask: u32,
        map: Option<Vec<MockMap>>,
    }
    
    impl TestStruct {
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

    let map = vec![MockMap { value: 0, key: 0 }; 4];
    let test_struct = TestStruct { mask: 3, map: Some(map) };

    // Lookup the key that is not present and map[i].value should be Default::default()
    let result = test_struct.lookup(5);
    assert_eq!(result, 1); // Expecting to return an index based on Default case
}

#[test]
fn test_lookup_with_populated_map() {
    struct MockMap {
        value: usize,
        key: u32,
    }
    
    struct TestStruct {
        mask: u32,
        map: Option<Vec<MockMap>>,
    }
    
    impl TestStruct {
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

    let map = vec![
        MockMap { value: 1, key: 1 },
        MockMap { value: 2, key: 2 },
        MockMap { value: 0, key: 0 }, // Default case
        MockMap { value: 3, key: 3 },
    ];
    
    let test_struct = TestStruct { mask: 3, map: Some(map) };

    // Looking for a key that exists
    let result_existing = test_struct.lookup(2);
    assert_eq!(result_existing, 2); // Should return the index where key 2 is

    // Looking for a key that does not exist but causes hash to default
    let result_non_existing = test_struct.lookup(4);
    assert_eq!(result_non_existing, 0); // Expecting the next available default index based on hashing logic
}

