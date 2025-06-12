// Answer 0

#[test]
fn test_get_with_empty_map() {
    struct TestStruct {
        map: Option<Vec<TestValue>>,
    }

    struct TestValue {
        value: u32,
    }

    impl TestStruct {
        fn lookup(&self, key: u32) -> usize {
            // Return 0 for this test case assuming we have at least 1 element in the map.
            0
        }

        fn get(&self, key: u32) -> u32 {
            self.map
                .as_ref()
                .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
        }
    }

    let test_struct = TestStruct { map: None };
    assert_eq!(test_struct.get(1), 0); // Default value
}

#[test]
fn test_get_with_valid_key() {
    struct TestStruct {
        map: Option<Vec<TestValue>>,
    }

    struct TestValue {
        value: u32,
    }

    impl TestStruct {
        fn lookup(&self, key: u32) -> usize {
            // For this test, we assume key maps directly to valid index.
            key as usize
        }

        fn get(&self, key: u32) -> u32 {
            self.map
                .as_ref()
                .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
        }
    }

    let values = vec![TestValue { value: 10 }, TestValue { value: 20 }];
    let test_struct = TestStruct { map: Some(values) };
    assert_eq!(test_struct.get(0), 10);
    assert_eq!(test_struct.get(1), 20);
}

#[test]
#[should_panic]
fn test_get_with_out_of_bound_key() {
    struct TestStruct {
        map: Option<Vec<TestValue>>,
    }

    struct TestValue {
        value: u32,
    }

    impl TestStruct {
        fn lookup(&self, key: u32) -> usize {
            // Assuming key is directly the index
            key as usize
        }

        fn get(&self, key: u32) -> u32 {
            self.map
                .as_ref()
                .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
        }
    }

    let values = vec![TestValue { value: 10 }];
    let test_struct = TestStruct { map: Some(values) };
    test_struct.get(1);  // This should panic as there is no second element
}

