// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    struct TestMap {
        entries: Vec<(usize, usize)>, // Using usize for both key and value for simplicity
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: vec![(1, 10), (2, 20), (3, 30)],
                indices: vec![0, 1, 2],
            }
        }
        
        fn swap_remove_finish(&mut self, index: usize) -> (usize, usize) {
            let last = self.entries.len() - 1;
            self.entries.swap_remove(index)
        }
    }

    fn erase_index(indices: &mut Vec<usize>, hash: usize, index: usize) {
        indices.swap_remove(index);
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_index(1);
    assert_eq!(result, Some((2, 20)));
    assert_eq!(map.entries.len(), 2);
}

#[test]
fn test_swap_remove_index_invalid() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                entries: vec![(1, 10), (2, 20)],
                indices: vec![0, 1],
            }
        }
        
        fn swap_remove_finish(&mut self, index: usize) -> (usize, usize) {
            self.entries.swap_remove(index)
        }
    }

    fn erase_index(indices: &mut Vec<usize>, hash: usize, index: usize) {
        indices.swap_remove(index);
    }

    let mut map = TestMap::new();
    let result = map.swap_remove_index(5);
    assert_eq!(result, None);
}

