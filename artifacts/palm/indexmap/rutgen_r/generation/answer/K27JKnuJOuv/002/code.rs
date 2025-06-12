// Answer 0

#[test]
fn test_get_index_mut2_none() {
    struct TestStruct {
        map: indexmap::IndexMap<usize, usize>,
    }

    impl TestStruct {
        fn new() -> Self {
            Self {
                map: indexmap::IndexMap::new(),
            }
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut usize> {
            match self.map.get_index_mut2(index) {
                Some((value, ())) => Some(value),
                None => None,
            }
        }
    }

    let mut test_struct = TestStruct::new();
    let result = test_struct.get_index_mut2(0);
    assert_eq!(result, None);
}

