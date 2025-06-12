// Answer 0

#[test]
fn test_shrink_to() {
    struct Map {
        indices: Vec<usize>,
        entries: Vec<Option<usize>>,
    }

    impl Map {
        fn new() -> Self {
            Map {
                indices: vec![],
                entries: vec![],
            }
        }
        
        fn shrink_to(&mut self, min_capacity: usize) {
            self.indices = self.indices.iter().cloned().take(min_capacity).collect();
            self.entries = self.entries.iter().cloned().take(min_capacity).collect();
        }
    }

    let mut map = Map::new();
    map.indices.extend(vec![0, 1, 2, 3, 4]);
    map.entries.extend(vec![Some(10), Some(20), Some(30), Some(40), Some(50)]);

    map.shrink_to(3);

    assert_eq!(map.indices.len(), 3);
    assert_eq!(map.entries.len(), 3);
    assert_eq!(map.indices, vec![0, 1, 2]);
    assert_eq!(map.entries, vec![Some(10), Some(20), Some(30)]);
}

#[test]
fn test_shrink_to_exceeding_capacity() {
    struct Map {
        indices: Vec<usize>,
        entries: Vec<Option<usize>>,
    }

    impl Map {
        fn new() -> Self {
            Map {
                indices: vec![],
                entries: vec![],
            }
        }
        
        fn shrink_to(&mut self, min_capacity: usize) {
            self.indices = self.indices.iter().cloned().take(min_capacity).collect();
            self.entries = self.entries.iter().cloned().take(min_capacity).collect();
        }
    }

    let mut map = Map::new();
    map.indices.extend(vec![0, 1, 2]);
    map.entries.extend(vec![Some(10), Some(20), Some(30)]);

    map.shrink_to(5);

    assert_eq!(map.indices.len(), 3);
    assert_eq!(map.entries.len(), 3);
    assert_eq!(map.indices, vec![0, 1, 2]);
    assert_eq!(map.entries, vec![Some(10), Some(20), Some(30)]);
}

#[test]
fn test_shrink_to_zero() {
    struct Map {
        indices: Vec<usize>,
        entries: Vec<Option<usize>>,
    }

    impl Map {
        fn new() -> Self {
            Map {
                indices: vec![],
                entries: vec![],
            }
        }
        
        fn shrink_to(&mut self, min_capacity: usize) {
            self.indices = self.indices.iter().cloned().take(min_capacity).collect();
            self.entries = self.entries.iter().cloned().take(min_capacity).collect();
        }
    }

    let mut map = Map::new();
    map.indices.extend(vec![0, 1, 2]);
    map.entries.extend(vec![Some(10), Some(20), Some(30)]);

    map.shrink_to(0);

    assert_eq!(map.indices.len(), 0);
    assert_eq!(map.entries.len(), 0);
    assert_eq!(map.indices, vec![]);
    assert_eq!(map.entries, vec![]);
}

