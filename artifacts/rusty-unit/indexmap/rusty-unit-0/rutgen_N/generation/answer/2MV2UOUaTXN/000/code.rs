// Answer 0

#[test]
fn test_index_valid() {
    struct TestMap {
        iter: Vec<(String, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                iter: vec![
                    (String::from("key1"), 1),
                    (String::from("key2"), 2),
                    (String::from("key3"), 3),
                ],
            }
        }

        fn index(&self, index: usize) -> &String {
            &self.iter[index].0
        }
    }

    let map = TestMap::new();
    assert_eq!(map.index(0), "key1");
    assert_eq!(map.index(1), "key2");
    assert_eq!(map.index(2), "key3");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_out_of_bounds() {
    struct TestMap {
        iter: Vec<(String, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                iter: vec![
                    (String::from("key1"), 1),
                ],
            }
        }

        fn index(&self, index: usize) -> &String {
            &self.iter[index].0
        }
    }

    let map = TestMap::new();
    let _ = map.index(1); // This should panic
}

