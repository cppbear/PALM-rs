// Answer 0

#[test]
fn test_swap_indices_valid() {
    struct TestMap {
        data: IndexMap<u32, u32, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = IndexMap::new();
            data.insert(1, 10);
            data.insert(2, 20);
            data.insert(3, 30);
            TestMap { data }
        }
        
        fn swap(&mut self, a: usize, b: usize) {
            self.data.swap_indices(a, b);
        }

        fn get(&self, index: usize) -> Option<(&u32, &u32)> {
            self.data.get_index(index)
        }
    }

    let mut map = TestMap::new();
    assert_eq!(map.get(0), Some((&1, &10)));
    assert_eq!(map.get(1), Some((&2, &20)));

    map.swap(0, 1);
    
    assert_eq!(map.get(0), Some((&2, &20)));
    assert_eq!(map.get(1), Some((&1, &10)));
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds() {
    struct TestMap {
        data: IndexMap<u32, u32, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = IndexMap::new();
            data.insert(1, 10);
            data.insert(2, 20);
            TestMap { data }
        }
        
        fn swap(&mut self, a: usize, b: usize) {
            self.data.swap_indices(a, b);
        }
    }

    let mut map = TestMap::new();
    map.swap(0, 2); // This should panic because index 2 is out of bounds
}

#[test]
fn test_swap_indices_same_index() {
    struct TestMap {
        data: IndexMap<u32, u32, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = IndexMap::new();
            data.insert(1, 10);
            data.insert(2, 20);
            TestMap { data }
        }
        
        fn swap(&mut self, a: usize, b: usize) {
            self.data.swap_indices(a, b);
        }

        fn get(&self, index: usize) -> Option<(&u32, &u32)> {
            self.data.get_index(index)
        }
    }

    let mut map = TestMap::new();
    assert_eq!(map.get(0), Some((&1, &10)));
    assert_eq!(map.get(1), Some((&2, &20)));

    map.swap(0, 0); // Swapping the same index
    
    assert_eq!(map.get(0), Some((&1, &10)));
    assert_eq!(map.get(1), Some((&2, &20)));
}

