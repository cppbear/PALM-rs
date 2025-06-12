// Answer 0

#[test]
fn test_index_within_bounds() {
    struct TestIndexSet {
        data: Vec<usize>,
    }

    impl TestIndexSet {
        fn get_index(&self, index: usize) -> Option<&usize> {
            self.data.get(index)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn index(&self, index: usize) -> &usize {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(), index
                );
            })
        }
    }

    let set = TestIndexSet { data: vec![1, 2, 3, 4, 5] };
    let _value_1 = set.index(0);
    let _value_2 = set.index(2);
    let _value_3 = set.index(4);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 5 but the index is 5")]
fn test_index_out_of_bounds() {
    struct TestIndexSet {
        data: Vec<usize>,
    }

    impl TestIndexSet {
        fn get_index(&self, index: usize) -> Option<&usize> {
            self.data.get(index)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn index(&self, index: usize) -> &usize {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(), index
                );
            })
        }
    }

    let set = TestIndexSet { data: vec![1, 2, 3, 4, 5] };
    let _value = set.index(5);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn test_index_empty_set() {
    struct TestIndexSet {
        data: Vec<usize>,
    }

    impl TestIndexSet {
        fn get_index(&self, index: usize) -> Option<&usize> {
            self.data.get(index)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn index(&self, index: usize) -> &usize {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(), index
                );
            })
        }
    }

    let set = TestIndexSet { data: vec![] };
    let _value = set.index(0);
}

#[test]
fn test_index_edge_case_large_index() {
    struct TestIndexSet {
        data: Vec<usize>,
    }

    impl TestIndexSet {
        fn get_index(&self, index: usize) -> Option<&usize> {
            self.data.get(index)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn index(&self, index: usize) -> &usize {
            self.get_index(index).unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(), index
                );
            })
        }
    }

    let set = TestIndexSet { data: vec![0, 1, 2, 3] };
    let _value = set.index(3);
}

