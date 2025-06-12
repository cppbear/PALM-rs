// Answer 0

#[test]
fn test_swap_indices_valid() {
    struct TestSet {
        map: indexmap::IndexMap<usize, usize>,
    }

    impl TestSet {
        fn new() -> Self {
            let mut map = indexmap::IndexMap::new();
            map.insert(0, 10);
            map.insert(1, 20);
            map.insert(2, 30);
            TestSet { map }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            self.map.swap_indices(a, b);
        }
    }

    let mut set = TestSet::new();
    set.swap_indices(0, 1);
    assert_eq!(set.map[&0], 20);
    assert_eq!(set.map[&1], 10);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_indices_out_of_bounds() {
    struct TestSet {
        map: indexmap::IndexMap<usize, usize>,
    }

    impl TestSet {
        fn new() -> Self {
            let mut map = indexmap::IndexMap::new();
            map.insert(0, 10);
            map.insert(1, 20);
            TestSet { map }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            self.map.swap_indices(a, b);
        }
    }

    let mut set = TestSet::new();
    set.swap_indices(0, 2);
}

