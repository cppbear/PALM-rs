// Answer 0

#[test]
fn test_get_index_mut2_valid_index() {
    struct TestStruct {
        map: indexmap::IndexMap<usize, String>,
    }

    impl TestStruct {
        fn new() -> Self {
            let mut map = indexmap::IndexMap::new();
            map.insert(0, "zero".to_string());
            map.insert(1, "one".to_string());
            map.insert(2, "two".to_string());
            TestStruct { map }
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut String> {
            match self.map.get_index_mut2(index) {
                Some((value, ())) => Some(value),
                None => None,
            }
        }
    }

    let mut test_struct = TestStruct::new();
    let value = test_struct.get_index_mut2(1);
    assert!(value.is_some());
    assert_eq!(value.unwrap(), &mut "one".to_string());
}

#[test]
fn test_get_index_mut2_invalid_index() {
    struct TestStruct {
        map: indexmap::IndexMap<usize, String>,
    }

    impl TestStruct {
        fn new() -> Self {
            let mut map = indexmap::IndexMap::new();
            map.insert(0, "zero".to_string());
            map.insert(1, "one".to_string());
            map.insert(2, "two".to_string());
            TestStruct { map }
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut String> {
            match self.map.get_index_mut2(index) {
                Some((value, ())) => Some(value),
                None => None,
            }
        }
    }

    let mut test_struct = TestStruct::new();
    let value = test_struct.get_index_mut2(3);
    assert!(value.is_none());
}

