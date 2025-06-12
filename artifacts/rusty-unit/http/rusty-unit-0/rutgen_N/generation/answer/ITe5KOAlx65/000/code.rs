// Answer 0

#[test]
fn test_capacity_initial_state() {
    struct HeaderMap {
        indices: Vec<usize>,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap { indices: Vec::new() }
        }
        
        fn capacity(&self) -> usize {
            usable_capacity(self.indices.len())
        }
    }

    fn usable_capacity(len: usize) -> usize {
        match len {
            0 => 0,
            _ => len * 2 // Example capacity approximation
        }
    }

    let map = HeaderMap::new();
    assert_eq!(0, map.capacity());
}

#[test]
fn test_capacity_after_insertion() {
    struct HeaderMap {
        indices: Vec<usize>,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap { indices: Vec::new() }
        }

        fn insert(&mut self, _: &str, _: &str) {
            self.indices.push(self.indices.len());
        }
        
        fn capacity(&self) -> usize {
            usable_capacity(self.indices.len())
        }
    }

    fn usable_capacity(len: usize) -> usize {
        match len {
            0 => 0,
            _ => len * 2 // Example capacity approximation
        }
    }

    let mut map = HeaderMap::new();
    assert_eq!(0, map.capacity());

    map.insert("HOST", "hello.world");
    assert_eq!(2, map.capacity());
}

