// Answer 0

#[test]
fn test_move_index_valid() {
    struct TestMap {
        map: IndexMap<u32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, "one".into());
            map.insert(2, "two".into());
            map.insert(3, "three".into());
            TestMap { map }
        }

        fn get_keys(&self) -> Vec<u32> {
            self.map.iter().map(|(k, _)| *k).collect()
        }
    }

    let mut test_map = TestMap::new();
    test_map.map.move_index(0, 2);
    assert_eq!(test_map.get_keys(), vec![2, 3, 1]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_from() {
    let mut map = IndexMap::new();
    map.insert(1, "one".into());
    map.insert(2, "two".into());
    map.insert(3, "three".into());

    map.move_index(3, 0); // Invalid `from` index
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_to() {
    let mut map = IndexMap::new();
    map.insert(1, "one".into());
    map.insert(2, "two".into());
    map.insert(3, "three".into());

    map.move_index(0, 3); // Invalid `to` index
}

#[test]
fn test_move_index_same_position() {
    let mut test_map = IndexMap::new();
    test_map.insert(1, "one".into());
    test_map.insert(2, "two".into());
    test_map.insert(3, "three".into());

    test_map.move_index(1, 1); // Move to the same index
    assert_eq!(test_map.iter().map(|(k, _)| *k).collect::<Vec<_>>(), vec![1, 2, 3]);
}

