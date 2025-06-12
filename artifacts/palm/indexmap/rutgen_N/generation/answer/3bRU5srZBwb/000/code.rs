// Answer 0

#[test]
fn test_get_index_mut2_valid_index() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }
    
    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut Vec<(usize, usize)> {
            &mut self.entries
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut usize, &mut usize)> {
            self.as_entries_mut().get_mut(index).map(|entry| (&mut entry.0, &mut entry.1))
        }
    }

    let mut map = TestMap { entries: vec![(1, 10), (2, 20), (3, 30)] };
    if let Some((key, value)) = map.get_index_mut2(1) {
        *key = 4;
        *value = 40;
    }
    assert_eq!(map.entries[1], (4, 40));
}

#[test]
fn test_get_index_mut2_invalid_index() {
    struct TestMap {
        entries: Vec<(usize, usize)>,
    }
    
    impl TestMap {
        fn as_entries_mut(&mut self) -> &mut Vec<(usize, usize)> {
            &mut self.entries
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut usize, &mut usize)> {
            self.as_entries_mut().get_mut(index).map(|entry| (&mut entry.0, &mut entry.1))
        }
    }

    let mut map = TestMap { entries: vec![(1, 10), (2, 20), (3, 30)] };
    let result = map.get_index_mut2(5);
    assert!(result.is_none());
}

