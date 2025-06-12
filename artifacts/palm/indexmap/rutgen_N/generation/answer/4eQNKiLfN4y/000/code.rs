// Answer 0

#[test]
fn test_capacity() {
    struct TestMap {
        core: Vec<i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: Vec::new(),
            }
        }

        fn capacity(&self) -> usize {
            self.core.capacity()
        }

        fn push(&mut self, value: i32) {
            self.core.push(value);
        }
    }

    let map = TestMap::new();
    assert_eq!(map.capacity(), 0);

    let mut map_with_elements = TestMap::new();
    map_with_elements.push(1);
    assert!(map_with_elements.capacity() >= 1);

    for i in 2..=10 {
        map_with_elements.push(i);
    }
    assert!(map_with_elements.capacity() >= 10);
}

