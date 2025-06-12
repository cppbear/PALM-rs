// Answer 0

#[test]
fn test_reserve_exact_no_additional_capacity() {
    struct Map {
        indices: std::collections::HashMap<usize, usize>,
        entries: Vec<(usize, usize)>,
    }

    impl Map {
        pub(crate) fn reserve_exact(&mut self, additional: usize) {
            self.indices.reserve(additional);
            self.entries.reserve_exact(additional);
        }
    }

    let mut map = Map {
        indices: std::collections::HashMap::new(),
        entries: Vec::new(),
    };

    let initial_capacity = map.entries.capacity();
    map.reserve_exact(0);
    assert_eq!(initial_capacity, map.entries.capacity());
}

#[test]
fn test_reserve_exact_with_additional_capacity() {
    struct Map {
        indices: std::collections::HashMap<usize, usize>,
        entries: Vec<(usize, usize)>,
    }

    impl Map {
        pub(crate) fn reserve_exact(&mut self, additional: usize) {
            self.indices.reserve(additional);
            self.entries.reserve_exact(additional);
        }
    }

    let mut map = Map {
        indices: std::collections::HashMap::new(),
        entries: Vec::new(),
    };

    map.reserve_exact(5);
    assert!(map.entries.capacity() >= 5);
}

#[test]
fn test_reserve_exact_edge_case_large_additional_capacity() {
    struct Map {
        indices: std::collections::HashMap<usize, usize>,
        entries: Vec<(usize, usize)>,
    }

    impl Map {
        pub(crate) fn reserve_exact(&mut self, additional: usize) {
            self.indices.reserve(additional);
            self.entries.reserve_exact(additional);
        }
    }

    let mut map = Map {
        indices: std::collections::HashMap::new(),
        entries: Vec::new(),
    };

    map.reserve_exact(usize::MAX);
    assert!(map.entries.capacity() >= usize::MAX);
}

