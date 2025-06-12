// Answer 0

#[test]
fn test_update_index_success() {
    struct Indices {
        indices: Vec<usize>,
    }
    
    impl Indices {
        fn new() -> Self {
            Indices { indices: vec![] }
        }
        
        fn find_mut<F>(&mut self, hash: usize, f: F) -> Option<&mut usize>
        where
            F: Fn(&usize) -> bool,
        {
            self.indices.iter_mut().find(f)
        }
        
        fn insert(&mut self, value: usize) {
            self.indices.push(value);
        }
    }
    
    struct HashValue {
        value: usize,
    }
    
    impl HashValue {
        fn new(value: usize) -> Self {
            HashValue { value }
        }
        
        fn get(&self) -> usize {
            self.value
        }
    }

    let mut table = Indices::new();
    table.insert(1);
    table.insert(2);
    table.insert(3);
    
    let hash = HashValue::new(0);
    update_index(&mut table, hash, 1, 4);

    assert_eq!(table.indices, vec![4, 2, 3]);
}

#[test]
#[should_panic(expected = "index not found")]
fn test_update_index_not_found() {
    struct Indices {
        indices: Vec<usize>,
    }
    
    impl Indices {
        fn new() -> Self {
            Indices { indices: vec![] }
        }
        
        fn find_mut<F>(&mut self, hash: usize, f: F) -> Option<&mut usize>
        where
            F: Fn(&usize) -> bool,
        {
            self.indices.iter_mut().find(f)
        }
        
        fn insert(&mut self, value: usize) {
            self.indices.push(value);
        }
    }
    
    struct HashValue {
        value: usize,
    }
    
    impl HashValue {
        fn new(value: usize) -> Self {
            HashValue { value }
        }
        
        fn get(&self) -> usize {
            self.value
        }
    }

    let mut table = Indices::new();
    table.insert(2);
    table.insert(3);
    
    let hash = HashValue::new(0);
    update_index(&mut table, hash, 1, 4);
}

