// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, 10), (2, 20), (3, 30)],
                indices: vec![0, 1, 2],
            }
        }

        fn entries(&self) -> &Vec<(i32, i32)> {
            &self.entries
        }

        fn shift_remove_finish(&mut self, index: usize) -> (i32, i32) {
            self.entries.remove(index)
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, i32)> {
            match self.entries.get(index) {
                Some(entry) => {
                    let hash = 0; // Assume there's a hash function returning 0 for simplicity
                    erase_index(&mut self.indices, hash, index);
                    Some(self.shift_remove_finish(index))
                }
                None => None,
            }
        }
    }

    fn erase_index(indices: &mut Vec<usize>, hash: usize, index: usize) {
        indices.remove(index);
    }

    let mut map = TestMap::new();
    let removed = map.shift_remove_index(1);
    assert_eq!(removed, Some((2, 20)));
    assert_eq!(map.entries.len(), 2);
    assert!(map.entries.iter().all(|&(k, _)| k != 2));
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
        indices: Vec<usize>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                entries: vec![(1, 10), (2, 20)],
                indices: vec![0, 1],
            }
        }

        fn shift_remove_index(&mut self, index: usize) -> Option<(i32, i32)> {
            match self.entries.get(index) {
                Some(entry) => {
                    let hash = 0; // Assume there’s a hash function returning 0 for simplicity
                    erase_index(&mut self.indices, hash, index);
                    Some(self.entries.remove(index))
                }
                None => None,
            }
        }
    }

    fn erase_index(indices: &mut Vec<usize>, hash: usize, index: usize) {
        indices.remove(index);
    }

    let mut map = TestMap::new();
    let removed = map.shift_remove_index(5);
    assert_eq!(removed, None);
}

