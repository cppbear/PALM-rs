// Answer 0

#[test]
fn test_move_index_within_bounds() {
    struct IndexMap {
        core: Vec<(usize, usize)>,
    }

    impl IndexMap {
        fn new() -> Self {
            Self { core: Vec::new() }
        }

        fn insert(&mut self, key: usize, value: usize) {
            self.core.push((key, value));
        }

        fn move_index(&mut self, from: usize, to: usize) {
            if from >= self.core.len() || to >= self.core.len() {
                panic!("index out of bounds");
            }
            let item = self.core.remove(from);
            self.core.insert(to, item);
        }

        fn get(&self) -> &Vec<(usize, usize)> {
            &self.core
        }
    }

    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    map.move_index(0, 2);
    
    let result = map.get();
    assert_eq!(result, &[(2, 20), (3, 30), (1, 10)]);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_from() {
    struct IndexMap {
        core: Vec<(usize, usize)>,
    }

    impl IndexMap {
        fn new() -> Self {
            Self { core: Vec::new() }
        }

        fn insert(&mut self, key: usize, value: usize) {
            self.core.push((key, value));
        }

        fn move_index(&mut self, from: usize, to: usize) {
            if from >= self.core.len() || to >= self.core.len() {
                panic!("index out of bounds");
            }
            let item = self.core.remove(from);
            self.core.insert(to, item);
        }
    }

    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    map.move_index(2, 1); // from index 2 (out of bounds)
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_to() {
    struct IndexMap {
        core: Vec<(usize, usize)>,
    }

    impl IndexMap {
        fn new() -> Self {
            Self { core: Vec::new() }
        }

        fn insert(&mut self, key: usize, value: usize) {
            self.core.push((key, value));
        }

        fn move_index(&mut self, from: usize, to: usize) {
            if from >= self.core.len() || to >= self.core.len() {
                panic!("index out of bounds");
            }
            let item = self.core.remove(from);
            self.core.insert(to, item);
        }
    }

    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    
    map.move_index(0, 2); // to index 2 (out of bounds)
}

